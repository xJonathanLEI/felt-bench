# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [lambdaworks-math](https://github.com/lambdaclass/lambdaworks) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | --------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 3.4542 ns                                           | 3.3229 ns :crown:                                          | 3.6381 ns                                                      | 18.843 ns                                             |
| `add_assign` | 1.6485 ns                                           | 1.4402 ns :crown:                                          | 2.4716 ns                                                      | 24.376 ns                                             |
| `sub`        | 3.4924 ns                                           | 3.4418 ns                                                  | 2.7616 ns :crown:                                              | 20.667 ns                                             |
| `sub_assign` | 1.6626 ns :crown:                                   | 2.4374 ns                                                  | -                                                              | 22.982 ns                                             |
| `mul`        | 13.685 ns                                           | 16.719 ns                                                  | 11.972 ns :crown:                                              | 179.34 ns                                             |
| `mul_assign` | 12.328 ns :crown:                                   | 19.707 ns                                                  | -                                                              | 189.37 ns                                             |
| `invert`     | 4.2253 µs                                           | 1.7744 µs :crown:                                          | 1.8171 µs                                                      | -                                                     |
| `sqrt`       | 1.8123 ms                                           | 133.30 µs :crown:                                          | 135.79 µs                                                      | 5.7315 ms                                             |
| `pow`        | 1.0756 µs                                           | -                                                          | 753.13 ns :crown:                                              | 7.6336 µs                                             |

## Environment

- **CPU**

  _AMD Ryzen 9 5950X 16-Core Processor_

- **OS**

  _Ubuntu 22.04.1 LTS_

## Running benchmarks

To run the benchmarks:

```console
$ cargo bench
```

Alternatively, pipe all the console output into a file:

```console
$ cargo bench > ./result.log 2>&1
```

and then generate the Markdown table above with:

```console
$ cargo run
```
