use clap::Parser;
#[derive(Debug, Parser)]
pub struct CalpStuff {
    #[arg(long)]
    pub email: String, //target
    #[arg(long)]
    pub promote: bool,
}
