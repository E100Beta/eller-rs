extern crate eller;

use std::env;

const USAGE: &'static str = "USAGE: rust_eller <width> <height> [h[orizontal]|v[ertical]|n[ormal]]";

fn main() {

    //Argument parsing
    if env::args().count() > 4 || env::args().count() == 1 {
        println!("{}", USAGE);
        return;
    }
    let width: usize = match env::args().nth(1) {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => {
                println!("Cannot parse first argument to number: {}.\n{}", e, USAGE);
                return;
            },
        },
        None => {
            return;
        },
    };
    let height: usize = match env::args().nth(2) {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => {
                println!("Cannot parse second argument to number: {}.\n{}", e, USAGE);
                return;
            },
        },
        None => {
            return;
        },
    };
    let orient = match env::args().nth(3) {
        Some(x) => match x.trim() {
            "h" | "horizontal" => eller::MazeOrient::Horizontal,
            "v" | "vertical" => eller::MazeOrient::Vertical,
            "n" | "normal" => eller::MazeOrient::Normal,
            _ => {
                println!("Cannot parse the mode, assuming normal");
                eller::MazeOrient::Normal
            }
        },
        None => eller::MazeOrient::Normal,
    };

    //Generate the maze
    let maze = eller::EllerMaze::generate(width, height, orient);
    println!("{}", maze);
}
