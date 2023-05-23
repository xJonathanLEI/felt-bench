# Starknet Field Element Benchmark

Comparison of performance of various Starknet field element type implementations.

## Results

|              | [starknet-ff](https://github.com/xJonathanLEI/starknet-rs) :crown: | [cairo-felt](https://github.com/lambdaclass/cairo-rs) |
| ------------ | ------------------------------------------------------------------ | ----------------------------------------------------- |
| `add`        | 3.5075 ns                                                          | 31.661 ns                                             |
| `add_assign` | 2.7045 ns                                                          | 27.158 ns                                             |
| `sub`        | 8.1949 ns                                                          | 29.316 ns                                             |
| `sub_assign` | 10.804 ns                                                          | 24.897 ns                                             |

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
