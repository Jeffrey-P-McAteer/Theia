
use app_dirs::AppInfo;
use structopt::StructOpt;

pub const APP_INFO: AppInfo = AppInfo{name: "Theia", author: "jeffrey.p.mcateer"};

mod config;
mod args;
mod gui;

fn main() {
    let args = args::Args::from_args();

    if args.dump_config_info {
        config::print_config_files();
        return;
    }

    gui::open(&config::get_config(), args.mrl);

}

