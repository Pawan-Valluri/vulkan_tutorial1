use anyhow::Result;
use log::*;

use winit::window::Window;

use vulkanalia::vk::{self, KhrSwapchainExtension};
use vulkanalia::prelude::v1_4::*;

use crate::app::appdata::AppData;
use crate::app::queue_families::QueueFamilyIndices;

pub mod support;

pub unsafe fn create_swapchain (
    window: &Window,
    instance: &Instance,
    device: &Device,
    data: &mut AppData
) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
    let support = support::SwapchainSupport::get(instance, data, data.physical_device)?;

    let surface_format = support::get_swapchain_surface_format(&support.formats);
    let present_mode = support::get_swapchain_present_mode(&support.present_modes);
    let surface_extent = support::get_swapchain_extent(window, support.capabilities);

    let mut image_count = support.capabilities.min_image_count + 1;
    if support.capabilities.max_image_count != 0 
        && support.capabilities.max_image_count < image_count {
        image_count = support.capabilities.max_image_count;
    }

    let mut queue_family_indices = vec![];
    let image_sharing_mode = if indices.graphics != indices.present {
        queue_family_indices.push(indices.graphics);
        queue_family_indices.push(indices.present);
        vk::SharingMode::CONCURRENT
    } else {
        vk::SharingMode::EXCLUSIVE
    };

    info!("image sharing mode: {}", if image_sharing_mode == vk::SharingMode::EXCLUSIVE {"Exclusive"} else {"Concurrent"});
    info!("queue family indices: {:?}", queue_family_indices);

    let info = vk::SwapchainCreateInfoKHR::builder()
        .surface(data.surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(surface_extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(image_sharing_mode)
        .queue_family_indices(&queue_family_indices)
        .pre_transform(support.capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(vk::SwapchainKHR::null());

    data.swapchain = device.create_swapchain_khr(&info, None)?;
    data.swapchain_images = device.get_swapchain_images_khr(data.swapchain)?;

    data.swapchain_format = surface_format.format;
    data.swapchain_extent = surface_extent;

    Ok(())
}


pub unsafe fn create_swapchain_image_views(
    device: &Device,
    data:&mut AppData
) -> Result<()> {
    data.swapchain_image_views = data
        .swapchain_images
        .iter()
        .map(|i| {
            let components = vk::ComponentMapping::builder()
                .r(vk::ComponentSwizzle::IDENTITY)
                .g(vk::ComponentSwizzle::IDENTITY)
                .b(vk::ComponentSwizzle::IDENTITY)
                .a(vk::ComponentSwizzle::IDENTITY);

            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let info = vk::ImageViewCreateInfo::builder()
                .image(*i)
                .view_type(vk::ImageViewType::_2D)
                .format(data.swapchain_format)
                .components(components)
                .subresource_range(subresource_range);

            device.create_image_view(&info, None)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
