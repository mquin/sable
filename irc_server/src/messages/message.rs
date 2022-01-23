use irc_network::wrapper::{
    Channel
};
use irc_network::validated::*;
use ircd_macros::define_messages;
use super::*;

define_messages! {
    Nick    => { (source, newnick: &Nickname)               => ":{source} NICK {newnick}" },
    Join    => { (source, chan: &Channel.name())            => ":{source} JOIN {chan}" },
    Part    => { (source, chan: &ChannelName, msg: &str)    => ":{source} PART {chan} :{msg}" },
    Invite  => { (source, target, chan: &Channel.name())    => ":{source} INVITE {target} :{chan}" },
    Quit    => { (source, message: &str)                    => ":{source} QUIT :{message}" },
    Topic   => { (source, chan: &Channel.name(), text: &str)=> ":{source} TOPIC {chan} :{text}" },

    Mode    => { (source, target, changes: &str)            => ":{source} MODE {target} {changes}" },

    Notice  => { (source, target, message: &str)            => ":{source} NOTICE {target} :{message}" },
    Privmsg => { (source, target, message: &str)            => ":{source} PRIVMSG {target} :{message}" },
    Message => { (source, target, message_type: state::MessageType, message: &str)
                                                            => ":{source} {message_type} {target} :{message}" },

    Ping    => { (source, target, cookie: &str)             => ":{source} PING {target} :{cookie}" },
    Pong    => { (source, cookie: &str)                     => ":{source} PONG {source} :{cookie}" },

    Error   => { (text: &str)   => "ERROR :{text}" }
}