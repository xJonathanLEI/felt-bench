use std::{cmp::Ordering, fmt::Display, str::FromStr};

pub const BENCHMARKS: [&str; 9] = [
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

pub const IMPLEMENTATIONS: [Implementation; 4] = [
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

pub const IMPL_STARK_CURVE: Implementation = IMPLEMENTATIONS[0];
pub const IMPL_STARKNET_FF: Implementation = IMPLEMENTATIONS[1];
pub const IMPL_LAMBDAWORKS_MATH: Implementation = IMPLEMENTATIONS[2];
pub const IMPL_CAIRO_FELT: Implementation = IMPLEMENTATIONS[3];

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Implementation {
    pub name: &'static str,
    pub source: &'static str,
    pub repo: &'static str,
}

#[derive(Clone)]
pub struct BenchmarkResult {
    pub benchmark: &'static str,
    pub implementation: Implementation,
    pub result: Time,
}

#[derive(Clone)]
pub struct RankedBenchmarkResult {
    pub result: BenchmarkResult,
    pub is_fastest: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Time {
    pub value: String,
    pub unit: Unit,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
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
