use super::*;

command_handler!("PRIVMSG", PrivmsgHandler);

impl CommandHandler for PrivmsgHandler
{
    fn min_parameters(&self) -> usize { 2 }

    fn handle_user(&self, server: &Server, source: &wrapper::User, cmd: &ClientCommand, proc: &mut CommandProcessor) -> CommandResult
    {
        let target_name = &cmd.args[0];
        let target_id = if let Ok(chname) = ChannelName::new(target_name.clone()) {
            let channel = server.network().channel_by_name(&chname)?;
            channel.can_send(source)?;
            ObjectId::Channel(channel.id())
        } else if let Ok(nick) = Nickname::new(target_name.clone()) {
            ObjectId::User(server.network().user_by_nick(&nick)?.id())
        } else {
            return Err(numeric::NoSuchTarget::new(&target_name).into());
        };

        let details = event::details::NewMessage {
            source: source.id(),
            target: target_id,
            text: cmd.args[1].clone(),
        };
        proc.action(CommandAction::StateChange(server.create_event(server.next_message_id(), details)))?;
        Ok(())
    }
}