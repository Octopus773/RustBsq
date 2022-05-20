use unicode_segmentation::UnicodeSegmentation;

pub struct World<'a> {
    world: &'a str,
    empty_char: &'a str,
    width: usize,
}


impl<'a> World<'a> {
    pub fn new(world: &'a str) -> World {
        World {
            world,
            empty_char: ".",
            width: world.lines().count(),
        }
    }
}

/// Checks if the the square of the specified size fit at the coords
///
/// # Arguments
///
/// * `world` - The data to look into
/// * `coords` - The top left corner of the square to check
/// * `size` - Size of the square to check starting at coords 
///
pub fn is_square_valid(world: &World, coords: (usize, usize), size: usize) -> bool {
    for l in world.world.lines().skip(coords.0).take(size) {
        let ss = match l.get(coords.1..coords.1 + size) {
            None => return false,
            Some(ss) => ss,
        };
        if ss.graphemes(true).any(|g| g != world.empty_char) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    #[test]
    fn is_square_valid() {
        let world = super::World::new("..3\n1..\n...");
        assert_eq!(super::is_square_valid(&world, (0, 0), 2), false);
        assert_eq!(super::is_square_valid(&world, (0, 0), 1), true);
        assert_eq!(super::is_square_valid(&world, (1, 1), 2), true);
        assert_eq!(super::is_square_valid(&world, (1, 0), 1), false);
        assert_eq!(super::is_square_valid(&world, (2, 2), 12), false);
    }
}
/*
fn get_max_size_from_coords(world: &World, coords: (usize, usize)) -> i32 {
    let lines: vec![&str] = world.world.lines().collect();
    let mut check_line = String::new();
    let mut square_width = 2;

    for i in 2..square_width {
        for j in 2..square_width {
            if 
        }
    }
    
    while check_line.graphemes(true).all(|g| g == world.empty_char) {

    }
    0

}*/

pub fn find_biggest_square(world: &World) -> Option<(usize, usize)> {
    for (i, l) in world.world.lines().enumerate() {
        for (j, c) in l.graphemes(true).enumerate() {
            if c == world.empty_char {
                return Some((i, j));
            }
        }
    }
    None
}