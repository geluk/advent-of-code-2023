use std::ops::{Add, Mul};

use itertools::Itertools;

use crate::{input::DayInput, Day};

pub struct Day11;
impl Day for Day11 {
    type Input = Vec<Planet>;

    const DAY_NO: usize = 11;

    fn solve_challenge_1(input: &Self::Input) -> u64 {
        const EXPANSION_COEFFICIENT: u64 = 1;

        input
            .iter()
            .copied()
            .tuple_combinations()
            .map(|(a, b)| a.measure_distance(&b, EXPANSION_COEFFICIENT))
            .sum()
    }

    fn solve_challenge_2(input: &Self::Input) -> u64 {
        const EXPANSION_COEFFICIENT: u64 = 999999;

        input
            .iter()
            .copied()
            .tuple_combinations()
            .map(|(a, b)| a.measure_distance(&b, EXPANSION_COEFFICIENT))
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}
impl Point {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn taxicab_distance(&self, other: Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl Mul<u64> for Point {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    position: Point,
    expansion_correction: Point,
}
impl Planet {
    fn new(position: Point, expansion_correction: Point) -> Planet {
        Planet {
            position,
            expansion_correction,
        }
    }

    fn measure_distance(&self, other: &Planet, expansion_coefficient: u64) -> u64 {
        self.corrected_position(expansion_coefficient)
            .taxicab_distance(other.corrected_position(expansion_coefficient))
    }

    fn corrected_position(&self, expansion_coefficient: u64) -> Point {
        let expansion_transform = self.expansion_correction * expansion_coefficient;
        self.position + expansion_transform
    }
}

impl DayInput for Vec<Planet> {
    fn load(input: &'static str) -> Self {
        let mut density = CosmicDensity::from_map(input);
        let mut planets = Vec::with_capacity(256);

        for (y, line) in input.lines().enumerate() {
            density.correct_for_y(y);
            density.reset_x_correction();

            for (x, ch) in line.chars().enumerate() {
                density.correct_for_x(x);

                if ch == '#' {
                    planets.push(Planet::new(
                        Point::new(x as u64, y as u64),
                        density.current_correction_factor(),
                    ));
                }
            }
        }

        planets
    }
}

struct CosmicDensity {
    empty_x: Vec<bool>,
    empty_y: Vec<bool>,
    x_correction: u64,
    y_correction: u64,
}
impl CosmicDensity {
    fn from_map(map: &str) -> CosmicDensity {
        let mut empty_x = Vec::with_capacity(140);
        let mut empty_y = Vec::with_capacity(140);
        for (y, line) in map.lines().enumerate() {
            // Each newly encountered row starts out empty unless proven otherwise.
            empty_y.push(true);
            for (x, ch) in line.chars().enumerate() {
                if y == 0 {
                    // We only need to discover new columns when iterating through the first row.
                    empty_x.push(true);
                }

                if ch == '#' {
                    empty_x[x] = false;
                    empty_y[y] = false;
                }
            }
        }

        Self {
            empty_x,
            empty_y,
            x_correction: 0,
            y_correction: 0,
        }
    }

    fn current_correction_factor(&self) -> Point {
        Point::new(self.x_correction, self.y_correction)
    }

    fn reset_x_correction(&mut self) {
        self.x_correction = 0;
    }

    fn correct_for_y(&mut self, y: usize) {
        if self.empty_y[y] {
            self.y_correction += 1;
        }
    }

    fn correct_for_x(&mut self, x: usize) {
        if self.empty_x[x] {
            self.x_correction += 1;
        }
    }
}
