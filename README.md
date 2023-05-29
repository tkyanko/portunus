# Portunus
A Discord bot for sharing game keys.

## Setup
This project is intended to be built with Nix.
After installing Nix and enabling Flake support, run `nix build` to build a Docker image.
Run `docker load < result` to load the image into the local repository.

## How to use
Create a Discord bot in the Discord developer portal and generate a bot token.
Copy the `.env-sample` file to a new `.env` file and add the following to it:
- Bot token that was just generated
- Server ID that the bot will be attached to
- Location on the Docker host where the DB file will be stored

Start the bot with `docker-compose up -d`.
Once it is running, the following commands are available:
- `/addkey`
  - Adds a new key to the database for the specified game.
- `/getkey`
  - Gets the oldest key for the given game.
- `/listkeys`
  - Lists all available keys and how many for each title.

By default, each member of a server can only get one key per 24-hour period.
Also note that keys are provided on an honor system, as there is no way for the bot to verify the validity of provided keys.
It is recommended to only use this bot on servers where all members are trusted.
If needed, key add/get times and the number of keys each user has added/gotten are tracked in the database.
Any SQLite client can be used to track this information.

It is recommended that the database file is backed up regularly as well.