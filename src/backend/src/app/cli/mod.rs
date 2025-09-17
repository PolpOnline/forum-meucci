use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Migrate the user
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
#[command(rename_all = "kebab-case")]
pub enum Command {
    /// Sort out the users
    SortOutForumUsers(SeedArgs),
    /// Seed the user table
    SeedUser(SeedArgs),
    /// Seed the forum_activity table
    SeedForumActivity(SeedArgs),
    SeedForumAll(SeedArgs),
    /// Seed the forum_admin table
    SeedForumAdmin(SeedArgs),
    /// Seed the hosts
    /// (extracted from the forum_activity table,
    /// do not use this if hosts were already seeded with that)
    SeedForumHosts(SeedArgs),
    /// Generate an Excel file with the rounds and activities with the users
    /// subscribed to those activities
    ExportForumRounds,
    ExportForumPresences,
}

#[derive(Parser, Debug)]
pub struct SeedArgs {
    /// Set to true if you intend to actually writing to the database
    #[clap(short, long)]
    pub write: bool,
}
