use anyhow::Result;

use vulkanalia::vk;
use vulkanalia::prelude::v1_0::*;

use crate::app::data::AppData;
use crate::device::queues::QueueFamilyIndices;


pub unsafe fn create_command_pool(
    instance: &Instance,
    device: &Device,
    data: &mut AppData
) -> Result<()> {
    let indices = QueueFamilyIndices::get(instance, data, data.physical_device)?;

    let info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::empty())
        .queue_family_index(indices.graphics);
    Ok(())
}
