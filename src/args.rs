use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg()]
    pub width: f64,

    #[arg()]
    pub height: f64,

    #[arg()]
    pub count: f64
}