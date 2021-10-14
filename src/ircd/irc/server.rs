use crate::ircd::*;
use super::*;
use std::{
//    sync::Arc,
    collections::HashMap,
};
use async_std::{
    prelude::*,
    channel,
    net::SocketAddr
};
use crate::ircd::event::*;
use crate::ircd::irc::connection::EventDetail::*;
use log::{info,error};
use async_broadcast;
use futures::{select,FutureExt};

pub struct Server
{
    my_id: ServerId,
    name: String,
    net: Network,
    user_idgen: UserIdGenerator,
    channel_idgen: ChannelIdGenerator,
    message_idgen: MessageIdGenerator,
    eventlog: event::EventLog,
    event_receiver: async_broadcast::Receiver<Event>,
    listeners: ListenerCollection,
    connection_events: channel::Receiver<connection::ConnectionEvent>,
    client_connections: HashMap<ConnectionId, ClientConnection>,
    user_connections: HashMap<UserId, ConnectionId>,
    command_dispatcher: command::CommandDispatcher,
}

impl Server
{
    pub fn new(id: ServerId, name: String) -> Self
    {
        let (connevent_send, connevent_recv) = channel::unbounded::<connection::ConnectionEvent>();
        let mut eventlog = event::EventLog::new(EventIdGenerator::new(id, 1));
        let event_receiver = eventlog.attach();

        Self {
            my_id: id,
            name: name,
            net: Network::new(),
            user_idgen: UserIdGenerator::new(id, 1),
            channel_idgen: ChannelIdGenerator::new(id, 1),
            message_idgen: MessageIdGenerator::new(id, 1),
            eventlog: eventlog,
            event_receiver: event_receiver,
            listeners: ListenerCollection::new(connevent_send),
            connection_events: connevent_recv,
            client_connections: HashMap::new(),
            user_connections: HashMap::new(),
            command_dispatcher: command::CommandDispatcher::new(),
        }
    }

    pub fn add_listener(&mut self, address: SocketAddr)
    {
        self.listeners.add(address);
    }

    pub fn create_event<T: event::DetailType>(&self, target: <T as DetailType>::Target, details: T) -> event::Event
    {
        self.eventlog.create(target.into(), details.into())
    }

    pub fn next_user_id(&self) -> UserId
    {
        self.user_idgen.next()
    }

    pub fn next_channel_id(&self) -> ChannelId
    {
        self.channel_idgen.next()
    }

    pub fn next_message_id(&self) -> MessageId
    {
        self.message_idgen.next()
    }

    pub fn network(&self) -> &Network
    {
        &self.net
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn id(&self) -> ServerId
    {
        self.my_id
    }

    pub fn command_dispatcher(&self) -> &command::CommandDispatcher
    {
        &self.command_dispatcher
    }

    pub fn find_connection(&self, id: ConnectionId) -> Option<&ClientConnection>
    {
        self.client_connections.get(&id)
    }

    pub async fn run(&mut self)
    {
        loop {
            select! {
                res = self.connection_events.next().fuse() => {
                    match res {
                        Some(msg) => {
                            match msg.detail {
                                NewConnection(conn) => {
                                    info!("Got new connection {:?}", msg.source);
                                    self.client_connections.insert(msg.source, ClientConnection::new(conn));
                                },
                                Message(m) => { 
                                    info!("Got message from connection {:?}: {}", msg.source, m);

                                    if let Some(message) = ClientMessage::parse(msg.source, &m)
                                    {
                                        let mut processor = CommandProcessor::new(&self);
                                        processor.process_message(message).await;
                                        
                                        for action in processor.actions()
                                        {
                                            self.apply_action(action)
                                        }
                                    }
                                },
                                Error(e) => {
                                    error!("Got error from connection {:?}: {:?}", msg.source, e);
                                    self.client_connections.remove(&msg.source);
                                }
                            }
                        },
                        None => {
                            panic!("what to do here?");
                        }
                    }
                },
                res = self.event_receiver.next().fuse() => {
                    match res {
                        Some(event) => {
                            self.net.apply(&event);
                            self.handle_event(&event).await;
                        },
                        None => { 
                            panic!("what to do here?");
                        }
                    }
                }
            }
        }
    }
}

mod command_action;
mod event_handler;