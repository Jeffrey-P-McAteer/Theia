
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Theia", about = "A video stream manager")]
pub struct Args {
	name: String
}
