use std::{
    fs::{self, File},
    io::{ErrorKind, Read, Write},
    path::PathBuf,
};

use clap::Parser;
use reqwest::{blocking::Client, header::COOKIE};

mod y2022;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
enum Year {
    #[value(id = "2022")]
    Y2022,
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Y2022 => write!(f, "2022"),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
enum Day {
    #[value(id = "1")]
    D1,

    #[value(id = "2")]
    D2,

    #[value(id = "3")]
    D3,

    #[value(id = "4")]
    D4,

    #[value(id = "5")]
    D5,

    #[value(id = "6")]
    D6,

    #[value(id = "7")]
    D7,

    #[value(id = "8")]
    D8,

    #[value(id = "9")]
    D9,

    #[value(id = "10")]
    D10,

    #[value(id = "11")]
    D11,

    #[value(id = "12")]
    D12,

    #[value(id = "13")]
    D13,

    #[value(id = "14")]
    D14,

    #[value(id = "15")]
    D15,

    #[value(id = "16")]
    D16,

    #[value(id = "17")]
    D17,

    #[value(id = "18")]
    D18,

    #[value(id = "19")]
    D19,

    #[value(id = "20")]
    D20,

    #[value(id = "21")]
    D21,

    #[value(id = "22")]
    D22,

    #[value(id = "23")]
    D23,

    #[value(id = "24")]
    D24,

    #[value(id = "25")]
    D25,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D1 => write!(f, "1"),
            Self::D2 => write!(f, "2"),
            Self::D3 => write!(f, "3"),
            Self::D4 => write!(f, "4"),
            Self::D5 => write!(f, "5"),
            Self::D6 => write!(f, "6"),
            Self::D7 => write!(f, "7"),
            Self::D8 => write!(f, "8"),
            Self::D9 => write!(f, "9"),
            Self::D10 => write!(f, "10"),
            Self::D11 => write!(f, "11"),
            Self::D12 => write!(f, "12"),
            Self::D13 => write!(f, "13"),
            Self::D14 => write!(f, "14"),
            Self::D15 => write!(f, "15"),
            Self::D16 => write!(f, "16"),
            Self::D17 => write!(f, "17"),
            Self::D18 => write!(f, "18"),
            Self::D19 => write!(f, "19"),
            Self::D20 => write!(f, "20"),
            Self::D21 => write!(f, "21"),
            Self::D22 => write!(f, "22"),
            Self::D23 => write!(f, "23"),
            Self::D24 => write!(f, "24"),
            Self::D25 => write!(f, "25"),
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
#[repr(u8)]
enum Part {
    #[value(id = "p1")]
    Part1,

    #[value(id = "p2")]
    Part2,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "part 1"),
            Part::Part2 => write!(f, "part 2"),
        }
    }
}

#[derive(clap::Parser)]
struct Args {
    /// The year to solve
    year: Year,

    /// The day to solve
    day: Day,

    /// Which part to solve
    #[arg(value_enum)]
    part: Part,

    /// Example data to use (if left blank, use the actual puzzle input)
    #[arg(short, long)]
    example_data: Option<String>,
}

fn fetch_input(day: Day, year: Year) -> eyre::Result<String> {
    let mut dir = None;
    match fs::metadata("cached_input") {
        Ok(metadata) => {
            if metadata.is_dir() {
                dir = Some(PathBuf::from("cached_input"));
            }
        }
        Err(err) => {
            if let ErrorKind::NotFound = err.kind() {
                if let Ok(()) = std::fs::create_dir("cached_input") {
                    dir = Some(PathBuf::from("cached_input"))
                }
            }
        }
    }

    let mut file = None;
    if let Some(mut path) = dir {
        path.push(format!("y{year}d{day}.txt"));
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    if let Ok(mut opened_file) = File::open(&path) {
                        let mut input = String::new();
                        if let Ok(_) = opened_file.read_to_string(&mut input) {
                            return Ok(input);
                        }
                    }
                }
            }
            Err(err) => {
                if let ErrorKind::NotFound = err.kind() {
                    if let Ok(opened_file) = File::create(&path) {
                        file = Some(opened_file);
                    }
                }
            }
        }
    }

    const SESSION: &str = concat!("session=", include_str!("../session.txt"));
    let client = Client::new();
    let request = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(COOKIE, SESSION)
        .build()?;
    let mut input = client.execute(request)?.text()?;

    while input.ends_with(&['\r', '\n']) {
        let _ = input.pop();
    }

    if let Some(mut file) = file {
        let _ = file.write_all(input.as_bytes());
    }

    Ok(input)
}

fn main() -> eyre::Result<()> {
    let Args {
        day,
        year,
        part,
        example_data,
    } = Args::try_parse()?;

    let input = match example_data {
        Some(example_data) => example_data,
        None => fetch_input(day, year)?,
    };

    let answer: Box<dyn std::fmt::Display> = match (year, day, part) {
        (Year::Y2022, Day::D17, Part::Part1) => Box::new(y2022::d17::p1::solve(&input)?),
        (Year::Y2022, Day::D17, Part::Part2) => Box::new(y2022::d17::p2::solve(&input)?),
        (Year::Y2022, Day::D18, Part::Part1) => Box::new(y2022::d18::p1::solve(&input)?),
        (Year::Y2022, Day::D18, Part::Part2) => Box::new(y2022::d18::p2::solve(&input)?),
        (Year::Y2022, Day::D19, Part::Part1) => Box::new(y2022::d19::p1::solve(&input)?),
        (Year::Y2022, Day::D19, Part::Part2) => Box::new(y2022::d19::p2::solve(&input)?),
        (Year::Y2022, Day::D20, Part::Part1) => Box::new(y2022::d20::p1::solve(&input)?),
        (Year::Y2022, Day::D20, Part::Part2) => Box::new(y2022::d20::p2::solve(&input)?),
        (Year::Y2022, Day::D21, Part::Part1) => Box::new(y2022::d21::p1::solve(&input)?),
        (Year::Y2022, Day::D21, Part::Part2) => Box::new(y2022::d21::p2::solve(&input)?),
        (Year::Y2022, Day::D22, Part::Part1) => Box::new(y2022::d22::p1::solve(&input)?),
        (Year::Y2022, Day::D22, Part::Part2) => Box::new(y2022::d22::p2::solve(&input)?),
        (Year::Y2022, Day::D23, Part::Part1) => Box::new(y2022::d23::p1::solve(&input)?),
        (Year::Y2022, Day::D23, Part::Part2) => Box::new(y2022::d23::p2::solve(&input)?),
        (Year::Y2022, Day::D24, Part::Part1) => Box::new(y2022::d24::p1::solve(&input)?),
        (Year::Y2022, Day::D24, Part::Part2) => Box::new(y2022::d24::p2::solve(&input)?),
        _ => eyre::bail!("There is not yet a solution for that puzzle"),
    };
    println!("The solution for {year} day {day} {part} is {answer}");

    Ok(())
}
