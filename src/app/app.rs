use anyhow::{anyhow, Result};

use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension, KhrSwapchainExtension};
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_4::*;

use winit::window::Window;

use crate::instance::VALIDATION_ENABLED;
use crate::app::data;
use crate::{
    device,
    swapchain::{
        swapchain,
        image_views
    },
    pipeline,
    instance,
    frame,
    command
};

// Our Vulkan app.
#[derive(Clone, Debug)]
pub struct App {
    entry: Entry,
    instance: Instance,
    data: data::AppData,
    device: Device,
}

impl App {
    /// Creates our Vulkan app.
    pub unsafe fn create(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;
        let mut data = data::AppData::default();
        let instance = instance::create_instance(window, &entry, &mut data)?;

        data.surface = vulkanalia::window::create_surface(&instance, &window, &window)?;

        device::physical::pick_physical_device(&instance, &mut data)?;
        let device = device::logical::create_logical_device(&entry, &instance, &mut data)?;

        swapchain::create_swapchain(window, &instance, &device, &mut data)?;
        image_views::create_swapchain_image_views(&device, &mut data)?;

        pipeline::render_pass::create_render_pass(&instance, &device, &mut data)?;
        pipeline::graphics::create_pipeline(&device, &mut data)?;
        frame::create_framebuffers(&device, &mut data)?;
        command::pool::create_command_pool(&instance, &device, &mut data)?;

        println!("App created");
        Ok(Self { entry, instance, data, device })
    }

    /// Renders a frame for our Vulkan app.
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// Destroys our Vulkan app.
    pub unsafe fn destroy(&mut self) {
        self.data.framebuffers
            .iter()
            .for_each(|i| self.device.destroy_framebuffer(*i, None));

        self.device.destroy_pipeline(self.data.pipeline, None);
        self.device.destroy_pipeline_layout(self.data.pipeline_layout, None);
        self.device.destroy_render_pass(self.data.render_pass, None);

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
