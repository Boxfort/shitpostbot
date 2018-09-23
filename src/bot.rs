extern crate discord;

use super::markov::MarkovChain;
use bot::discord::model::{ChannelId, Event, Message, MessageId};
use bot::discord::{Connection, Discord, GetMessages, State};
use std::env;

pub struct DiscordBot {
    chain: MarkovChain,
    discord: Discord,
    connection: Connection,
    state: State,
}

impl DiscordBot {
    pub fn new() -> DiscordBot {
        let discord = Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("Expected token"))
            .expect("login failed");

        let (connection, ready) = discord.connect().expect("connect failed");

        DiscordBot {
            chain: MarkovChain::new(),
            discord: discord,
            connection: connection,
            state: State::new(ready),
        }
    }

    /// Starts the bot listening to incoming messages.
    pub fn start(&mut self) {
        loop {
            match self.connection.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                    // Log Message
                    println!("{} says: {}", message.author.name, message.content);

                    // If message was posted by bot then ignore.
                    if message.author.id == self.state.user().id {
                        continue;
                    }

                    match message.content.as_ref() {
                        "!shitpost" => {
                            let _result = self.discord.broadcast_typing(message.channel_id);
                            self.let_it_rip(message.channel_id);
                        }
                        "!generate" => {
                            self.generate(message);
                        }
                        _ => self.chain.add_line(message.content.to_string()),
                    }
                }

                Ok(_) => continue,

                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    break;
                }
                Err(err) => println!("Receive error: {:?}", err),
            }
        }
    }

    fn generate(&mut self, message: Message) {
        // Retreive messages
        let _ = self.discord.send_message(
            message.channel_id,
            "Learning the art of shitposting...",
            "",
            false,
        );
        let messages = self.get_all_messages(message.channel_id, message.id);

        // Add messages to the chain
        let _ = self.discord.send_message(
            message.channel_id,
            format!("Adding {} shitposts...", &messages.len()).as_str(),
            "",
            false,
        );
        self.chain.add_vec(messages);

        // Send message to say it's done.
        let _ = self.discord
            .send_message(message.channel_id, "Done :^)", "", false);
    }

    // TODO: Remove messages from the bot
    //       Add a limit
    fn get_all_messages(&mut self, channel: ChannelId, before: MessageId) -> Vec<String> {
        let mut id = before;
        let mut message_vec: Vec<String> = vec![];

        loop {
            println!("Getting messages before {}", &id);

            let result = self.discord
                .get_messages(channel, GetMessages::Before(id), Some(100));

            let mut result: Vec<String> = match result {
                Ok(m) => {
                    if m.len() == 0 {
                        break;
                    }
                    id = m.iter().last().unwrap().id;
                    m.iter().map(|x| x.clone().content).collect()
                }
                Err(e) => panic!("Shits fucked"),
            };

            message_vec.append(&mut result);
        }

        message_vec
    }

    /// Generates a sentence from the markov chain
    /// and posts it to the specified channelID.
    fn let_it_rip(&mut self, channel: ChannelId) {
        let mut word: Option<String> = self.chain.get_next(None);
        let mut sentence: String = "".to_string();

        while word.clone().is_some() {
            sentence.push_str(&word.clone().unwrap());
            sentence.push_str(" ");
            println!("{}", word.clone().unwrap());
            word = self.chain.get_next(word);
        }

        let _ = self.discord
            .send_message(channel, sentence.as_str(), "", false);
    }
}
