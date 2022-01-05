use crate::*;
use super::*;

/// A wrapper around a [`state::User`]
pub struct User<'a> {
    network: &'a Network,
    data: &'a state::User,
}

impl User<'_> {
    /// Return this object's ID
    pub fn id(&self) -> UserId {
        self.data.id
    }

    /// Infallibly returns a nickname for this user. 
    /// If a nickname binding exists, then the associated nick is returned; otherwise,
    /// a fallback nick based on the hash of the user ID is used - this is the same 
    /// computed nickname used in case of binding collisions.
    pub fn nick(&self) -> Nickname
    {
        if let Ok(binding) = self.nick_binding()
        {
            binding.nick()
        }
        else
        {
            log::error!("Attempted to get nickname of user {:?} without a nick binding", self.data.id);
            state_utils::hashed_nick_for(self.data.id)
        }
    }
    
    /// Return the nickname binding currently active for this user
    pub fn nick_binding(&self) -> LookupResult<NickBinding> {
        self.network.nick_binding_for_user(self.data.id)
    }

    /// The user's username
    pub fn user(&self) -> &Username {
        &self.data.user
    }

    /// The user's visible hostname, for client protocol purposes
    pub fn visible_host(&self) -> &Hostname {
        &self.data.visible_host
    }

    /// The user's realname
    pub fn realname(&self) -> &str {
        &self.data.realname
    }

    /// The user's current modes
    pub fn mode(&self) -> LookupResult<UserMode> {
        self.network.user_mode(self.data.mode_id)
    }

    /// The server through which the user is connected
    pub fn server(&self) -> LookupResult<Server> {
        self.network.server(self.data.server)
    }

    /// Iterate over the user's channel memberships
    pub fn channels(&self) -> impl Iterator<Item=Membership> {
        let my_id = self.data.id;
        self.network.raw_memberships().filter(move|x| x.user == my_id).wrap(self.network)
    }

    /// Test whether the user is in a given channel
    pub fn is_in_channel(&self, c: ChannelId) -> Option<Membership>
    {
        self.channels().filter(|m| m.channel_id() == c).next()
    }

    /// Test whether an invite exists for this user to a given channel
    pub fn has_invite_for(&self, c: ChannelId) -> Option<ChannelInvite>
    {
        self.network.channel_invite(InviteId::new(self.data.id, c)).ok()
    }
}

impl<'a> super::ObjectWrapper<'a> for User<'a> {
    type Underlying = state::User;

    fn wrap(net: &'a Network, data: &'a state::User) -> Self {
        Self{network: net, data: data}
    }
}