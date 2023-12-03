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
    solve_day::<Day1>();
    solve_day::<Day2>();
    solve_day::<Day3>();
}

fn solve_day<D: Day>() {
    let input = input::load_day(D::DAY_NO);

    println!("Day {}: ", D::DAY_NO);
    let answer_1 = D::solve_challenge_1(&input);
    println!(" - first answer: {answer_1}");

    let answer_2 = D::solve_challenge_2(&input);
    println!(" - second answer: {answer_2}");
}
