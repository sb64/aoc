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
    #[value(id = "17")]
    D17,

    #[value(id = "18")]
    D18,

    #[value(id = "19")]
    D19,

    #[value(id = "20")]
    D20,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::D17 => write!(f, "17"),
            Self::D18 => write!(f, "18"),
            Self::D19 => write!(f, "19"),
            Self::D20 => write!(f, "20"),
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
    const SESSION: &str = concat!("session=", include_str!("../session.txt"));
    let client = Client::new();
    let request = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(COOKIE, SESSION)
        .build()?;
    Ok(client.execute(request)?.text()?)
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

    print!("The solution for {year} day {day} {part} is ");
    match (year, day, part) {
        (Year::Y2022, Day::D17, Part::Part1) => println!("{}", y2022::d17::p1::solve(&input)?),
        (Year::Y2022, Day::D17, Part::Part2) => println!("{}", y2022::d17::p2::solve(&input)?),
        (Year::Y2022, Day::D18, Part::Part1) => println!("{}", y2022::d18::p1::solve(&input)?),
        (Year::Y2022, Day::D18, Part::Part2) => println!("{}", y2022::d18::p2::solve(&input)?),
        (Year::Y2022, Day::D19, Part::Part1) => println!("{}", y2022::d19::p1::solve(&input)?),
        (Year::Y2022, Day::D19, Part::Part2) => println!("{}", y2022::d19::p2::solve(&input)?),
        (Year::Y2022, Day::D20, Part::Part1) => println!("{}", y2022::d20::p1::solve(&input)?),
        (Year::Y2022, Day::D20, Part::Part2) => println!("{}", y2022::d20::p2::solve(&input)?),
        _ => eyre::bail!("There is not yet a solution for that puzzle"),
    }

    Ok(())
}
