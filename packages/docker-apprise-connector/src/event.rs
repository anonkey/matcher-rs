use std::collections::HashMap;

use matcher_derive_impl::matcher::Matcher;

use serde::{Deserialize, Serialize};

/// Actor describes something that generates events, like a container, network, or a volume.
#[derive(matcher_derive::Match, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventActor {
    /// The ID of the object emitting the event
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Various key/value attributes of the object, depending on its type.
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
}

/// EventMessage represents the information an event contains.
#[derive(matcher_derive::Match, Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    /// The type of object emitting the event
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<EventMessageTypeEnum>,

    /// The type of event
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[match_start_with]
    pub action: Option<String>,

    #[serde(rename = "Actor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<EventActor>,

    /// Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<EventMessageScopeEnum>,

    /// Timestamp of event
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,

    /// Timestamp of event, with nanosecond accuracy
    #[serde(rename = "timeNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_nano: Option<i64>,
}

#[derive(matcher_derive::Match)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum EventMessageTypeEnum {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "builder")]
    Builder,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "daemon")]
    Daemon,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "plugin")]
    Plugin,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "volume")]
    Volume,
}

impl ::std::fmt::Display for EventMessageTypeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventMessageTypeEnum::Empty => write!(f, ""),
            EventMessageTypeEnum::Builder => write!(f, "builder"),
            EventMessageTypeEnum::Config => write!(f, "config"),
            EventMessageTypeEnum::Container => write!(f, "container"),
            EventMessageTypeEnum::Daemon => write!(f, "daemon"),
            EventMessageTypeEnum::Image => write!(f, "image"),
            EventMessageTypeEnum::Network => write!(f, "network"),
            EventMessageTypeEnum::Node => write!(f, "node"),
            EventMessageTypeEnum::Plugin => write!(f, "plugin"),
            EventMessageTypeEnum::Secret => write!(f, "secret"),
            EventMessageTypeEnum::Service => write!(f, "service"),
            EventMessageTypeEnum::Volume => write!(f, "volume"),
        }
    }
}

impl ::std::str::FromStr for EventMessageTypeEnum {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(EventMessageTypeEnum::Empty),
            "builder" => Ok(EventMessageTypeEnum::Builder),
            "config" => Ok(EventMessageTypeEnum::Config),
            "container" => Ok(EventMessageTypeEnum::Container),
            "daemon" => Ok(EventMessageTypeEnum::Daemon),
            "image" => Ok(EventMessageTypeEnum::Image),
            "network" => Ok(EventMessageTypeEnum::Network),
            "node" => Ok(EventMessageTypeEnum::Node),
            "plugin" => Ok(EventMessageTypeEnum::Plugin),
            "secret" => Ok(EventMessageTypeEnum::Secret),
            "service" => Ok(EventMessageTypeEnum::Service),
            "volume" => Ok(EventMessageTypeEnum::Volume),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EventMessageTypeEnum {
    fn as_ref(&self) -> &str {
        match self {
            EventMessageTypeEnum::Empty => "",
            EventMessageTypeEnum::Builder => "builder",
            EventMessageTypeEnum::Config => "config",
            EventMessageTypeEnum::Container => "container",
            EventMessageTypeEnum::Daemon => "daemon",
            EventMessageTypeEnum::Image => "image",
            EventMessageTypeEnum::Network => "network",
            EventMessageTypeEnum::Node => "node",
            EventMessageTypeEnum::Plugin => "plugin",
            EventMessageTypeEnum::Secret => "secret",
            EventMessageTypeEnum::Service => "service",
            EventMessageTypeEnum::Volume => "volume",
        }
    }
}

#[derive(matcher_derive::Match)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
pub enum EventMessageScopeEnum {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "swarm")]
    Swarm,
}

impl ::std::fmt::Display for EventMessageScopeEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventMessageScopeEnum::Empty => write!(f, ""),
            EventMessageScopeEnum::Local => write!(f, "local"),
            EventMessageScopeEnum::Swarm => write!(f, "swarm"),
        }
    }
}

impl ::std::str::FromStr for EventMessageScopeEnum {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(EventMessageScopeEnum::Empty),
            "local" => Ok(EventMessageScopeEnum::Local),
            "swarm" => Ok(EventMessageScopeEnum::Swarm),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EventMessageScopeEnum {
    fn as_ref(&self) -> &str {
        match self {
            EventMessageScopeEnum::Empty => "",
            EventMessageScopeEnum::Local => "local",
            EventMessageScopeEnum::Swarm => "swarm",
        }
    }
}
