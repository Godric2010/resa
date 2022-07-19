use std::error::Error;
use ash::{Device, LoadingError};
use ash::extensions::khr::{Surface, Swapchain};
use ash::prelude::VkResult;
use ash::vk::{ColorSpaceKHR, CompositeAlphaFlagsKHR, Extent2D, Format, ImageUsageFlags, KhrPortabilitySubsetFn, PhysicalDevice, PhysicalDeviceFeatures, PresentModeKHR, Result, SharingMode, SurfaceFormatKHR, SurfaceKHR, SurfaceTransformFlagsKHR, SwapchainCreateInfoKHR, SwapchainCreateInfoKHRBuilder, SwapchainKHR};
use num_format::Locale::de;
use crate::system::error::{ResaError, ResaResult};
use crate::system::rendering::vulkan::global::VkInstance;

pub struct VkLogicalDevice {
    device: Device,
    swapchain_loader: Swapchain,
    swapchain: SwapchainKHR,

}

impl VkLogicalDevice {
    pub fn new(instance: &VkInstance, window_size: &Extent2D) -> ResaResult<VkLogicalDevice> {
        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let mut extension_names = Vec::new();
        extension_names.push(Swapchain::name().as_ptr());
        extension_names.push(KhrPortabilitySubsetFn::name().as_ptr());
        let device = instance.create_device(&features, &extension_names);
        let graphics_queue_index = instance.selected_physical_device.graphics_queue_family_index.clone();
        let compute_queue_index = instance.selected_physical_device.compute_queue_family_index.clone();
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_index, 0) };
        let compute_queue = unsafe { device.get_device_queue(compute_queue_index, 0) };

        let swapchain_loader = Swapchain::new(&instance.instance, &device);
        let swapchain: SwapchainKHR;
        match VkLogicalDevice::create_swapchain(&instance.selected_physical_device.physical_device, &swapchain_loader,
                                                &instance.surface, instance.surface_handle, window_size) {
            Ok(sw) => { swapchain = sw }
            Err(e) => {
                println!("Swapchain creation failed!");
                return ResaResult::Err();
            }
        }

        ResaResult::Ok(VkLogicalDevice {
            device,
            swapchain_loader,
            swapchain,

        })
    }

    pub fn destroy(&self) {
        unsafe {
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
        }
    }

    fn create_swapchain(physical_device: &PhysicalDevice, swapchain_loader: &Swapchain, surface: &Surface, surface_handle: SurfaceKHR, window_size: &Extent2D) -> VkResult<SwapchainKHR> {
        let physical = physical_device.clone();
        let swapchain: VkResult<SwapchainKHR>;
        unsafe {

            // Get present mode
            let present_modes =
                surface.get_physical_device_surface_present_modes(physical,
                                                                  surface_handle)
                    .expect("Present modes loading failed!");

            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == PresentModeKHR::MAILBOX)
                .unwrap_or(PresentModeKHR::FIFO);

            // load surface capabilities
            let surface_capabilities = surface
                .get_physical_device_surface_capabilities(physical, surface_handle)
                .unwrap();

            let mut number_of_images: u32 = surface_capabilities.min_image_count + 1;
            if surface_capabilities.max_image_count > 0 && number_of_images > surface_capabilities.max_image_count {
                number_of_images = surface_capabilities.max_image_count;
            }

            // Define size of images
            let mut size_of_images = window_size.clone();
            if size_of_images.width < surface_capabilities.min_image_extent.width {
                size_of_images.width = surface_capabilities.min_image_extent.width;
            } else if size_of_images.width > surface_capabilities.max_image_extent.width {
                size_of_images.width = surface_capabilities.max_image_extent.width;
            }

            if size_of_images.height < surface_capabilities.min_image_extent.height {
                size_of_images.height = surface_capabilities.min_image_extent.height;
            } else if size_of_images.height > surface_capabilities.max_image_extent.height {
                size_of_images.height = surface_capabilities.max_image_extent.height;
            }

            // define image usages
            let desired_usages = ImageUsageFlags::COLOR_ATTACHMENT;
            let image_usage: ImageUsageFlags = desired_usages & surface_capabilities.supported_usage_flags;
            let image_usages_available = desired_usages == image_usage;


            // get surface transform values
            let desired_transform = SurfaceTransformFlagsKHR::IDENTITY;
            let mut surface_transform: SurfaceTransformFlagsKHR;
            if surface_capabilities.supported_transforms.contains(desired_transform) {
                surface_transform = desired_transform;
            } else {
                surface_transform = surface_capabilities.current_transform
            }

            // define image format
            let supported_formats;
            match surface.get_physical_device_surface_formats(physical, surface_handle) {
                Ok(formats) => supported_formats = formats,
                Err(e) => {
                    println!("Failed to receive surface formats");
                    return Err(Result::INCOMPLETE);
                }
            }

            let desired_surface_format = SurfaceFormatKHR::builder()
                .format(Format::A8B8G8R8_SRGB_PACK32)
                .color_space(ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT)
                .build();

            let format_and_color_space = VkLogicalDevice::get_supported_image_format_and_color_space(&supported_formats, desired_surface_format);
            let image_format = format_and_color_space.0;
            let image_color_space = format_and_color_space.1;

            let swapchain_create_info = SwapchainCreateInfoKHR::builder()
                .surface(surface_handle)
                .min_image_count(number_of_images)
                .image_format(image_format)
                .image_color_space(image_color_space)
                .image_extent(size_of_images)
                .image_array_layers(1)
                .image_usage(image_usage)
                .image_sharing_mode(SharingMode::EXCLUSIVE)
                .queue_family_indices(&[0])
                .pre_transform(surface_transform)
                .present_mode(present_mode)
                .clipped(true)
                .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE);

            swapchain = swapchain_loader.create_swapchain(&swapchain_create_info, None);
        }
        swapchain
    }
    fn get_supported_image_format_and_color_space(surface_formats: &Vec<SurfaceFormatKHR>, desired_format: SurfaceFormatKHR) -> (Format, ColorSpaceKHR) {
        let format_and_color_space: (Format, ColorSpaceKHR);

        if surface_formats.len() == 1 && surface_formats[0].format == Format::UNDEFINED {
            format_and_color_space = (desired_format.format, desired_format.color_space);
            return format_and_color_space;
        }

        for surface_format in surface_formats {
            if surface_format.format == desired_format.format && surface_format.color_space == desired_format.color_space {
                format_and_color_space = (desired_format.format, desired_format.color_space);
                return format_and_color_space;
            }
        }

        for surface_format in surface_formats {
            if surface_format.format == desired_format.format {
                format_and_color_space = (desired_format.format, surface_format.color_space);
                return format_and_color_space;
            }
        }

        format_and_color_space = (surface_formats[0].format, surface_formats[0].color_space);
        format_and_color_space
    }
}