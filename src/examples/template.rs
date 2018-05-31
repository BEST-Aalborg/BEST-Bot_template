#[macro_use]
extern crate template;
use template::logger;
use template::Name;
use template::plugin_api_v2::*;

/// Tells the build in logging system what the plugins name is.
/// Note: The variable name has to be "PLUGIN_NAME" or the plugin will not compile when using the build in logging
static PLUGIN_NAME: &str = "posts2mail";

/// Tells BEST-Bot what version of the api the plugin uses.
/// Note: The plugin will not be loaded if this function is missing.
#[no_mangle]
pub extern "C" fn api_version() -> u32 {
    2
}

/// Creates the plugin object which The BEST-Bot needs
#[no_mangle]
pub extern "C" fn load() -> Box<Plugin> {
    Box::new(Base {
        sender: None,
    })
}

struct Base {
    sender: Option<Sender>,
}

impl Name for Base {
    /// Returns the name of the plugin to The BEST-Bot
    fn name(&self) -> &'static str {
        PLUGIN_NAME
    }
}

impl Plugin for Base {
    /// This function is called right after the plugin is loaded into BEST-Bot
    fn on_plugin_load(&mut self, logger: logger::LoggerSender, channel: Sender) {
        logger::set_logger(logger);
        self.sender = Some(channel);
    }

    /// Tells BEST-Bot which events/actions the plugin requires to work
    fn event_subscript(&self) -> Vec<EventSubscribe> {
        vec![EventSubscribe::StandardMessage]
    }

    /// Then one of the event the plugin subscript to are triggered, this functions is called and
    /// the information are handed over to the plugin.
    fn event(&self, _event: Event) {

    }
}