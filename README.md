# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) :crown: | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [lambdaworks-math](https://github.com/lambdaclass/lambdaworks) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | ----------------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 3.1055 ns                                                   | 3.4377 ns                                                  | 8.9938 ns                                                      | 30.547 ns                                             |
| `add_assign` | 1.3846 ns                                                   | 2.4741 ns                                                  | 9.9680 ns                                                      | 25.851 ns                                             |
| `sub`        | 2.2578 ns                                                   | 8.1349 ns                                                  | 7.5174 ns                                                      | 27.516 ns                                             |
| `sub_assign` | 1.3335 ns                                                   | 10.207 ns                                                  | -                                                              | 23.835 ns                                             |
| `mul`        | 12.807 ns                                                   | 16.439 ns                                                  | 18.788 ns                                                      | 196.49 ns                                             |
| `mul_assign` | 11.334 ns                                                   | 19.647 ns                                                  | -                                                              | 180.26 ns                                             |
| `invert`     | 4.1439 µs                                                   | 1.9090 µs                                                  | 9.4014 µs                                                      | -                                                     |

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
