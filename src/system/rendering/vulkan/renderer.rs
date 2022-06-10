use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::default::{Default};
use winit::window::Window;
use crate::system::mesh::Mesh;

use ash::{vk, Entry, LoadingError, Instance, Device};
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{AccessFlags, Bool32, CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel, CommandBufferResetFlags, CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerEXT, DependencyFlags, DeviceCreateInfo, DeviceMemory, DeviceQueueCreateInfo, Extent2D, Fence, FenceCreateFlags, FenceCreateInfo, Format, Image, ImageAspectFlags, ImageCreateInfo, ImageCreateInfoBuilder, ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, ImageTiling, ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryAllocateInfo, MemoryPropertyFlags, MemoryRequirements, MemoryType, PhysicalDevice, PhysicalDeviceMemoryProperties, PipelineStageFlags, PresentModeKHR, Queue, QueueFlags, SampleCountFlags, Semaphore, SemaphoreCreateInfo, SharingMode, SubmitInfo, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR, SurfaceTransformFlagsKHR, SwapchainCreateInfoKHR, SwapchainKHR};
use ash_window::create_surface;
use num_format::Locale::se;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode::N;
use crate::system::rendering::vulkan::device::VkDevice;
use crate::system::rendering::vulkan::swapchain::VkSwapchain;

unsafe extern "system" fn vulkan_debug_callback(message_severity: DebugUtilsMessageSeverityFlagsEXT,
                                                message_type: DebugUtilsMessageTypeFlagsEXT,
                                                p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
                                                _user_data: *mut std::os::raw::c_void) -> Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else { CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy() };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else { CStr::from_ptr(callback_data.p_message).to_string_lossy() };

    println!("{:?}:\n{:?} [{} ({})] : {}\n",
             message_severity,
             message_type,
             message_id_name,
             &message_id_number.to_string(),
             message, );

    vk::FALSE
}

pub struct VkRenderer {
    instance: Instance,
    debug_call_back: DebugUtilsMessengerEXT,
    surface: SurfaceKHR,
    surface_loader: Surface,
    vk_device: VkDevice,
    vk_swapchain: VkSwapchain,
    command_pool: CommandPool,
    present_image_views: Vec<ImageView>,
    depth_image: Image,
    depth_image_memory: DeviceMemory,
    depth_image_view: ImageView,
    draw_commands_reuse_fence: Fence,
    setup_commands_reuse_fence: Fence,
    present_complete_semaphore: Semaphore,
    rendering_complete_semaphore: Semaphore,
}

