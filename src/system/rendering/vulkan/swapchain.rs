use ash::{Instance, Device};
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{CompositeAlphaFlagsKHR, Extent2D, ImageUsageFlags, PhysicalDevice, PresentModeKHR, SharingMode, SurfaceFormatKHR, SurfaceKHR, SurfaceTransformFlagsKHR, SwapchainCreateInfoKHR, SwapchainKHR};
use winit::dpi::PhysicalSize;

pub struct VkSwapchain{
    pub vk_swapchain: SwapchainKHR,
    pub swapchain_loader: Swapchain,
    pub surface_format: SurfaceFormatKHR,
    pub surface_resolution: Extent2D,

}

impl VkSwapchain{
    pub fn new(instance: &Instance, pdevice: &PhysicalDevice, device: &Device, surface: &SurfaceKHR, surface_loader: &Surface, window_size: &PhysicalSize<u32>)-> Self{
        let surface_format = unsafe { surface_loader.get_physical_device_surface_formats(*pdevice, *surface).unwrap()[0] };

        let surface_capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(*pdevice, *surface).unwrap() };
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => Extent2D {
                width: window_size.width,
                height: window_size.height,
            },
            _ => surface_capabilities.current_extent,
        };


        let mut pre_transform: SurfaceTransformFlagsKHR;
        if surface_capabilities.supported_transforms.contains(SurfaceTransformFlagsKHR::IDENTITY)
        {
            pre_transform = SurfaceTransformFlagsKHR::IDENTITY;
        } else { pre_transform = surface_capabilities.current_transform };

        let present_modes = unsafe {
            surface_loader.get_physical_device_surface_present_modes(*pdevice, *surface)
                .expect("Present modes loading failed!")
        };

        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == PresentModeKHR::MAILBOX)
            .unwrap_or(PresentModeKHR::FIFO);

        let swapchain_loader = Swapchain::new(instance, device);
        let swapchain_create_info = SwapchainCreateInfoKHR::builder()
            .surface(*surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let vk_swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None).expect("Swapchain creation failed!") };

        VkSwapchain{
            vk_swapchain,
            swapchain_loader,
            surface_format,
            surface_resolution,
        }
    }

    pub fn destroy(&self){
        unsafe{
            self.swapchain_loader.destroy_swapchain(self.vk_swapchain, None);
        }
    }
}