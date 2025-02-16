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
    /// Sort out the users
    #[command(name = "sort-out-users")]
    SortOutUsers,
    /// Seed the user table
    #[command(name = "seed-user")]
    SeedUser,
    /// Seed the activity table
    #[command(name = "seed-activity")]
    SeedActivity,
    #[command(name = "seed-all")]
    SeedAll,
}
