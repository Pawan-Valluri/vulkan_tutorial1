
use log::*;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

use vulkanalia::vk;
use vulkanalia::prelude::v1_4::*;

use crate::core::SuitabilityError;
use crate::app::data::AppData;
use crate::device::queues::QueueFamilyIndices;
use crate::swapchain::support::SwapchainSupport;
use crate::device::DEVICE_EXTENTIONS;

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
