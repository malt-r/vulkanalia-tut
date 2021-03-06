// logging
#[allow(dead_code, unused_variables, unused_imports)]
use anyhow::{anyhow, Result};

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::vk::{ExtDebugUtilsExtension, KhrSurfaceExtension};
use vulkanalia::window as vk_window;

use winit::window::Window;

use crate::render::device;
use crate::render::instance;
use crate::render::validation;

#[derive(Clone, Debug)]
pub struct App {
    entry: Entry,
    instance: Instance,
    data: AppData,
    device: Device,
}

#[derive(Clone, Debug, Default)]
pub struct AppData {
    pub surface: vk::SurfaceKHR,
    pub messenger: vk::DebugUtilsMessengerEXT,
    // this will be implicitly destroyed, if the instance is destroyed,
    // so no further handling of this in App::destroy() required
    pub physical_device: vk::PhysicalDevice,

    // queues, which will be created along with logic device creation
    // queues are implicitly cleaned up, when the device is destroyed
    pub graphics_queue: vk::Queue,

    // the presentation queue also needs to be created with the logic
    // device
    pub present_queue: vk::Queue,
}

// TODO: expose own safe wrapper around vulkan calls, which asserts the calling
// of the correct invariants of the vulkan API functions
impl App {
    /// creates the app
    pub unsafe fn create(window: &Window) -> Result<Self> {
        // create library loader for the vulkan library (LIBRARY is a constant
        // path pointing to the Vulkan library)
        // this loads initial Vulkan commands from the library
        let loader = LibloadingLoader::new(LIBRARY)?;
        // load the entry point of the Vulkan library using the loader
        let entry = Entry::new(loader).map_err(|e| anyhow!("{}", e))?;
        // use the window and entry to create a vulkan instance
        let mut data = AppData::default();
        let instance = instance::create_instance(window, &entry, &mut data)?;

        // setup window surface
        data.surface = vk_window::create_surface(&instance, window)?;

        device::pick_physical_device(&instance, &mut data)?;
        let device = device::create_logical_device(&instance, &mut data)?;
        Ok(Self {
            entry,
            instance,
            data,
            device,
        })
    }

    /// renders one frame
    pub unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// destroy the app
    pub unsafe fn destroy(&mut self) {
        // None is for allocation callbacks
        self.device.destroy_device(None);

        // if validation is enabled, the debug messenger needs to be destroyed,
        // before the instance is destroyed
        if validation::ENABLED {
            self.instance
                .destroy_debug_utils_messenger_ext(self.data.messenger, None);
        }

        self.instance.destroy_surface_khr(self.data.surface, None);
        // be explicit about it
        self.instance.destroy_instance(None);
    }
}
