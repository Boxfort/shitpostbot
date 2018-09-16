extern crate discord;

use super::markov::MarkovChain;
use bot::discord::{Discord, Connection};
use bot::discord::model::{Event, ReadyEvent, ChannelId};
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

    pub fn start(&mut self) {
        loop {
            match self.connection.0.recv_event() {
                Ok(Event::MessageCreate(message)) => {
                    println!("{} says: {}", message.author.name, message.content);
                    if message.content == "!shitpost" {
                        self.let_it_rip(message.channel_id);
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

    fn let_it_rip(&mut self, id : ChannelId){
        let mut word : Option<String> = self.chain.get_next(None);
        let mut sentence : String = "".to_string();

        while word.clone().is_some() {
            sentence.push_str(&word.clone().unwrap());
            sentence.push_str(" ");
            println!("{}", word.clone().unwrap());
            word = self.chain.get_next(word);
        }

        let _ = self.discord.send_message(id, sentence.as_str(), "", false);
    }
}
