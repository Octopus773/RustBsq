use std::fs;
use std::io;

pub struct World {
    world: String,
    empty_char: char,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct Square {
    pub x: usize,
    pub y: usize,
    pub size: usize
}

impl Square {
    pub fn new((y, x, size): (usize, usize, usize)) -> Square {
        Square {y, x, size}
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.size == other.size
    }
}

impl World {
    pub fn new(mut world: String) -> Result<World, String> {
        let mut lines = world.lines();

        let number_of_lines = match lines.next() {
            None => return Err(String::from("The file must have the number of lines at the first line")),
            Some(s) => s.parse::<usize>().unwrap()
        };

        let mut i = 0;
        let mut line_length: Option<usize> = None;
        for l in lines {
            i += 1;
            let len_line = l.len();

            if line_length.is_none() {
                line_length = Some(len_line);
                continue;
            }
            if len_line != line_length.unwrap() {
                return Err(format!("The content of the file must be a rectangle; error on line {}", i + 1));
            }
        }
        if i != number_of_lines {
            return Err(format!("The file has not the number of lines expected; found {}, expected {}", i , number_of_lines));
        }

        world.drain(..=world.chars().position(|c| c == '\n').unwrap());
        Ok(World {
            height: world.lines().count(),
            width: line_length.unwrap(),
            world,
            empty_char: '.',
        })
    }

    /// Epitech Bsq scenario enforcing almost all mandatory checks (missing that only some types of char are allowed)
    /// 
    /// # Arguments
    /// 
    /// * `path_to_file` - The path to the Epitech formatted BSQ file
    /// 
    pub fn new_from_epitech_file(path_to_file: &str) -> io::Result<World> {
        match World::new(fs::read_to_string(path_to_file)?) {
            Err(s) => Err(io::Error::new(io::ErrorKind::Other, s)),
            Ok(w) => Ok(w),
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
/// # Warning
///
/// The function only works with sqaure shaped world with a single `\n` to separate each line
///
pub fn is_square_valid(world: &World, s: &Square) -> bool {
    if s.y + s.size > world.width || s.x + s.size > world.width {
        return false;
    }
    for i in s.y..s.y + s.size {
        for j in s.x..s.x + s.size {
            let width = if i != world.width { world.width + 1} else { world.width };
            if world.world.as_bytes()[i * width + j] != '.' as u8 {
                return false;
            }
        }
    }
    true
}

/// Checks if the the square of the specified size will fit at the coords if grown by 1
///
/// # Arguments
///
/// * `world` - The data to look into
/// * `coords` - The top left corner of the square to check
/// * `size` - Size of the square to check starting at coords 
///
/// # Warning
///
/// The function only works with sqaure shaped world with a single `\n` to separate each line
///
pub fn is_square_enlargment_valid(world: &World, s: &Square) -> bool {
    if s.y + s.size + 1 > world.height || s.x + s.size + 1 > world.width {
        return false;
    }
    // one \n at the end of each line
    let width = world.width + 1;
    for i in s.y..=s.y + s.size {
        for j in s.x..=s.x + s.size {

            let width = world.width + 1;
            if i != s.y + s.size {
                if world.world.as_bytes()[i * width + j + s.size] != '.' as u8 {
                    return false;
                }
                break;
            }
        }
    }
    let start_index = (s.y + s.size) * width + s.x;
    let new_bottom_line:&[u8] = &world.world.as_bytes()[start_index..=start_index + s.size];

    new_bottom_line.iter().all(|c| *c == '.' as u8)
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
    let mut max_size = 1;
    loop {
        if !is_square_enlargment_valid(world, &Square{ y: coords.0, x: coords.1, size: max_size}) {
            return max_size;
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
pub fn find_biggest_square(world: &World) -> Option<Square> {
    let mut s = Square{y:0, x:0, size:0};
    for (i, l) in world.world.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c != world.empty_char {
                continue;
            }
            let m_s = get_max_size_from_coords(world, (i, j));
            if m_s > s.size {
                s = Square{y: i, x: j, size: m_s};
            }
        }
    }
    if s.size == 0 {
        return None;
    }
    Some(s)
}

pub fn print_world_and_square(mut world: World, square: &Square) {
    let s = std::iter::repeat("x").take(square.size).collect::<String>();
    for i in square.y..square.y + square.size {
        let offset = (world.width + 1) * i + square.x;
        world.world.replace_range(offset..offset + square.size, &s);
    }
    print!("{}", world.world);
}

pub fn print_world(world: &World) {
    print!("{}", world.world);
}

#[cfg(test)]
mod test {
    #[test]
    fn is_square_valid() {
        let world = super::World::new(String::from("3\n..3\n1..\n...")).unwrap();
        assert_eq!(super::is_square_valid(&world, &super::Square::new((0, 0, 2))), false);
        assert_eq!(super::is_square_valid(&world, &super::Square::new((0, 0, 1))), true);
        assert_eq!(super::is_square_valid(&world, &super::Square::new((1, 1, 2))), true);
        assert_eq!(super::is_square_valid(&world, &super::Square::new((1, 0, 1))), false);
        assert_eq!(super::is_square_valid(&world, &super::Square::new((2, 2, 12))), false);
        assert_eq!(super::is_square_valid(&world, &super::Square::new((2, 1, 2))), false);

        let world = super::World::new(String::from("3\n...\n...\n.o.")).unwrap();
        assert_eq!(super::is_square_valid(&world, &super::Square::new((1, 0, 2))), false);
    }

    #[test]
    fn is_square_enlargment_valid() {
        let world = super::World::new(String::from("3\n..3\n1..\n...")).unwrap();
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((0, 0, 1))), false);
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((1, 1, 1))), true);
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((1, 1, 2))), false);
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((2, 2, 12))), false);
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((2, 1, 2))), false);

        let world = super::World::new(String::from("3\n...\n...\n.o.")).unwrap();
        assert_eq!(super::is_square_enlargment_valid(&world, &super::Square::new((0, 1, 1))), true);
    }

    #[test]
    fn get_max_size_from_coords() {
        let world = super::World::new(String::from("3\n..3\n1..\n...")).unwrap();
        assert_eq!(super::get_max_size_from_coords(&world, (0, 0)), 1);
        assert_eq!(super::get_max_size_from_coords(&world, (1, 1)), 2);
        assert_eq!(super::get_max_size_from_coords(&world, (2, 1)), 1);
    }

    #[test]
    fn find_biggest_square() {
        let world = super::World::new(String::from("3\n..3\n1..\n...")).unwrap();
        assert_eq!(super::find_biggest_square(&world), Some(super::Square::new((1, 1, 2))));


        let world = super::World::new(String::from("3\n...\n...\n...")).unwrap();
        assert_eq!(super::find_biggest_square(&world), Some(super::Square::new((0, 0, 3))));

        let world = super::World::new(String::from("5\n.....\n.....\n.....\n.....\n.....\n")).unwrap();
        assert_eq!(super::find_biggest_square(&world), Some(super::Square::new((0, 0, 5))));
    }
}