impl<'a> VkRenderer {
    pub fn init(window: &Window) -> Self {
        let entry = VkRenderer::create_vk_entry().expect("Creation of entry failed!");
        let instance = VkRenderer::create_vk_instance(&entry, window);
        let debug_call_back = VkRenderer::create_vk_debug_callback(&entry, &instance);
        let surface = VkRenderer::create_vk_surface(&entry, &instance, window);
        let surface_loader = Surface::new(&entry, &instance);

        let vk_device = VkDevice::new(&instance, &surface_loader, &surface);
        let vk_swapchain = VkSwapchain::new(&instance, &vk_device.physical_device, &vk_device.logical_device, &surface, &surface_loader, &window.inner_size());

        let command_pool = VkRenderer::create_vk_command_buffer_pool(&vk_device.queue_family_index, &vk_device.logical_device);
        let command_buffers = VkRenderer::create_vk_command_buffers(&command_pool, &vk_device.logical_device);

        let setup_command_buffer = command_buffers[0];
        let draw_command_buffer = command_buffers[1];

        let present_image_views = VkRenderer::create_vk_present_images(&vk_swapchain, &vk_device.logical_device);

        let surface_res = vk_swapchain.surface_resolution;
        let depth_image = VkRenderer::create_vk_create_depth_image(&vk_device.logical_device, surface_res);
        let depth_image_memory = VkRenderer::create_vk_depth_image_memory(&instance, &vk_device, &depth_image);

        let draw_commands_reuse_fence = VkRenderer::create_vk_fences(&vk_device.logical_device);
        let setup_commands_reuse_fence = VkRenderer::create_vk_fences(&vk_device.logical_device);

        VkRenderer::record_submit_buffer(&vk_device.logical_device, setup_command_buffer,
                                         setup_commands_reuse_fence,
                                         vk_device.present_queue, &[],
                                         &[], &[], |device, setup_command_buffer| {
                let layout_transition_barriers = ImageMemoryBarrier::builder()
                    .image(depth_image)
                    .dst_access_mask(AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ | AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                    .new_layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                    .old_layout(ImageLayout::UNDEFINED)
                    .subresource_range(ImageSubresourceRange::builder()
                                           .aspect_mask(ImageAspectFlags::DEPTH)
                                           .layer_count(1)
                                           .level_count(1)
                                           .build(),
                    );
                unsafe {
                    device.cmd_pipeline_barrier(setup_command_buffer,
                                                PipelineStageFlags::BOTTOM_OF_PIPE,
                                                PipelineStageFlags::LATE_FRAGMENT_TESTS,
                                                DependencyFlags::empty(),
                                                &[], &[],
                                                &[layout_transition_barriers.build()])
                }
            }, );

        let depth_image_view = VkRenderer::create_vk_depth_image_view(&vk_device.logical_device, &depth_image);

        let present_complete_semaphore = VkRenderer::create_vk_semaphore(&vk_device.logical_device);
        let rendering_complete_semaphore = VkRenderer::create_vk_semaphore(&vk_device.logical_device);

        VkRenderer {
            instance,
            debug_call_back,
            surface,
            surface_loader,
            vk_device,
            vk_swapchain,
            command_pool,
            present_image_views,
            depth_image,
            depth_image_memory,
            depth_image_view,
            draw_commands_reuse_fence,
            setup_commands_reuse_fence,
            present_complete_semaphore,
            rendering_complete_semaphore,
        }
    }

    fn create_vk_entry() -> Result<Entry, LoadingError> {
        let entry = unsafe { Entry::load()? };
        Ok(entry)
    }

    fn create_vk_instance(entry: &Entry, window: &Window) -> Instance {
        let mut instance: ash::Instance;
        unsafe {
            let app_name = CStr::from_bytes_with_nul_unchecked(b"RESA");
            let layer_names = [CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0")];
            let layer_names_raw: Vec<*const c_char> = layer_names.iter().map(|raw_name| raw_name.as_ptr()).collect();

            let mut extension_names = ash_window::enumerate_required_extensions(window).unwrap().to_vec();
            extension_names.push(DebugUtils::name().as_ptr());

            let appinfo = vk::ApplicationInfo::builder()
                .application_name(app_name)
                .application_version(0)
                .engine_name(app_name)
                .engine_version(0)
                .api_version(vk::make_api_version(0, 1, 0, 0));

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
                .enabled_layer_names(&layer_names_raw)
                .enabled_extension_names(&extension_names);

            instance = entry.create_instance(&create_info, None).expect("Instance creation error!");
        }
        instance
    }

    fn create_vk_debug_callback(entry: &Entry, instance: &Instance) -> DebugUtilsMessengerEXT {
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(DebugUtilsMessageSeverityFlagsEXT::ERROR | DebugUtilsMessageSeverityFlagsEXT::WARNING | DebugUtilsMessageSeverityFlagsEXT::INFO)
            .message_type(DebugUtilsMessageTypeFlagsEXT::GENERAL | DebugUtilsMessageTypeFlagsEXT::VALIDATION | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE)
            .pfn_user_callback(Some(vulkan_debug_callback));

        let debug_utils_loader = DebugUtils::new(entry, instance);
        let debug_call_back = unsafe { debug_utils_loader.create_debug_utils_messenger(&debug_info, None).unwrap() };
        debug_call_back
    }

    fn create_vk_surface(entry: &Entry, instance: &Instance, window: &Window) -> SurfaceKHR {
        let surface = unsafe { create_surface(entry, instance, window, None).expect("Surface creation failed!") };
        surface
    }

    fn create_vk_command_buffer_pool(queue_family_index: &u32, device: &Device) -> CommandPool {
        let pool_create_info = CommandPoolCreateInfo::builder().
            flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(*queue_family_index);

        let pool = unsafe { device.create_command_pool(&pool_create_info, None).expect("Creating command pool failed!") };
        pool
    }

    fn create_vk_command_buffers(pool: &CommandPool, device: &Device) -> Vec<CommandBuffer> {
        let command_buffer_allocate_info = CommandBufferAllocateInfo::builder()
            .command_buffer_count(2)
            .command_pool(*pool)
            .level(CommandBufferLevel::PRIMARY);

        let command_buffers = unsafe { device.allocate_command_buffers(&command_buffer_allocate_info).expect("Command buffer allocation failed!") };
        command_buffers
    }

    fn create_vk_present_images(swapchain: &VkSwapchain, device: &Device) -> Vec<ImageView> {
        let mut present_image_views: Vec<ImageView>;
        unsafe {
            let present_images =
                swapchain.swapchain_loader.get_swapchain_images(swapchain.vk_swapchain)
                    .expect("Creation of present images failed!");

            present_image_views = present_images
                .iter()
                .map(|&image| {
                    let create_view_info = ImageViewCreateInfo::builder()
                        .view_type(ImageViewType::TYPE_2D)
                        .format(swapchain.surface_format.format)
                        .components(ComponentMapping {
                            r: ComponentSwizzle::R,
                            g: ComponentSwizzle::G,
                            b: ComponentSwizzle::B,
                            a: ComponentSwizzle::A,
                        })
                        .subresource_range(ImageSubresourceRange {
                            aspect_mask: ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image);
                    device.create_image_view(&create_view_info, None).expect("Creation of present image failed!")
                })
                .collect();
        }
        present_image_views
    }

    fn create_vk_create_depth_image(device: &Device, surface_res: Extent2D) -> Image {
        let depth_image: Image;

        unsafe {
            let depth_image_create_info = ImageCreateInfo::builder()
                .image_type(ImageType::TYPE_2D)
                .format(Format::D16_UNORM)
                .extent(surface_res.into())
                .mip_levels(1)
                .array_layers(1)
                .samples(SampleCountFlags::TYPE_1)
                .tiling(ImageTiling::OPTIMAL)
                .usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
                .sharing_mode(SharingMode::EXCLUSIVE);
            depth_image = device
                .create_image(&depth_image_create_info, None)
                .expect("Creation of depth image failed!");
        }
        depth_image
    }

    fn create_vk_depth_image_memory(instance: &Instance, vk_device: &VkDevice, depth_image: &Image) -> DeviceMemory {
        let depth_image_memory: DeviceMemory;

        unsafe {
            let device_memory_properties: PhysicalDeviceMemoryProperties = instance
                .get_physical_device_memory_properties(vk_device.physical_device);
            let depth_image_memory_req = vk_device.logical_device.get_image_memory_requirements(*depth_image);
            let depth_image_memory_index = VkRenderer::find_memory_index(
                &depth_image_memory_req, &device_memory_properties, MemoryPropertyFlags::DEVICE_LOCAL)
                .expect("Unable to find suitable memory index for depth image!");

            let depth_image_alloc_info = MemoryAllocateInfo::builder()
                .allocation_size(depth_image_memory_req.size)
                .memory_type_index(depth_image_memory_index);

            depth_image_memory = vk_device.logical_device.allocate_memory(&depth_image_alloc_info, None)
                .expect("Unable to allocate depth image memory!");

            vk_device.logical_device.bind_image_memory(*depth_image, depth_image_memory, 0)
                .expect("Unable to bind depth image memory!");
        }

        depth_image_memory
    }

    fn find_memory_index(memory_req: &MemoryRequirements, memory_prop: &PhysicalDeviceMemoryProperties, flags: MemoryPropertyFlags) -> Option<u32> {
        memory_prop.memory_types[..memory_prop.memory_type_count as _]
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & memory_req.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _memory_type)| index as _)
    }

    fn create_vk_fences(device: &Device) -> Fence {
        let fence_create_info = FenceCreateInfo::builder().flags(FenceCreateFlags::SIGNALED);
        let fence = unsafe { device.create_fence(&fence_create_info, None).expect("Create fence failed!") };
        fence
    }

    fn record_submit_buffer<F: FnOnce(&Device, CommandBuffer)>(device: &Device,
                                                               command_buffer: CommandBuffer,
                                                               command_buffer_reuse_fence: Fence,
                                                               submit_queue: Queue,
                                                               wait_mask: &[PipelineStageFlags],
                                                               wait_semaphores: &[Semaphore],
                                                               signal_semaphores: &[Semaphore],
                                                               f: F) {
        unsafe {
            device.wait_for_fences(&[command_buffer_reuse_fence], true, u64::MAX).expect("Wait for fence failed!");
            device.reset_fences(&[command_buffer_reuse_fence]).expect("Reset fences failed!");
            device.reset_command_buffer(command_buffer, CommandBufferResetFlags::RELEASE_RESOURCES).expect("Reset cmd buffer failed!");
            let command_buffer_begin_info = CommandBufferBeginInfo::builder().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            device.begin_command_buffer(command_buffer, &command_buffer_begin_info).expect("Begin cmd buffer!");
            f(device, command_buffer);
            device.end_command_buffer(command_buffer).expect("End cmd buffer!");
            let command_buffers = vec![command_buffer];

            let submit_info = SubmitInfo::builder()
                .wait_semaphores(wait_semaphores)
                .wait_dst_stage_mask(wait_mask)
                .command_buffers(&command_buffers)
                .signal_semaphores(signal_semaphores);

            device.queue_submit(submit_queue, &[submit_info.build()], command_buffer_reuse_fence).expect("queue submit failed!");
        }
    }

    fn create_vk_depth_image_view(device: &Device, depth_image: &Image) -> ImageView{
        let depth_image_view_info = ImageViewCreateInfo::builder()
            .subresource_range(ImageSubresourceRange::builder()
                .aspect_mask(ImageAspectFlags::DEPTH)
                .level_count(1)
                .layer_count(1)
                .build(),)
            .image(*depth_image)
            .format(Format::D16_UNORM)
            .view_type(ImageViewType::TYPE_2D);

        let depth_image_view = unsafe{device.create_image_view(&depth_image_view_info, None).expect("Depth image view creation failed!")};
        depth_image_view
    }

    fn create_vk_semaphore(device: &Device) -> Semaphore {
        let semaphore_create_info = SemaphoreCreateInfo::default();
        let semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None).expect("Semaphore creation failed!") };
        semaphore
    }
}


impl Drop for VkRenderer {
    fn drop(&mut self) {
        unsafe {
            self.vk_device.logical_device.device_wait_idle().unwrap();
            self.vk_device.logical_device.destroy_semaphore(self.present_complete_semaphore, None);
            self.vk_device.logical_device.destroy_semaphore(self.rendering_complete_semaphore, None);
            self.vk_device.logical_device.destroy_fence(self.draw_commands_reuse_fence, None);
            self.vk_device.logical_device.destroy_fence(self.setup_commands_reuse_fence, None);
            self.vk_device.logical_device.free_memory(self.depth_image_memory, None);
            self.vk_device.logical_device.destroy_image_view(self.depth_image_view, None);
            self.vk_device.logical_device.destroy_image(self.depth_image, None);
            for &image_view in self.present_image_views.iter() {
                self.vk_device.logical_device.destroy_image_view(image_view, None);
            }
            self.vk_device.logical_device.destroy_command_pool(self.command_pool, None);
            self.vk_swapchain.destroy();
            self.vk_device.destroy();
            self.surface_loader.destroy_surface(self.surface, None);
            // Destroy debug callback
            self.instance.destroy_instance(None);
        }
    }
}
