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

pub fn is_square_valid(world: &World, coords: (usize, usize), size: usize) -> bool {
    for l in world.world.lines().skip(coords.0).take(size) {
        if l[coords.1..coords.1 + size].graphemes(true).any(|g| g != world.empty_char) {
            return false;
        }
    }
    true
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