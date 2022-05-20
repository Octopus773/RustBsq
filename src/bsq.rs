use std::fs;
use std::io;

pub struct World {
    world: String,
    empty_char: char,
    width: usize,
}


impl World {
    pub fn new(world: String) -> World {
        World {
            width: world.lines().count(),
            world,
            empty_char: '.',
        }
    }

    /// Epitech Bsq scenario enforcing almost all mandatory checks (missing that only some types of char are allowed)
    /// 
    /// # Arguments
    /// 
    /// * `path_to_file` - The path to the Epitech formatted BSQ file
    /// 
    pub fn new_from_epitech_file(path_to_file: &str) -> io::Result<World> {
        let data = fs::read_to_string(path_to_file)?;
        let mut lines = data.lines();

        let number_of_lines = match lines.next() {
            None => return Err(io::Error::new(io::ErrorKind::Other, "The file must have the number of lines at the first line")),
            Some(s) => s.parse::<usize>().unwrap()
        };

        let mut i = 0;
        for l in lines {
            i += 1;
            if l.len() != number_of_lines {
                return Err(io::Error::new(io::ErrorKind::Other, format!("The content of the file must be a square; error on line {}", i)));
            }
        }
        if i != number_of_lines {
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                format!("The content of the file must be a square; Too much lines, found {}, expected {}", i , number_of_lines)
            ));
        }

        Ok(World {
            world: data,
            empty_char: '.',
            width: number_of_lines
        })
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
    let mut assessed_lines = 0;
    for l in world.world.lines().skip(coords.0).take(size) {
        assessed_lines += 1;
        let ss = match l.get(coords.1..coords.1 + size) {
            None => return false,
            Some(ss) => ss,
        };
        if ss.chars().any(|g| g != world.empty_char) {
            return false;
        }
    }
    assessed_lines == size
}

/// Gives the size of the maximum legal square at coords
/// 
/// # Warning
/// 
/// Should be called on empty coords (to have at least a valid square of 1)
///
/// # Arguments
///
/// * `world` - The data to look into
/// * `coords` - The top left corner of the square to check
///
fn get_max_size_from_coords(world: &World, coords: (usize, usize)) -> usize {
    let mut max_size = 2;
    loop {
        if !is_square_valid(world, coords, max_size) {
            return max_size - 1;
        }
        max_size += 1;
    }
}


/// Gives the coords and the size of the biggest square on the world
/// 
/// # Arguments
/// 
/// * `world` - The world to look for the square
/// 
/// Returns None if no square was found
/// 
/// Return Some(y_coord, x_coord, size) of the biggest possible square on the map
/// 
pub fn find_biggest_square(world: &World) -> Option<(usize, usize, usize)> {
    let mut max_coords = (0, 0, 0);
    for (i, l) in world.world.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c != world.empty_char {
                continue;
            }
            let m_s = get_max_size_from_coords(world, (i, j));

            if m_s > max_coords.2 {
                max_coords = (i, j, m_s);
            }
        }
    }
    if max_coords.2 == 0 {
        return None;
    }
    Some(max_coords)
}

#[cfg(test)]
mod test {
    #[test]
    fn is_square_valid() {
        let world = super::World::new(String::from("..3\n1..\n..."));
        assert_eq!(super::is_square_valid(&world, (0, 0), 2), false);
        assert_eq!(super::is_square_valid(&world, (0, 0), 1), true);
        assert_eq!(super::is_square_valid(&world, (1, 1), 2), true);
        assert_eq!(super::is_square_valid(&world, (1, 0), 1), false);
        assert_eq!(super::is_square_valid(&world, (2, 2), 12), false);
        assert_eq!(super::is_square_valid(&world, (2, 1), 2), false);
    }

    #[test]
    fn get_max_size_from_coords() {
        let world = super::World::new(String::from("..3\n1..\n..."));
        assert_eq!(super::get_max_size_from_coords(&world, (0, 0)), 1);
        assert_eq!(super::get_max_size_from_coords(&world, (1, 1)), 2);
        assert_eq!(super::get_max_size_from_coords(&world, (2, 1)), 1);
    }

    #[test]
    fn find_biggest_square() {
        let world = super::World::new(String::from("..3\n1..\n..."));
        assert_eq!(super::find_biggest_square(&world), Some((1, 1, 2)));


        let world = super::World::new(String::from(".....\n.....\n....."));
        assert_eq!(super::find_biggest_square(&world), Some((0, 0, 3)));

        let world = super::World::new(String::from(".....\n.....\n.....\n.....\n.....\n"));
        assert_eq!(super::find_biggest_square(&world), Some((0, 0, 5)));
    }
}