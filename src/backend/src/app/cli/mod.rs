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
    #[command(name = "sort-out-forum-users")]
    SortOutForumUsers(SeedArgs),
    /// Seed the user table
    #[command(name = "seed-user")]
    SeedUser(SeedArgs),
    /// Seed the forum_activity table
    #[command(name = "seed-forum-activity")]
    SeedForumActivity(SeedArgs),
    #[command(name = "seed-all")]
    SeedForumAll(SeedArgs),
    /// Seed the forum_admin table
    #[command(name = "seed-forum-admin")]
    SeedForumAdmin(SeedArgs),
    /// Seed the hosts
    /// (extracted from the forum_activity table,
    /// do not use this if hosts were already seeded with that)
    #[command(name = "seed-forum-hosts")]
    SeedForumHosts(SeedArgs),
    /// Generate an Excel file with the rounds and activities with the users
    /// subscribed to those activities
    #[command(name = "export-forum-rounds")]
    ExportForumRounds,
    #[command(name = "export-forum-presences")]
    ExportForumPresences,
}

#[derive(Parser, Debug)]
pub struct SeedArgs {
    /// Set to true if you intend to actually writing to the database
    #[clap(short, long)]
    pub write: bool,
}
