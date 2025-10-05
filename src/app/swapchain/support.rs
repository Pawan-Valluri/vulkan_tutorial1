use anyhow::Result;
use winit::window::Window;

use vulkanalia::vk::KhrSurfaceExtension;
use vulkanalia::prelude::v1_4::*;

use crate::app::appdata::AppData;


#[derive(Clone, Debug)]
pub struct SwapchainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {
    pub unsafe fn get(
            instance: &Instance,
            data: &AppData,
            physical_device: vk::PhysicalDevice,
        ) -> Result<Self> {
        Ok( Self {
            capabilities: instance
                .get_physical_device_surface_capabilities_khr(
                    physical_device, data.surface
                )?,
            formats: instance
                .get_physical_device_surface_formats_khr(physical_device, data.surface)?,
            present_modes: instance
                .get_physical_device_surface_present_modes_khr(physical_device, data.surface)?,
        })
    }
}

pub fn get_swapchain_surface_format(
        formats: &[vk::SurfaceFormatKHR]
) -> vk::SurfaceFormatKHR {
    *formats
        .iter()
        // .cloned()
        .find(|f| {
            f.format == vk::Format::B8G8R8_SRGB &&
            f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        }).unwrap_or_else(|| &formats[0])
}

pub fn get_swapchain_present_mode (
    present_mode: &[vk::PresentModeKHR]
) -> vk::PresentModeKHR {
    present_mode
        .iter()
        .cloned()
        .find(|p| *p == vk::PresentModeKHR::MAILBOX)
        .unwrap_or(vk::PresentModeKHR::FIFO)
}

pub fn get_swapchain_extent(
    window: &Window,
    capabilities: vk::SurfaceCapabilitiesKHR
) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        capabilities.current_extent
    } else {
        vk::Extent2D::builder()
            .width(window.inner_size().width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            ))
            .height(window.inner_size().height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            ))
            .build()
    }
}

