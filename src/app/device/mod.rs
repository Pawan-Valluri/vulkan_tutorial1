use log::*;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

use vulkanalia::vk;
use vulkanalia::prelude::v1_4::*;

use crate::app::appdata::AppData;
use crate::app::instance::{
    VALIDATION_ENABLED,
    VALIDATION_LAYER,
    PORTABILITY_MACOS_VERSION
};
use crate::app::core::SuitabilityError;
use crate::app::queue_families::QueueFamilyIndices;
use crate::app::swapchain::support::SwapchainSupport;



const DEVICE_EXTENTIONS: &[vk::ExtensionName] = &[
    vk::KHR_SWAPCHAIN_EXTENSION.name
];


unsafe fn check_physical_device(
    instance: &Instance,
    data: &AppData,
    physical_device: vk::PhysicalDevice,
) -> Result<()> {
    let properties = instance.get_physical_device_properties(physical_device);
    if properties.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
        return Err(anyhow!(SuitabilityError("Only discrete GPU are supported!")));
    }

    let features = instance.get_physical_device_features(physical_device);
    if features.geometry_shader != vk::TRUE {
        return Err(anyhow!(SuitabilityError("Missing Geometry Shader Support")));
    }

    QueueFamilyIndices::get(instance, data, physical_device)?;

    check_physical_device_extentions(instance, physical_device)?;

    let support = SwapchainSupport::get(instance, data, physical_device)?;
    if support.formats.is_empty() || support.present_modes.is_empty() {
        return Err(anyhow!(SuitabilityError("Insufficient swapchain support.")))
    }

    Ok(())
}

unsafe fn check_physical_device_extentions(instance: &Instance, physical_device: vk::PhysicalDevice) -> Result<()>{
    let extensions = instance
        .enumerate_device_extension_properties(physical_device, None)?
        .iter()
        .map(|e| e.extension_name)
        .collect::<HashSet<_>>();

    if DEVICE_EXTENTIONS.iter().all(|e| extensions.contains(e)) {
        Ok(())
    } else {
        Err(anyhow!(SuitabilityError("Missing required device extensions.")))
    }
}


pub unsafe fn pick_physical_device(instance: &Instance, data: &mut AppData) -> Result<()> {
    for physical_device in instance.enumerate_physical_devices()? {
        let properties = instance.get_physical_device_properties(physical_device);
        info!("inferring Physical device (`{}`)", properties.device_name);

        if let Err(error) = check_physical_device(instance, data, physical_device) {
            warn!("Skipping Physical device (`{}`) {}", properties.device_name, error)
        } else {
            info!("Selected Physical device (`{}`)", properties.device_name);
            data.physical_device = physical_device;
            return Ok(());
        }
    }

    Err(anyhow!("Failed to find suitable Physical device"))
}

pub unsafe fn create_logical_device(
    entry: &Entry,
    instance: &Instance,
    data: &mut AppData
) -> Result<Device> {
    let queue_priorities = &[1.0];

    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;
    let unique_indices = HashSet::from([
        indices.graphics, indices.present
    ]);

    let queue_infos = unique_indices.iter()
        .map(|index| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*index)
                .queue_priorities(queue_priorities)
        }).collect::<Vec<_>>();

    let layers = if VALIDATION_ENABLED {
        vec![VALIDATION_LAYER.as_ptr()]
    } else {
        vec![]
    };
    info!(
        "device layer names {:#?}",
        vk::ExtensionName::from_ptr(layers[0]).to_string_lossy()
    );

    let mut extensions = DEVICE_EXTENTIONS
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();
    // Required by Vulkan SDK on macOS since 1.3.216.
    if cfg!(target_os = "macos") && entry.version()? >= PORTABILITY_MACOS_VERSION {
        extensions.push(vk::KHR_PORTABILITY_SUBSET_EXTENSION.name.as_ptr());
    }

    let features = vk::PhysicalDeviceFeatures::builder();

    let device_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_infos)
        .enabled_layer_names(&layers)
        .enabled_extension_names(&extensions)
        .enabled_features(&features);

    let device = instance.create_device(data.physical_device, &device_info, None)?;

    data.graphics_queue = device.get_device_queue(indices.graphics, 0);
    data.present_queue = device.get_device_queue(indices.present, 0);

    Ok(device)
}
