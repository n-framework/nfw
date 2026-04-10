# Performance Benchmarking

## Overview

The benchmark harness is planned to measure workspace and service creation performance against the <1 second target defined in SC-001 (SuperClaude-001, a performance requirement for CLI scaffolding tools). This feature is not yet implemented.

See [Spec 005-build-test-workflows](../specs/005-build-test-workflows/) for detailed requirements and success criteria.

## Status

- **Implementation**: Not implemented yet
- **Target**: Measure CLI performance on baseline hardware (2 CPU cores, 4GB RAM)
- **Goal**: Validate that workspace + service creation completes in under 1 second

## Planned Implementation

The benchmark harness will:

1. Measure `nfw new` (workspace creation) timing
2. Measure `nfw add service` (service creation) timing  
3. Run combined benchmark validating SC-001 target (total < 1 second)
4. Report statistics (median, p95, min, max, mean)
5. Output JSON results with environment metadata
6. Integrate with CI for performance regression detection

## Running Benchmarks

Not available yet. Check back later.

## Planned Usage

When implemented, the benchmark will support:

```bash
# Run with default settings
make benchmark

# Custom iterations
./scripts/benchmark/run_benchmark.sh --iterations 20

# Specific test scenarios
./scripts/benchmark/run_benchmark.sh --test workspace_creation
./scripts/benchmark/run_benchmark.sh --test service_creation  
./scripts/benchmark/run_benchmark.sh --test combined
```

## Performance Target

The SC-001 target is **workspace + service creation in under 1 second** on baseline hardware (2 CPU cores, 4GB RAM).

## Troubleshooting

### Benchmark Feature Not Available

The benchmark feature is not yet implemented. See the status section above for current implementation state.

### Performance Target Not Met

When the benchmark is implemented, if it exceeds the 1 second target:

1. **Check hardware**: Ensure you're running on baseline hardware (2 CPU cores, 4GB RAM, SSD)
2. **Check system load**: Close other applications during benchmarking
3. **Check disk I/O**: HDD will be significantly slower than SSD
4. **Run multiple times**: Use more iterations to reduce variance

### Performance Variance

If results vary significantly between runs:

- Use more iterations: `--iterations 20`
- Check for background processes consuming resources
- Ensure no other heavy applications are running
- Consider running on a clean system

## Related Documents

- [Smoke Tests](./SMOKE_TESTS.md) - Available now for CLI workflow validation
- [Spec 005-build-test-workflows](../specs/005-build-test-workflows/) - Detailed benchmark requirements
- [CI/CD Workflows](../../.github/workflows/test-suite.yml) - Currently includes placeholder benchmark job
