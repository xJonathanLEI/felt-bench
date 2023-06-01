use std::io::{BufRead, BufReader};

use felt_bench::{BenchmarkResult, RankedBenchmarkResult, Time, BENCHMARKS, IMPLEMENTATIONS};
use regex::Regex;

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
