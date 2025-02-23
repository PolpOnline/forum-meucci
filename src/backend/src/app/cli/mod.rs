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
    /// Seed the admin table
    #[command(name = "seed-admin")]
    SeedAdmin(SeedArgs),
    /// Seed the hosts
    /// (extracted from the activity table,
    /// do not use this if hosts were already seeded with that)
    #[command(name = "seed-hosts")]
    SeedHosts(SeedArgs),
    /// Generate an Excel file with the rounds and activities with the users
    /// subscribed to those activities
    #[command(name = "export-rounds")]
    ExportRounds,
    #[command(name = "export-presences")]
    ExportPresences,
}

#[derive(Parser, Debug)]
pub struct SeedArgs {
    /// Set to true if you intend to actually writing to the database
    #[clap(short, long)]
    pub write: bool,
}
