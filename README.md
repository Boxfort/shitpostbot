# ShitpostBot [![Build Status](https://travis-ci.com/Boxfort/shitpostbot.svg?branch=master)](https://travis-ci.com/Boxfort/shitpostbot) [![Build Status](https://img.shields.io/badge/posts-shitty-red.svg)](https://i.ytimg.com/vi/lHcTJLv4Vtw/hqdefault.jpg)

ShitpostBot is a bot which learns from your messages and makes dumb posts.

Current Features:

- Markov Chain Message Generation
- Shitposts
- More to come...

## Requirements

- [Rust](https://www.rust-lang.org/en-US/install.html)
- [libsodium >= 1.0.6](https://download.libsodium.org/doc/)
- A discord server

## Building from source

- Clone the repository
- Navigate to the project folder
- Run `Cargo build --release`

## Running

- Create an application / bot [here](https://discordapp.com/developers/applications/)
- Invite the bot to your server using this link, and substituting `YOUR_CLIENT_ID_HERE` for the client_id of the bot. 
  - https://discordapp.com/oauth2/authorize?&client_id=YOUR_CLIENT_ID_HERE&scope=bot&permissions=0
- Add the environment variable `DISCORD_TOKEN` to your system and set it to your created bot's token.
- Navigate to `shitpostbot/target/release` and run `shitpostbot`
- All done!

## License

Released under the [GNU GPL v3.](https://trisquel.info/files/richard%20stallman.jpg)
