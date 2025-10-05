use anyhow::{anyhow, Result};

use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension, KhrSwapchainExtension};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_4::*;

use winit::window::Window;

mod core;
mod device;
mod appdata;
mod instance;
mod queue_families;
mod swapchain;
mod pipeline;

use crate::app::instance::VALIDATION_ENABLED;


// Our Vulkan app.
#[derive(Clone, Debug)]
pub struct App {
    entry: Entry,
    instance: Instance,
    data: appdata::AppData,
    device: Device,
}

impl App {
    /// Creates our Vulkan app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = appdata::AppData::default();
        let instance = instance::create_instance(window, &entry, &mut data)?;

        data.surface = vulkanalia::window::create_surface(&instance, &window, &window)?;

        device::pick_physical_device(&instance, &mut data)?;
        let device = device::create_logical_device(&entry, &instance, &mut data)?;

        swapchain::create_swapchain(window, &instance, &device, &mut data)?;
        swapchain::create_swapchain_image_views(&device, &mut data)?;

        pipeline::create_pipeline(&device, &data)?;

        println!("App created");
        Ok(Self { entry, instance, data, device })
    }

    /// Renders a frame for our Vulkan app.
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    pub unsafe fn destroy(&mut self) {
        self.data.swapchain_image_views.iter()
            .for_each(|v| self.device.destroy_image_view(*v, None));
        self.device.destroy_swapchain_khr(self.data.swapchain, None);

        self.instance.destroy_surface_khr(self.data.surface, None);

        self.device.destroy_device(None);

        if VALIDATION_ENABLED {
            self.instance.destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_instance(None);


    }
}
