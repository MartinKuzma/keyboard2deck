mod app;
mod keyboard;
mod macros;

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

    let mut app = App::new(String::from("config.yaml")).unwrap();
    app.run().unwrap();
}
