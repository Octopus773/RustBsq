mod bsq;

fn main() {
    let world = bsq::World::new("...\n...\n...");


    println!("is square {}", bsq::is_square_valid(&world, (0, 0), 3));

    match bsq::find_biggest_square(&world) {
        Some((i, j)) => println!("Coords {} {}", i, j),
        None => println!("No square found"),
    }
}
