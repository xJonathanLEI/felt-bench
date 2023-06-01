use std::io::{BufRead, BufReader};

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

struct BenchmarkResult {
    benchmark: &'static str,
    implementation: Implementation,
    result: String,
}

fn main() {
    // Name your console log output file as `result.log`
    let log_file = std::fs::File::open("./result.log").expect("console log file not found");
    let log_reader = BufReader::new(log_file);

    let mut all_results = vec![];

    let mut line_iter = log_reader.lines();

    let time_regex = Regex::new(
        r###"time:   \[[\d\.]* [a-zµ]* (?P<median>[\d\.]* [a-zµ]*) [\d\.]* [a-zµ]*\]"###,
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

                    let median = time_regex
                        .captures(&next_line)
                        .expect("regex not matched")
                        .name("median")
                        .expect("unable to extract median")
                        .as_str();

                    all_results.push(BenchmarkResult {
                        benchmark,
                        implementation,
                        result: median.into(),
                    })
                }
            }
        }
    }

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
            let result = all_results
                .iter()
                .find(|item| item.benchmark == benchmark && item.implementation == implementation);

            print!(
                " {} |",
                match result {
                    Some(result) => &result.result,
                    None => "-",
                }
            );
        }

        println!();
    }

    println!();
}
