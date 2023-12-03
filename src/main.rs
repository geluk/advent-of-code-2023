use std::time::{Duration, Instant};

use input::DayInput;
use solutions::*;

mod input;
mod solutions;

trait Day {
    type Input: DayInput;
    const DAY_NO: usize;

    fn solve_challenge_1(input: &Self::Input) -> u32;
    fn solve_challenge_2(input: &Self::Input) -> u32;
}

fn main() {
    let now = Instant::now();

    solve_day::<Day1>();
    solve_day::<Day2>();
    solve_day::<Day3>();

    let elapsed = now.elapsed().as_micros() as f32 / 1000.0;

    println!("🔥blazing fast🔥: all solutions calculated in {elapsed} ms",)
}

fn solve_day<D: Day>() {
    let (input, _) = measure(|| input::load_day(D::DAY_NO));

    let (answer_1, t_1) = measure(|| D::solve_challenge_1(&input));
    let (answer_2, t_2) = measure(|| D::solve_challenge_2(&input));

    println!("Day {}:", D::DAY_NO);
    println!(" - first answer: {answer_1} (in {}µs)", t_1.as_micros());
    println!(" - second answer: {answer_2} (in {}µs)", t_2.as_micros());
}

fn measure<F, R>(action: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let now = Instant::now();
    let res = action();
    (res, now.elapsed())
}
