mod bot;
mod markov;

use bot::DiscordBot;

fn main() {
    let mut bot = DiscordBot::new();
    bot.start();
}
