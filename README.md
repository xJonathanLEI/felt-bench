# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [lambdaworks-math](https://github.com/lambdaclass/lambdaworks) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | --------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 2.4629 ns                                           | 875.44 ps                                                  | 868.54 ps :crown:                                              | 19.382 ns                                             |
| `add_assign` | 1.6079 ns                                           | 1.1108 ns :crown:                                          | 1.6464 ns                                                      | 23.097 ns                                             |
| `sub`        | 2.2762 ns                                           | 1.5992 ns                                                  | 844.94 ps :crown:                                              | 16.815 ns                                             |
| `sub_assign` | 1.2995 ns :crown:                                   | 1.3808 ns                                                  | -                                                              | 20.515 ns                                             |
| `mul`        | 12.869 ns :crown:                                   | 13.622 ns                                                  | 13.406 ns                                                      | 185.81 ns                                             |
| `mul_assign` | 11.324 ns :crown:                                   | 19.025 ns                                                  | -                                                              | 172.23 ns                                             |
| `invert`     | 4.1449 µs                                           | 1.7087 µs :crown:                                          | 7.7503 µs                                                      | -                                                     |

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
