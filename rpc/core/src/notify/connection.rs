use crate::Notification;

pub type ChannelConnection = kash_notify::connection::ChannelConnection<Notification>;
pub use kash_notify::connection::ChannelType;
