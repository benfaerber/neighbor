
[![Tests](https://github.com/benfaerber/neighbor/actions/workflows/test.yml/badge.svg)](https://github.com/benfaerber/neighbor/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/benfaerber/neighbor/branch/master/graph/badge.svg)](https://codecov.io/gh/benfaerber/neighbor)

## Ben Faerber Take-Home Challenge
Here is my submission to the take-home challenge.
This is a Rust `actix-web` service featuring an API, test suite and criterion benchmark.

### Getting Started:
- `cargo run` - Run the API on port 8080
- `cargo test` - Run the test suite
- `cargo bench` - Run the criterion benchmarks. 
- `ENDPOINT="http://127.0.0.1:8080/search" ./scripts/test_api.sh` - Test the local API using CURL

### Features:
- Unit Testing with `cargo test`
- Benchmarking with `criterion`
- Code Coverage with `codecov.io`
- CI/CD pipeline with Github Actions

### Benchmark Results:
```
api_search/readme_example
                        time:   [1.4652 ms 1.4661 ms 1.4672 ms]
                        change: [-1.6017% -1.3524% -1.1324%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
```

The old [PROMPT](./PROMPT.md) can be found here 
