use anyhow::{anyhow, Result};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
// vulkan imports
use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // Create window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vulkanalia Tutorial")
        // using the logical size will be dpi-scaled
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&event_loop)?;

    let mut app = unsafe { App::create(&window)? };
    let mut destroying = false;
    event_loop.run(move |event, _, control_flow| {
        // poll for events, even if none is available
        *control_flow = ControlFlow::Poll;

        match event {
            // render a new frame, if all events other than the RequestRequested have
            // been cleared
            Event::MainEventsCleared if !destroying => unsafe { app.render(&window) }.unwrap(),
            // emitted, if the OS sends an event to the winit window (specifically
            // a request to close the window)
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                log::info!("Hello");
                unsafe {
                    app.destroy();
                }
            }
            _ => {}
        }
    });
}

/// creates a new vulkan instance using entry.create_instance
/// the window parameter is used to enumerate all required extensions
///
/// The 'Instance' returned by this function is not a raw vulkan instance
/// (this would be vk::Instance), it is an abstraction created by vulkanalia,
/// which combines the raw vulkan instance and the loaded commands for that instance
unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    // no strictly necessary
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"Vulkan Tutorial\0")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"No Engine\0")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    // lots of information is passed to vulkan (and vulkanalia) by passing structs
    // so for creating an instance, we need to fill in one more struct
    //
    // enumerate all globally required extensions for vk_window and convert them to
    // null terminated c_strings (*const i8)
    //
    // globally means global for the whole program
    let extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();

    // create a vulkan instance (the connection between our program and the
    // Vulkan library)
    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions);

    Ok(entry.create_instance(&info, None)?)
}

#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

// TODO: expose own safe wrapper around vulkan calls, which asserts the calling
// of the correct invariants of the vulkan API functions
impl App {
    /// creates the app
    unsafe fn create(window: &Window) -> Result<Self> {
        // create library loader for the vulkan library (LIBRARY is a constant
        // path pointing to the Vulkan library)
        // this loads initial Vulkan commands from the library
        let loader = LibloadingLoader::new(LIBRARY)?;
        // load the entry point of the Vulkan library using the loader
        let entry = Entry::new(loader).map_err(|e| anyhow!("{}", e))?;
        // use the window and entry to create a vulkan instance
        let instance = create_instance(window, &entry)?;
        Ok(Self { entry, instance })
    }

    /// renders one frame
    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    /// destroy the app
    unsafe fn destroy(&mut self) {
        // be explicit about it
        self.instance.destroy_instance(None);
    }
}

#[derive(Clone, Debug, Default)]
struct AppData {}
