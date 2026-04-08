# Performance Benchmarking

## Overview

The nfw CLI includes a benchmark harness that measures workspace and service creation performance against the <1 second target defined in SC-001. The benchmark validates that the CLI meets performance requirements on baseline hardware (2 CPU cores, 4GB RAM).

## Running Benchmarks

### Prerequisites

- `nfw` CLI built and available in PATH
- Template cache populated
- `make` available

### Run Default Benchmark

```bash
cd src/nfw
make benchmark
```

### Run with Custom Iterations

```bash
./scripts/benchmark/run_benchmark.sh --iterations 20
```

### Run Specific Benchmark

```bash
# Workspace creation only
./scripts/benchmark/run_benchmark.sh --test workspace_creation

# Service creation only
./scripts/benchmark/run_benchmark.sh --test service_creation

# Combined (workspace + service)
./scripts/benchmark/run_benchmark.sh --test combined
```

### Show Help

```bash
./scripts/benchmark/run_benchmark.sh --help
```

## Understanding Results

### Sample Output

```json
{
  "test_name": "workspace_and_service_creation",
  "iterations": 10,
  "results": [
    {"iteration": 1, "duration_ms": 850},
    {"iteration": 2, "duration_ms": 823},
    ...
  ],
  "statistics": {
    "median_ms": 850,
    "p95_ms": 887,
    "min_ms": 823,
    "max_ms": 891,
    "mean_ms": 853
  },
  "target_ms": 1000,
  "passed": true,
  "environment": {
    "cpu_cores": 2,
    "ram_mb": 4096,
    "os": "linux-x86_64",
    "disk_type": "ssd",
    "cli_version": "0.1.0"
  },
  "timestamp": "2026-04-07T14:30:00Z"
}
```

### Interpreting Results

| Field       | Description                               |
| ----------- | ----------------------------------------- |
| `p95_ms`    | 95th percentile duration (the key metric) |
| `target_ms` | Performance target (1000ms for SC-001)    |
| `passed`    | Whether p95_ms <= target_ms               |

- **passed: true** — Performance target met. No action needed.
- **passed: false** — Performance target exceeded. Investigate:
  - Is the hardware equivalent to baseline (2 CPU, 4GB RAM)?
  - Is the system under load during the benchmark?
  - Is disk I/O a bottleneck?

### Statistics Explained

| Statistic   | Description                             |
| ----------- | --------------------------------------- |
| `median_ms` | 50th percentile — typical performance   |
| `p95_ms`    | 95th percentile — accounts for outliers |
| `min_ms`    | Fastest iteration                       |
| `max_ms`    | Slowest iteration                       |
| `mean_ms`   | Arithmetic average                      |

## Performance Target

### SC-001: Workspace + Service Creation

The target is **workspace + service creation in under 1 second** on baseline hardware:

- 2 CPU cores
- 4GB RAM
- SSD storage

The benchmark passes when `p95_ms <= 1000`.

## Environment Metadata

The benchmark captures environment information for context:

- **cpu_cores**: Number of available CPU cores
- **ram_mb**: Available RAM in megabytes
- **os**: Operating system identifier
- **disk_type**: Disk type (ssd, hdd, unknown)
- **cli_version**: Version of the nfw CLI

## CI Integration

Benchmarks run on merge to main only (not on PRs) to avoid noise from variable CI hardware.

Results are:

- Saved to `benchmark-results.json`
- Uploaded as CI artifacts
- Summary posted to PR/commit

See [`.github/workflows/benchmarks.yml`](../../.github/workflows/benchmarks.yml) for configuration.

## Troubleshooting

### Benchmark Fails on Your Machine

1. **Check hardware**: Are you running on baseline hardware (2 CPU, 4GB RAM)?
2. **Check system load**: Close other applications during benchmarking
3. **Check disk**: SSD recommended; HDD will be significantly slower
4. **Run multiple times**: Single runs can be affected by system variance

### Benchmark Passes in CI but Fails Locally

- Your hardware may be slower than baseline
- Your system may have more background processes
- Consider the baseline hardware requirements

### Timing Variance

If results vary significantly between runs:

- Use more iterations: `--iterations 20`
- Check for background processes
- Ensure no other heavy applications are running
- Consider running on a clean system
