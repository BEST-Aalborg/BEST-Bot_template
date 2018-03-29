pub extern crate slack;
pub extern crate slack_api as api;

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod plugin_api_v1 {
    use std::any::Any;
    use super::api::MessageStandard;
    use std::path::PathBuf;

    /// Supported events
    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum EVENT_SUBSCRIBE {
        STANDARD_MESSAGE,
    }

    /// Events delivery
    #[derive(Eq, PartialEq)]
    pub enum EVENT<'a> {
        STANDARD_MESSAGE(&'a MessageStandard),
    }

    /// Struct for handling Slack keys
    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct Slack {
        /// The token are meant to be from a Slack Bot, but can also be from a normal user.
        /// Look in to Legacy Tokens on api.slack.com
        pub api_token: String,

        /// The token have to be from a normal user with admin privileges.
        /// Look in to Legacy Tokens on api.slack.com to figure out have to generate the token.
        pub admin_api_token: String,
    }

    pub trait Plugin: Any + Send + Sync {
        /// Returns the name of the plugin to The BEST-Bot
        fn name(&self) -> &'static str;

        /// This function is called right after the plugin is loaded into BEST-Bot
        fn on_plugin_load(&mut self, slack: Slack, config_path: PathBuf);

        /// Tells BEST-Bot which events/actions the plugin requires to work
        fn event_subscript(&self) -> Vec<EVENT_SUBSCRIBE>;

        /// Then one of the event the plugin subscript to are triggered, this functions is called and
        /// the information are handed over to the plugin.
        fn event(&self, event: EVENT);
    }
}