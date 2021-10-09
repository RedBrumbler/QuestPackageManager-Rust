use clap::{Clap, AppSettings};

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    /// Clear the cache
    #[clap(subcommand)]
    pub op: CacheOperation
}

#[derive(Clap, Debug)]
pub enum CacheOperation {
    /// Clear the cache
    Clear
}

pub fn execute_cache_operation(operation: Cache)
{
    match operation.op {
        CacheOperation::Clear => clear(),
    }
}

fn clear()
{
    println!("It should clear the cached files now");

    // clear cached dependencies from cachce, probably C drive
}