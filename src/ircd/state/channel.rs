use crate::ircd::*;
use crate::ircd::id::*;
use crate::ircd::validated::*;

#[derive(Debug)]
pub struct Channel {
    pub id: ChannelId,
    pub name: ChannelName,
    pub mode: CModeId,
}

#[derive(Debug)]
pub struct Membership {
    pub id: MembershipId,
    pub channel: ChannelId,
    pub user: UserId,
}

#[derive(Debug)]
pub struct ChannelMode {
    pub id: CModeId,
    pub modes: ChannelModeFlags,
}

impl Channel {
    pub fn new(id: ChannelId, name: &ChannelName, mode: CModeId) -> Self
    {
        Channel{ id: id, name: name.clone(), mode: mode }
    }
}

impl ChannelMode {
    pub fn new(id: CModeId, modes: ChannelModeFlags) -> Self
    {
        ChannelMode{ id: id, modes: modes }
    }
}

impl Membership {
    pub fn new(id: MembershipId, user: UserId, channel: ChannelId) -> Membership 
    {
        Membership{ id: id, user: user, channel: channel }
    }
}