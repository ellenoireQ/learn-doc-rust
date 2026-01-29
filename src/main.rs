use clap::Parser;

use crate::docs::ch01_02_hello_world::hello_world::hello_world;
mod docs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    args: String,
}

fn main() {
    let args = Args::parse();

    match args.args.as_str() {
        "hello_world" => {
            hello_world();
        }
        _ => { //}
        }
    }
}
