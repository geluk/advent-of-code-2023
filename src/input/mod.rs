const DAYS: [&str; 7] = [
    include_str!("day01.txt"),
    include_str!("day02.txt"),
    include_str!("day03.txt"),
    include_str!("day04.txt"),
    include_str!("day05.txt"),
    include_str!("day06.txt"),
    include_str!("day07.txt"),
];

pub fn load_day<I: DayInput>(day_no: usize) -> I {
    let day = DAYS[day_no - 1];

    I::load(day)
}

pub trait DayInput {
    fn load(input: &'static str) -> Self;
}

impl DayInput for &str {
    fn load(input: &'static str) -> Self {
        input
    }
}

impl<T: DayInput> DayInput for Vec<T> {
    fn load(input: &'static str) -> Self {
        let mut lines_vec = Vec::with_capacity(1000);
        lines_vec.extend(input.lines().map(T::load));
        lines_vec
    }
}
