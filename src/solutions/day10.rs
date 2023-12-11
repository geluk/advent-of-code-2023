use std::ops::Add;

use nom::{
    character::complete::{anychar, line_ending},
    combinator::{map, map_opt},
    multi::many1,
    sequence::terminated,
    IResult,
};

use crate::{common, input::DayInput, Day};

pub struct Day10;
impl Day for Day10 {
    type Input = Map;

    const DAY_NO: usize = 10;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        let mut cursor = Cursor::new(input, Direction::Down);
        let props = cursor.find_length().unwrap();
        props.midpoint
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        let mut cursor = Cursor::new(input, Direction::Down);
        let props = cursor.find_length().unwrap();
        props.enclosed_area
    }
}

struct LoopProperties {
    midpoint: u64,
    enclosed_area: u64,
}

struct Cursor<'m> {
    map: &'m Map,
    position: Point,
    orientation: Direction,
    step_count: u64,
    shoelace_area: i64,
}
impl<'m> Cursor<'m> {
    fn new(map: &'m Map, orientation: Direction) -> Self {
        Self {
            map,
            position: map.start,
            orientation,
            step_count: 0,
            shoelace_area: 0,
        }
    }

    fn find_length(&mut self) -> Option<LoopProperties> {
        while self.step() {
            if self.position == self.map.start {
                return Some(self.calculate_properties());
            }
        }
        None
    }

    fn calculate_properties(&self) -> LoopProperties {
        // The length of the pipeline is equal to the number of steps taken to
        // traverse it completely.
        let length = self.step_count as i64;
        // The midpoint lies exactly halfway along the pipeline.
        let midpoint = length / 2;
        // The shoelace formula produces the area of a shape times two.
        // Divide by two to get the actual area.
        let internal_area = self.shoelace_area / 2;
        // The area now calculated represents the area covered by a pipe network
        // where pipes cross through the centre of each tile. In other words,
        // if the pipe makes the simplest loop possible, using only four corner
        // pieces, we get an area of 1, since a quarter of each corner piece
        // falls within the shape.
        // However, the area we are looking for is the total area of all
        // enclosed tiles that aren't pipe sections. This means we should remove
        // the partial tiles areas contributed by tiles with a pipe section
        // going through it.
        // Every straight pipe section contributes 1/2 tile to the total area.
        // This half tile should be left out. The total number of straight pipe
        // sections in our loop multipled by 1/2 therefore gives the area we
        // should remove.
        // This does not account for corners, but it turns out they can almost
        // all be treated the same way. To create a loop going clockwise, we
        // have to take four right turns. If we take a left turn somewhere along
        // the road, we must compensate for it by taking an extra right turn.
        // Vice versa, if, in addition to the four required right turns, we take
        // another right turn, we must compensate with an extra left turn.
        // Going clockwise around the loop, a right turn contributes 1/4 of a
        // tile, while a left turn contributes 3/4 of a tile.
        // Ignoring the initial four right turns, we know each left turn must be
        // paired with a right turn, either pipe section covering 3/4 and 1/4 of
        // a tile respectively. This gives us two pipe sections covering one
        // tile in total, which averages out to 1/2 tile per pipe section.
        // In other words, all corners except our initial four also supply 1/2
        // tile per section!
        let non_corner_pipe_area = (length - 4) / 2;
        // The four remaining corners each contribute 1/4 of a tile each.
        let corner_area = 1;
        // Now we know the total internal area covered by pipe sections.
        let pipe_area = non_corner_pipe_area + corner_area;
        // Subtracting the internal area covered by pipe sections from the total
        // internal area calculated earlier, we get the enclosed area not
        // covered by any pipe sections.
        let enclosed_area = internal_area - pipe_area;

        LoopProperties {
            midpoint: midpoint as u64,
            enclosed_area: enclosed_area as u64,
        }
    }

    fn step(&mut self) -> bool {
        let next_position = self.position + self.orientation;
        match self
            .map
            .tile(next_position)
            .try_enter_from(self.orientation.opposite())
        {
            Some(new_orientation) => {
                // Calculate the area the shape the pipeline is drawing using the
                // shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
                self.shoelace_area += self.position.shoelace(next_position);
                self.orientation = new_orientation;
                self.position = next_position;
                self.step_count += 1;
                true
            }
            None => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Calculates a single step in the shoelace formula.
    fn shoelace(&self, next: Point) -> i64 {
        let (x1, y1) = (self.x as i64, self.y as i64);
        let (x2, y2) = (next.x as i64, next.y as i64);

        (y1 + y2) * (x1 - x2)
    }
}
impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Self::new(self.x, self.y - 1),
            Direction::Right => Self::new(self.x + 1, self.y),
            Direction::Down => Self::new(self.x, self.y + 1),
            Direction::Left => Self::new(self.x - 1, self.y),
        }
    }
}

pub struct Map {
    start: Point,
    width: usize,
    tiles: Vec<Tile>,
}
impl Map {
    fn from_lines(lines: Vec<Vec<Tile>>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut start = Point::new(0, 0);
        let mut tiles = Vec::with_capacity(height * width);
        for (y, line) in lines.into_iter().enumerate() {
            for (x, tile) in line.into_iter().enumerate() {
                tiles.push(tile);
                if tile == Tile::Start {
                    start = Point::new(x, y);
                }
            }
        }

        Self {
            start,
            width,
            tiles,
        }
    }

    fn tile(&self, point: Point) -> Tile {
        self.tiles[point.y * self.width + point.x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Start,
    Empty,
    Pipe(Pipe),
}
impl Tile {
    fn try_enter_from(&self, entry_point: Direction) -> Option<Direction> {
        match self {
            Tile::Start => Some(entry_point.opposite()),
            Tile::Empty => None,
            Tile::Pipe(pipe) => {
                let new_corner = match pipe.corners() {
                    [x, y] if x == entry_point => y,
                    [x, y] if y == entry_point => x,
                    _ => return None,
                };
                Some(new_corner)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    UD, // up <> down
    LR, // left <> right
    UR, // up <> right
    RD, // right <> down
    DL, // down <> left
    LU, // left <> up
}
impl Pipe {
    fn corners(&self) -> [Direction; 2] {
        match self {
            Pipe::UD => [Direction::Up, Direction::Down],
            Pipe::LR => [Direction::Left, Direction::Right],
            Pipe::UR => [Direction::Up, Direction::Right],
            Pipe::RD => [Direction::Right, Direction::Down],
            Pipe::DL => [Direction::Down, Direction::Left],
            Pipe::LU => [Direction::Left, Direction::Up],
        }
    }
}

impl DayInput for Map {
    fn load(input: &'static str) -> Self {
        common::parse(tiles, input)
    }
}

fn tiles(i: &str) -> IResult<&str, Map> {
    map(many1(terminated(many1(tile), line_ending)), Map::from_lines)(i)
}

fn tile(i: &str) -> IResult<&str, Tile> {
    map_opt(anychar, |c| match c {
        'S' => Some(Tile::Start),
        '.' => Some(Tile::Empty),
        '|' => Some(Tile::Pipe(Pipe::UD)),
        '-' => Some(Tile::Pipe(Pipe::LR)),
        'L' => Some(Tile::Pipe(Pipe::UR)),
        'F' => Some(Tile::Pipe(Pipe::RD)),
        '7' => Some(Tile::Pipe(Pipe::DL)),
        'J' => Some(Tile::Pipe(Pipe::LU)),
        _ => None,
    })(i)
}
