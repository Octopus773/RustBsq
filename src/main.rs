use std::io;
use std::env;
mod bsq;

fn main() -> io::Result<()> {
    let world = bsq::World::new_from_epitech_file(&env::args().nth(1).expect("Missing the path to file argument"))?;

    match bsq::find_biggest_square(&world) {
        Some(square) => bsq::print_world_and_square(world, &square),
        None => bsq::print_world(&world),
    };
    Ok(())
}
