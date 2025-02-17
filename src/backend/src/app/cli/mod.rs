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
    SortOutUsers(SeedArgs),
    /// Seed the user table
    #[command(name = "seed-user")]
    SeedUser(SeedArgs),
    /// Seed the activity table
    #[command(name = "seed-activity")]
    SeedActivity(SeedArgs),
    #[command(name = "seed-all")]
    SeedAll(SeedArgs),
    SeedAdmin(SeedArgs),
}

#[derive(Parser, Debug)]
pub struct SeedArgs {
    /// Set to true if you intend to actually writing to the database
    #[clap(short, long)]
    pub write: bool,
}
