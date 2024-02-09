pub fn solve(input: &str) -> (u32, u32) {
    (part_1(input), 0)
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let [a @ 'A'..='C', ' ', b @ 'X'..='Z'] = line.chars().collect::<Vec<_>>()[..] else {
                panic!("invalid line format");
            };

            (a.into(), b.into())
        })
        .map(|(a, b)| play(a, b))
        .sum()
}

fn play(a: Shape, b: Shape) -> u32 {
    let outcome = match (a, b) {
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Loss,
        (Shape::Paper, Shape::Rock) => Outcome::Loss,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Loss,
        _ => Outcome::Draw,
    };

    b.points() + outcome.points()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn points(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn points(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("invalid shape symbol '{}'", value),
        }
    }
}
