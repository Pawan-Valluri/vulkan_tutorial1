use log::*;
use anyhow::Result;
use std::collections::HashSet;

use vulkanalia::vk;
use vulkanalia::prelude::v1_4::*;

use crate::app::data::AppData;
use crate::instance::{
    VALIDATION_ENABLED,
    VALIDATION_LAYER,
    PORTABILITY_MACOS_VERSION
};
use crate::device::queues::QueueFamilyIndices;
use crate::device::DEVICE_EXTENTIONS;


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
