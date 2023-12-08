mod common;
mod input;
mod solutions;

use std::time::{Duration, Instant};

use input::DayInput;
use solutions::*;

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
    solve_day::<Day4>();
    solve_day::<Day5>();
    solve_day::<Day6>();
    solve_day::<Day7>();

    let elapsed = now.elapsed().as_micros() as f32 / 1000.0;

    println!("🔥blazing fast🔥: all solutions calculated in {elapsed} ms",)
}

fn solve_day<D: Day>() {
    let s = calculate_solution::<D>();
    println!("Day {}:", D::DAY_NO);
    println!(
        " - first answer: {} (in {}µs)",
        s.answer_1,
        s.t_1.as_micros()
    );
    println!(
        " - second answer: {} (in {}µs)",
        s.answer_2,
        s.t_2.as_micros()
    );
}

struct Solution {
    answer_1: u32,
    answer_2: u32,
    t_1: Duration,
    t_2: Duration,
}

fn calculate_solution<D: Day>() -> Solution {
    let (input, _) = measure(|| input::load_day(D::DAY_NO));
    let (answer_1, t_1) = measure(|| D::solve_challenge_1(&input));
    let (answer_2, t_2) = measure(|| D::solve_challenge_2(&input));

    Solution {
        answer_1,
        answer_2,
        t_1,
        t_2,
    }
}

fn measure<F, R>(action: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let now = Instant::now();
    let res = action();
    (res, now.elapsed())
}

#[cfg(test)]
mod tests {

    use super::*;

    fn verify_answers<D: Day>(answer_1: u32, answer_2: u32) {
        let solution = calculate_solution::<D>();

        assert_eq!(answer_1, solution.answer_1);
        assert_eq!(answer_2, solution.answer_2);
    }

    #[test]
    pub fn test_day1() {
        verify_answers::<Day1>(57346, 57345);
    }

    #[test]
    pub fn test_day2() {
        verify_answers::<Day2>(2317, 74804);
    }

    #[test]
    pub fn test_day3() {
        verify_answers::<Day3>(535078, 75312571);
    }

    #[test]
    pub fn test_day4() {
        verify_answers::<Day4>(23235, 5920640);
    }

    #[test]
    pub fn test_day5() {
        verify_answers::<Day5>(510109797, 9622622);
    }

    #[test]
    pub fn test_day6() {
        verify_answers::<Day6>(2374848, 39132886);
    }
}
