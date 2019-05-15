// Ferris-Bot, A Discord bot written in rust.
// Usage of discord api should be abstracted, asynchronous, and easy to use
// This file demonstrates an example usage of the api to-be implemented

use dotenv::dotenv;

mod bot_commands;
mod discord_api;

fn main() {
    //check for a .env file
    dotenv().ok();
    //get the bot api token from the environment
    let bot_token = std::env::var("FERRIS_BOT_TOKEN").expect("FERRIS_BOT_TOKEN must be set");
    // Initialize a new bot connection
    let con = discord_api::Connection::new(bot_token)
        .with_prefix('!') // the prefix to listen to for commands
        .with_commands(say, ping) // register the bot's commands
        .with_events(on_ready, on_message) // register all event callbacks
        .run(); // start the bot

    // The broadcast_message command sends a message
    // to the default channel of every connected server
    con.broadcast_message("hello world!");
}

#[derive(discord_api::Command)]
async fn say(ctx: discord_api::Context) {
    let mut buf = "";
    let to_say: &str = ctx.message;
    ferris_says::say(to_say, to_say.len(), &mut buf).unwrap();
    await!(ctx.send(buf));
}

#[derive(discord_api::Command)]
async fn ping(ctx: discord_api::Context) {
    await!(ctx.send("pong"));
}

#[derive(discord_api::Event)]
async fn on_ready(ctx: discord_api::Context) {
    println!("Bot user {} is ready!",);
}

#[derive(discord_api::Event)]
async fn on_message(ctx: discord_api::Context) {
    await!(ctx.send("hi!"));
}
