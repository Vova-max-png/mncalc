use clap::Parser;
mod math;
use math::*;
mod args;
use args::*;

fn main() {
    let args = Args::parse();
    let square = getSquare(args.width, args.height);
    let chunks = getChunks(args.width);
    let cube = getCube(chunks);
    let chunksForOne = getChunksForOne(chunks, args.count);
    println!("Square: {}\nChunks: {}\nCube: {}\nChunks for one country: {}", square, chunks, cube * 16.0, chunksForOne);
}
