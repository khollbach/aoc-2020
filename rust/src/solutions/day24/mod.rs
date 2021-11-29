use std::io;
use crate::Res;
use input::read_input;
use part_1::part_1;
use part_2::part_2;

mod input;
mod dir;
mod point;
mod part_1;
mod part_2;

pub fn main() -> Res<()> {
    let input = read_input(io::stdin().lock());

    let mut black_tiles = part_1(&input);
    println!("{}", black_tiles.len());

    part_2(&mut black_tiles);
    println!("{}", black_tiles.len());

    Ok(())
}