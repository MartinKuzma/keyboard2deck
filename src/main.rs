mod app;
mod config;
mod device;
mod keyboard;
mod macros;

use std::fs;

use app::App;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[structopt(short = 'l', long = "list-devices")]
    list_devices: bool,
    #[structopt(short = 'c', required_unless_present = "list-devices")]
    config: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.list_devices {
        app::list_devices();
        return;
    }

    let config_content = fs::read_to_string(args.config.unwrap()).unwrap();
    let configuration: config::Config = serde_yaml::from_str(config_content.as_str()).unwrap();

    let mut app = App::new(configuration).unwrap();
    app.run().unwrap();
}
