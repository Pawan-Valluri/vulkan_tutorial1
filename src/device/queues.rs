use anyhow::{anyhow, Result};

use vulkanalia::prelude::v1_4::*;
use vulkanalia::vk::KhrSurfaceExtension;

use crate::app::data::AppData;
use crate::core::SuitabilityError;

pub struct QueueFamilyIndices {
    pub graphics: u32,
    pub present: u32,
}

impl QueueFamilyIndices {
    pub unsafe fn get(
        instance: &Instance,
        data: &AppData,
        physical_device: vk::PhysicalDevice
    ) -> Result<Self> {
        let properties = instance.get_physical_device_queue_family_properties(physical_device);

        let graphics = properties
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);
        println!("queue graphics {}", graphics.unwrap());

        // let mut present = None;
        // for (index, properties) in properties.iter().enumerate() {
        //     if instance.get_physical_device_surface_support_khr(physical_device, index as u32, data.surface)? {
        //         present = Some(index as u32);
        //         break;
        //     }
        // }
        let present = properties
            .iter().enumerate()
            .position(|(index, _property)| {
                instance.get_physical_device_surface_support_khr(
                    physical_device,
                    index as u32,
                    data.surface,
                ).unwrap_or(false)
            }).map(|p| p as u32);
        println!("queue present {}", present.unwrap());

        // let present:u32 = 1;
        if let (Some(graphics), Some(present)) = (graphics, present) {
            Ok(Self { graphics, present })
        } else {
            Err(anyhow!(SuitabilityError("Missing required queue families.")))
        }
    }
}
