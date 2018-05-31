pub extern crate slack;
pub extern crate slack_api as api;
pub extern crate slack_hook;
#[macro_use]
extern crate lazy_static;
extern crate crossbeam_channel;
use crossbeam_channel::Sender;

pub mod channel_return;
pub mod logger;

extern crate serde;
#[macro_use]
extern crate serde_derive;

type Channel<Replay, Request> = (Sender<Replay>, Request);

pub trait Name {
    /// Returns the name of the plugin to The BEST-Bot
    fn name(&self) -> &'static str;
}

pub mod plugin_api_v2 {
    use std::any::Any;
    use super::api::MessageStandard;
    use std::path::PathBuf;
    use super::Name;
    use super::crossbeam_channel;
    use super::logger::LoggerSender;

    pub type Channel = super::Channel<Reply, Request>;
    pub type Sender = crossbeam_channel::Sender<Channel>;
    pub type Receiver = crossbeam_channel::Receiver<Channel>;

    /// Supported events
    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum EventSubscribe {
        StandardMessage,
    }

    /// Events delivery
    #[derive(Eq, PartialEq, Debug)]
    pub enum Event<'a> {
        StandardMessage(&'a MessageStandard),
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum Request {
        ApiToken,
        AdminApiToken,
        WebHooksIncomingToken,
        WebHooksOutgoingToken,

        GetChannelName(String),

        ConfigPath,
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum Reply {
        ApiToken(String),
        AdminApiToken(String),
        WebHooksIncomingToken(String),
        WebHooksOutgoingToken(String),

        ChannelName(String),
        ChannelNotFound,

        ConfigPath(PathBuf),
        NotConfigured,
    }

    pub trait Plugin: Any + Send + Sync + Name {
        /// This function is called right after the plugin is loaded into BEST-Bot
        fn on_plugin_load(&mut self, logger: LoggerSender, channel: Sender);

        /// Tells BEST-Bot which events/actions the plugin requires to work
        fn event_subscript(&self) -> Vec<EventSubscribe>;

        /// Then one of the event the plugin subscript to are triggered, this functions is called and
        /// the information are handed over to the plugin.
        fn event(&self, event: Event);
    }
}

pub mod plugin_api_v1 {
    use std::any::Any;
    use super::api::MessageStandard;
    use std::path::PathBuf;

    /// Supported events
    #[derive(Eq, PartialEq, Hash, Debug)]
    pub enum EventSubscribe {
        StandardMessage,
    }

    /// Events delivery
    #[derive(Eq, PartialEq)]
    pub enum Event<'a> {
        StandardMessage(&'a MessageStandard),
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
        fn event_subscript(&self) -> Vec<EventSubscribe>;

        /// Then one of the event the plugin subscript to are triggered, this functions is called and
        /// the information are handed over to the plugin.
        fn event(&self, event: Event);
    }
}