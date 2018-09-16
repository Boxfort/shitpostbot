extern crate discord;

use super::markov::MarkovChain;
use bot::discord::{Discord, Connection, GetMessages};
use bot::discord::model::{Event, ReadyEvent, ChannelId, MessageId};
use std::env;

pub struct DiscordBot {
    chain: MarkovChain,
    discord: Discord,
    connection: (Connection, ReadyEvent)
}

impl DiscordBot {
    pub fn new() -> DiscordBot {
        let discord = Discord::from_bot_token(
            &env::var("DISCORD_TOKEN").expect("Expected token"),
            ).expect("login failed");

        let connection = discord.connect().expect("connect failed");

        DiscordBot {
            chain: MarkovChain::new(),
            discord: discord,
            connection: connection,
        }
    }

    // TODO: Seperate this out.
    pub fn start(&mut self) {
        loop {
            match self.connection.0.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                    println!("{} says: {}", message.author.name, message.content);
                    if message.content == "!shitpost" {
                        self.discord.broadcast_typing(message.channel_id);
                        self.let_it_rip(message.channel_id);
                    } else if message.content == "!generate" {
                        let _ = self.discord.send_message(message.channel_id, "Learning the art of shitposting...", "", false);

                        let messages = self.get_all_messages(message.channel_id, message.id);

                        let _ = self.discord.send_message(message.channel_id, format!("Adding {} shitposts...", &messages.len()).as_str(), "", false);
                        self.chain.add_vec(messages);
                        let _ = self.discord.send_message(message.channel_id, "Done :^)", "", false);
                    } else {
                        self.chain.add_line(message.content.to_string());
                    }
                }
                Ok(_) => {}
                Err(discord::Error::Closed(code, body)) => {
                    println!("Gateway closed on us with code {:?}: {}", code, body);
                    break
                }
                Err(err) => println!("Receive error: {:?}", err)
            }
        }
    }

    fn get_all_messages(&mut self, channel : ChannelId, before : MessageId) -> Vec<String> {
        let mut id = before;
        let mut message_vec : Vec<String> = vec!();
        let mut retrieved = true;


        while retrieved {
            println!("Getting messages before {}", &id);

            let mut result = self.discord.get_messages(channel, GetMessages::Before(id), Some(100));

            let mut result: Vec<String> = match result {
                Ok(m) =>  {
                    if m.len() == 0 {
                        retrieved = false;
                        break;
                    }
                    id = m.iter().last().unwrap().id;
                    m.iter().map(|x| x.clone().content).collect()
                },
                Err(e) => panic!("Shits fucked"),
            };


            message_vec.append(&mut result);
        }

        message_vec
    }

    fn let_it_rip(&mut self, channel : ChannelId){
        let mut word : Option<String> = self.chain.get_next(None);
        let mut sentence : String = "".to_string();

        while word.clone().is_some() {
            sentence.push_str(&word.clone().unwrap());
            sentence.push_str(" ");
            println!("{}", word.clone().unwrap());
            word = self.chain.get_next(word);
        }

        let _ = self.discord.send_message(channel, sentence.as_str(), "", false);
    }
}
