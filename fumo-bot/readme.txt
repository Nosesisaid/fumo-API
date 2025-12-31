Discord bot for managing the API.

Currently the only way to submit new fumos. This is to prevent abuse.

# Implemented features 
- Slash command to create a fumo
- Create submissions sending messages to a text-channel
- Dispatch submissions to a administration channels for curators
- Allow curators to approve and deny fumos using the discord client.


# Notes

For security, the database/bucket does not store unapproved submissions and only the discord proxy url is kept,
this image is later uploaded to the r2 bucket using the worker.

# Dev Env setup
To run this bot you will need a discord account, a server, two channels and a discord bot.
Create it creating an application in discord.dev and then a bot. Use the token provided in the token page, 


1. Follow the global contribution guide 
3. Copy the `.env.example` file to `.env` and fill in the values. 
4. Use `cargo run` to start the bot. in the fumo-bot directory.