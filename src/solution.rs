#![allow(unused_imports, dead_code)]
pub use aoc::*;

pub use nom::branch::alt;
pub use nom::bytes::complete::tag;
pub use nom::character::complete::{alpha1, digit1, one_of};
pub use nom::combinator::{map, map_res};
pub use nom::multi::separated_list1;
pub use nom::sequence::{preceded, terminated, tuple};
pub use nom::IResult;

use colored::Colorize;
pub use std::collections::HashMap;
use std::fmt::write;
pub use std::fmt::Debug;
use std::iter;
pub use std::iter::once;
use std::time::Duration;
pub use std::time::Instant;

pub type Answer = Int;
pub type Year = i32;
pub type Day = u32;
pub type Input = String;
pub type ExampleInput = &'static str;

use colored::*;

/// Use the newtype pattern to implement `From` and `Into` for `Input` and `Vec<String>`. \
/// `InputLines` is only a wrapper for `Vec<String>`.
///
/// See: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct InputLines(Vec<String>);

impl InputLines {
    pub fn filter_empty_lines(self) -> InputLines {
        InputLines(self.0.into_iter().filter(|line| !line.is_empty()).collect())
    }
}

impl Debug for InputLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

/// Make Input convertible to InputLines(Vec<String>) by lines()
impl From<Input> for InputLines {
    fn from(input: Input) -> Self {
        InputLines(input.lines().map(String::from).collect())
    }
}

/// Make InputLines convertible to Vec<String>
impl Into<Vec<String>> for InputLines {
    fn into(self) -> Vec<String> {
        self.0
    }
}

/// Make InputLines convertible to Grid<char>
impl Into<Grid<char>> for InputLines {
    fn into(self) -> Grid<char> {
        self.0.into_iter().map(|s| s.chars().collect()).collect()
    }
}

/// Trim example_input, remove preceding spaces from all lines, remove first \n, keep empty lines intact
fn trim_example_input(input: ExampleInput) -> Input {
    input
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                line // Keep empty lines intact
            } else {
                line.trim_start() // Trim leading spaces from non-empty lines
            }
        })
        .skip(1) // Skip first
        .take(input.lines().count().saturating_sub(2)) // Skip last
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(PartialEq, Eq)]
pub enum TestStatus {
    Failed(Duration, Answer),
    Error(Duration),
    Success(Duration, Answer),
    Unknown,
}

impl Debug for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Failed(duration, answer) => {
                write!(
                    f,
                    "[TestStatus::{}] [{}] {:.2?}",
                    "Failed".red(),
                    answer,
                    duration
                )
            }
            Self::Error(duration) => write!(f, "[TestStatus::{}] {:.2?}", "Error".red(), duration),
            Self::Success(duration, answer) => {
                write!(
                    f,
                    "[TestStatus::{}] [{}] {:.2?}",
                    "Success".green(),
                    answer,
                    duration
                )
            }
            Self::Unknown => write!(f, "[TestStatus::Unknown]"),
        }
    }
}

pub struct TestResult {
    pub year: i32,
    pub day: u32,
    pub p1: TestStatus,
    pub p2: TestStatus,
    pub examples: TestStatus,
}

impl Debug for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestResult {{\n",).unwrap();
        write!(
            f,
            "\t[Ex] [{}] [{}] {:?}\n",
            self.year, self.day, self.examples
        )
        .unwrap();
        write!(f, "\t[P1] [{}] [{}] {:?}\n", self.year, self.day, self.p1).unwrap();
        write!(f, "\t[P2] [{}] [{}] {:?}\n}}", self.year, self.day, self.p2)
    }
}

/// Trait for implementing an Advent of Code problem
pub trait Solution {
    /// Solve AoC(`YEAR`, `DAY`) part one
    fn solve_part_one(&self, input: Input, is_example: bool) -> Answer;

    /// Solve AoC(`YEAR`, `DAY`) part two
    fn solve_part_two(&self, input: Input, is_example: bool) -> Answer;

    fn year(&self) -> Year;

    fn day(&self) -> Day;

    fn expect_part_one(&self) -> Answer;

    fn expect_part_two(&self) -> Answer;

    /// Define Advent of Code examples
    fn define_examples(&self) -> Vec<Example> {
        Vec::new()
    }

    fn run_part_one(&self) -> Answer {
        let input = aoc::get(self.year(), self.day());
        let solution = self.solve_part_one(input, false);

        solution
    }

    fn run_part_two(&self) -> Answer {
        let input = aoc::get(self.year(), self.day());
        let solution = self.solve_part_two(input, false);

        solution
    }

    /// Run all given examples
    fn run_examples(&self) -> bool {
        let format = |part: usize| {
            format!("[Ex] [{}] [{}] [{}]", self.year(), self.day(), part)
        };

        for (i, example) in self.define_examples().iter().enumerate() {
            let input = trim_example_input(example.input);
            match example.expect {
                Expect::PartOne(one) => {
                    test!(one, self.solve_part_one(input, true), format(1));
                }
                Expect::PartTwo(two) => {
                    test!(two, self.solve_part_two(input, true), format(2));
                }
                Expect::PartsOneAndTwo(one, two) => {
                    test!(
                        one,
                        self.solve_part_one(input.clone(), true),
                        format(1)
                    );
                    test!(two, self.solve_part_two(input, true), format(2));
                }
                Expect::Any => (),
            }
        }
        true
    }

    fn run(&self) -> TestResult {
        let mut test_result: TestResult = TestResult {
            day: self.day(),
            year: self.year(),
            p1: TestStatus::Unknown,
            p2: TestStatus::Unknown,
            examples: TestStatus::Unknown,
        };
        let mut instant = Instant::now();
        test_result.examples = match self.run_examples() {
            true => TestStatus::Success(instant.elapsed(), 1),
            false => TestStatus::Failed(instant.elapsed(), 0),
        };

        instant = Instant::now();
        let mut answer = self.run_part_one();
        test_result.p1 = match answer == self.expect_part_one() {
            true => TestStatus::Success(instant.elapsed(), answer),
            false => TestStatus::Failed(instant.elapsed(), answer),
        };

        instant = Instant::now();
        answer = self.run_part_two();
        test_result.p2 = match answer == self.expect_part_two() {
            true => TestStatus::Success(instant.elapsed(), answer),
            false => TestStatus::Failed(instant.elapsed(), answer),
        };

        test_result
    }

    fn create_box() -> Box<Self>
    where
        Self: Sized + Default,
    {
        Box::new(Default::default())
    }
}

/// Trait to allow a type to be parsed from Problem Input
pub trait Parse {
    fn parse(input: Input) -> Self;
}

/// Parse a single number
pub fn parse_num(input: &str) -> IResult<&str, Int> {
    map_res(digit1, str::parse::<Int>)(input)
}

/// Advent of Code ExampleInput expectation for Problem part one, part two, or both
pub enum Expect {
    PartOne(Answer),
    PartTwo(Answer),
    PartsOneAndTwo(Answer, Answer),
    Any,
}

/// Advent of Code ExampleInput and expectation
pub struct Example {
    pub input: ExampleInput,
    pub expect: Expect,
}

impl Example {
    pub fn get_input(&self) -> Input {
        trim_example_input(self.input)
    }
}

/// Define Advent of Code Examples
#[macro_export]
macro_rules! define_examples {
    (
        $(
            (
                $input:expr,
                $expect:expr,
            )
        ),* $(,)?
    ) => {
        fn define_examples(&self) -> Vec<Example> {
            vec![
                $(
                    Example {
                        input: $input,
                        expect: $expect,
                    },
                )*
            ]
        }
    };
}
