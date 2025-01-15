use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Migrate the user
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Seed the user table
    #[command(name = "seed-user")]
    SeedUser,
    /// Seed the event table
    #[command(name = "seed-event")]
    SeedEvent,
}
