# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [stark_curve](https://github.com/eqlabs/pathfinder) | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) | [lambdaworks-math](https://github.com/lambdaclass/lambdaworks) | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | --------------------------------------------------- | ---------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| `add`        | 2.8881 ns                                           | 2.7005 ns :crown:                                          | 2.7676 ns                                                      | 13.846 ns                                             |
| `add_assign` | 1.5875 ns                                           | 1.4151 ns :crown:                                          | 2.5144 ns                                                      | 16.843 ns                                             |
| `sub`        | 2.7582 ns                                           | 2.8521 ns                                                  | 2.4251 ns :crown:                                              | 15.117 ns                                             |
| `sub_assign` | 1.4572 ns :crown:                                   | 2.4310 ns                                                  | -                                                              | 17.170 ns                                             |
| `mul`        | 13.347 ns                                           | 13.862 ns                                                  | 11.004 ns :crown:                                              | 161.04 ns                                             |
| `mul_assign` | 11.552 ns :crown:                                   | 19.395 ns                                                  | -                                                              | 154.29 ns                                             |
| `invert`     | 3.7260 µs                                           | 1.6670 µs :crown:                                          | 1.7172 µs                                                      | -                                                     |
| `sqrt`       | 1.8468 ms                                           | 106.75 µs :crown:                                          | 230.09 µs                                                      | 4.3102 ms                                             |
| `pow`        | 1.0524 µs :crown:                                   | -                                                          | 1.0598 µs                                                      | 6.1588 µs                                             |

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
