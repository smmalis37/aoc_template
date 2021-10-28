use crate::{day_solver::DaySolver, util::*};

pub struct DayNUM;

impl<'a> DaySolver<'a> for DayNUM {
    type Parsed = u32;
    type Output = u32;

    fn parse(input: &'a str) -> Self::Parsed {
        todo!()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        todo!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dNUMp1() {
        assert_eq!(DayNUM::part1(DayNUM::parse("")), 0);
    }

    #[test]
    fn dNUMp2() {
        assert_eq!(DayNUM::part2(DayNUM::parse("")), 0);
    }
}
