use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "tbd",
    about = "Something, something tendies",
    version = "0.1.0"
)]
pub struct Opt {
    #[structopt(short, long, use_delimiter = true)]
    /// Comma separated list of stock symbols to start app with
    pub stocks: Vec<String>,
    #[structopt(long)]
    /// Hide help icon in top right
    pub hide_help: bool,
}

pub fn get_opts() -> Opt {
    Opt::from_args()
}