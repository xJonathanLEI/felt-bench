# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) :crown: | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | ----------------------------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 3.1639 ns                                                   | 3.4356 ns                                                  | 30.892 ns                                             |
| `add_assign` | 1.4107 ns                                                   | 2.5347 ns                                                  | 25.833 ns                                             |
| `sub`        | 2.3081 ns                                                   | 8.4291 ns                                                  | 28.645 ns                                             |
| `sub_assign` | 1.3403 ns                                                   | 10.267 ns                                                  | 23.913 ns                                             |
| `mul`        | 12.863 ns                                                   | 16.729 ns                                                  | 203.36 ns                                             |
| `mul_assign` | 11.411 ns                                                   | 19.223 ns                                                  | 173.98 ns                                             |

## Environment

- **CPU**

  _AMD Ryzen 9 5950X 16-Core Processor_

- **OS**

  _Debian GNU/Linux 11 (bullseye)_

## Running benchmarks

To run the benchmarks:

```console
$ cargo bench
```
