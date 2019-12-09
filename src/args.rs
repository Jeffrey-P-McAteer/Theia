
use structopt::StructOpt;
use url::{Url};

#[derive(Debug, StructOpt)]
#[structopt(name = "Theia", about = "A video stream manager")]
pub struct Args {
	// Commonly used things
	pub mrl: Option<Url>,

	// Less commonly used
	
	/// Prints location of config file and cache directory for your OS
	#[structopt(long)]
    pub dump_config_info: bool,

	/// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

}

impl Args {

}

