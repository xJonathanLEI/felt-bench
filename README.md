# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [lambdaworks-math](https://github.com/lambdaclass/lambdaworks) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | --------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 2.4747 ns                                           | 877.72 ps                                                  | 847.71 ps :crown:                                              | 18.514 ns                                             |
| `add_assign` | 1.5418 ns                                           | 1.0736 ns :crown:                                          | 1.6910 ns                                                      | 22.489 ns                                             |
| `sub`        | 2.2956 ns                                           | 1.7650 ns                                                  | 869.94 ps :crown:                                              | 17.627 ns                                             |
| `sub_assign` | 1.2958 ns :crown:                                   | 1.3825 ns                                                  | -                                                              | 20.443 ns                                             |
| `mul`        | 13.252 ns                                           | 13.982 ns                                                  | 871.34 ps :crown:                                              | 191.29 ns                                             |
| `mul_assign` | 11.673 ns :crown:                                   | 19.648 ns                                                  | -                                                              | 180.07 ns                                             |
| `invert`     | 4.0473 µs                                           | 1.6647 µs :crown:                                          | 7.5690 µs                                                      | -                                                     |

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
