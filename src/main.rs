use std::io;
use std::env;
mod bsq;

fn main() -> io::Result<()> {
    let world = bsq::World::new_from_epitech_file(&env::args().nth(1).expect("Missing the path to file argument"))?;

    match bsq::find_biggest_square(&world) {
        Some((i, j, size)) => println!("Coords {} {} size: {}", i, j, size),
        None => println!("No square found"),
    };
    Ok(())
}
