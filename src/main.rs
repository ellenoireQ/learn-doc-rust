use clap::Parser;

use crate::docs::{
    ch01_02_hello_world::hello_world::hello_world,
    ch02_00_guessing_game_tutorial::guessing_game::guessing_game,
    ch08_01_vectors::vectors::vectors, ch10_02_lifetime::lifetime::lifetime,
    traits::traits::test_traits,
};
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
        "guessing_game" => {
            guessing_game();
        }
        "lifetime" => {
            lifetime();
        }
        "vectors" => {
            vectors();
        }
        "traits" => {
            test_traits();
        }
        _ => { //}
        }
    }
}
