const DAYS: [&str; 2] = [include_str!("day01.txt"), include_str!("day02.txt")];

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
        input.lines().map(T::load).collect()
    }
}
