use std::{
    cmp::Ordering,
    fmt::Display,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

const BENCHMARKS: [&str; 9] = [
    "add",
    "add_assign",
    "sub",
    "sub_assign",
    "mul",
    "mul_assign",
    "invert",
    "sqrt",
    "pow",
];

const IMPLEMENTATIONS: [Implementation; 4] = [
    Implementation {
        name: "stark_curve",
        source: "eqlabs/pathfinder@5b131c5",
        repo: "https://github.com/eqlabs/pathfinder",
    },
    Implementation {
        name: "starknet-ff",
        source: "xJonathanLEI/starknet-rs@a6cbfa3",
        repo: "https://github.com/xJonathanLEI/starknet-rs",
    },
    Implementation {
        name: "lambdaworks-math",
        source: "lambdaclass/lambdaworks@46dd588",
        repo: "https://github.com/lambdaclass/lambdaworks",
    },
    Implementation {
        name: "cairo-felt",
        source: "lambdaclass/cairo-rs@5db2e65",
        repo: "https://github.com/lambdaclass/cairo-rs",
    },
];

#[derive(Clone, Copy, PartialEq, Eq)]
struct Implementation {
    name: &'static str,
    source: &'static str,
    repo: &'static str,
}

#[derive(Clone)]
struct BenchmarkResult {
    benchmark: &'static str,
    implementation: Implementation,
    result: Time,
}

#[derive(Clone)]
struct RankedBenchmarkResult {
    result: BenchmarkResult,
    is_fastest: bool,
}

#[derive(Clone, PartialEq, Eq)]
struct Time {
    value: String,
    unit: Unit,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Unit {
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.unit.cmp(&other.unit) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => {
                if self.value == other.value {
                    Some(Ordering::Equal)
                } else {
                    Some(
                        f32::from_str(&self.value)
                            .expect("unable to parse value")
                            .total_cmp(
                                &f32::from_str(&other.value).expect("unable to parse value"),
                            ),
                    )
                }
            }

            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl FromStr for Unit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ps" => Ok(Self::Picosecond),
            "ns" => Ok(Self::Nanosecond),
            "µs" => Ok(Self::Microsecond),
            "ms" => Ok(Self::Millisecond),
            "s" => Ok(Self::Second),
            _ => Err("unknown unit"),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Picosecond => write!(f, "ps"),
            Unit::Nanosecond => write!(f, "ns"),
            Unit::Microsecond => write!(f, "µs"),
            Unit::Millisecond => write!(f, "ms"),
            Unit::Second => write!(f, "s"),
        }
    }
}

fn main() {
    // Name your console log output file as `result.log`
    let log_file = std::fs::File::open("./result.log").expect("console log file not found");
    let log_reader = BufReader::new(log_file);

    let mut results = vec![];

    let mut line_iter = log_reader.lines();

    let time_regex = Regex::new(
        r###"time:   \[[\d\.]* [a-zµ]* (?P<median_value>[\d\.]*) (?P<median_unit>[a-zµ]*) [\d\.]* [a-zµ]*\]"###,
    )
    .expect("unable to build regex");

    loop {
        let current_line = match line_iter.next() {
            Some(line) => line.expect("invalid utf-8"),
            None => break,
        };

        for benchmark in BENCHMARKS.into_iter() {
            for implementation in IMPLEMENTATIONS.into_iter() {
                if current_line
                    == format!(
                        "{} | {} - {}",
                        benchmark, implementation.name, implementation.source
                    )
                {
                    let next_line = line_iter
                        .next()
                        .expect("unexpected end of log")
                        .expect("invalid utf-8");

                    let matches = time_regex.captures(&next_line).expect("regex not matched");

                    let median_value = matches
                        .name("median_value")
                        .expect("unable to extract median")
                        .as_str();

                    let median_unit = matches
                        .name("median_unit")
                        .expect("unable to extract median")
                        .as_str();

                    results.push(BenchmarkResult {
                        benchmark,
                        implementation,
                        result: Time {
                            value: median_value.into(),
                            unit: median_unit.parse().expect("unable to parse unit"),
                        },
                    })
                }
            }
        }
    }

    let results = results
        .iter()
        .map(|raw_result| RankedBenchmarkResult {
            result: raw_result.to_owned(),
            is_fastest: results
                .iter()
                .filter(|item| item.benchmark == raw_result.benchmark)
                .min_by_key(|item| &item.result)
                .expect("unable to find fastest item")
                .implementation
                == raw_result.implementation,
        })
        .collect::<Vec<_>>();

    print!("| |");
    for implementation in IMPLEMENTATIONS.into_iter() {
        print!(" [{}]({}) |", implementation.name, implementation.repo);
    }
    println!();

    print!("| - |");
    for _ in IMPLEMENTATIONS.into_iter() {
        print!(" - |");
    }
    println!();

    for benchmark in BENCHMARKS.into_iter() {
        print!("| `{}` |", benchmark);

        for implementation in IMPLEMENTATIONS.into_iter() {
            let result = results.iter().find(|item| {
                item.result.benchmark == benchmark && item.result.implementation == implementation
            });

            print!(
                " {} |",
                match result {
                    Some(result) => format!(
                        "{} {}{}",
                        result.result.result.value,
                        result.result.result.unit,
                        if result.is_fastest { " :crown:" } else { "" }
                    ),
                    None => "-".into(),
                }
            );
        }

        println!();
    }

    println!();
}
