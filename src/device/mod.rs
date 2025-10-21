pub mod logical;
pub mod physical;
pub mod queues;

use vulkanalia::vk;

const DEVICE_EXTENTIONS: &[vk::ExtensionName] = &[
    vk::KHR_SWAPCHAIN_EXTENSION.name
];
