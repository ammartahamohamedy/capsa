# Capsa

A container platform for running Linux workloads and managing images and
registries, being built in Rust and Go.

Capsa targets the OCI Runtime, Image, and Distribution specifications, bringing
container execution, image workflows, and a registry together with a CLI and
web application.

> **Early development:** this repository currently implements the Rust CLI
> parser and help foundation. Container execution, bundle loading and
> generation, image management, and the registry/web application are not yet
> implemented. Capsa does not currently claim OCI conformance.

## Platform scope

Linux workloads are intended to execute natively on Linux and inside a Linux
VM on macOS and Windows. Native host integration on macOS and Windows is part
of the platform's scope, including paths, terminal I/O, cancellation, and
network exposure. VM providers and the host/guest interfaces are not yet
selected.

Native Windows containers and non-Linux container backends are outside the
project's scope. The current parser can be explored independently of container
execution or VM setup.

## Architecture

The platform is being developed around the following components:

| Component | Responsibility | Direction |
| --- | --- | --- |
| CLI and Linux runtime | Parse requests, consume OCI bundles, and manage container execution and lifecycle. | Rust |
| Image workflow | Work with image metadata and layers, prepare filesystems, and produce runtime bundles. | Rust |
| Registry | Distribute manifests and blobs through the OCI Distribution HTTP API. | Go |
| Web application | Provide the platform's web interface and management workflows. | Modular monolith with hexagonal modules |

The CLI and image workflow communicate with the registry. Runtime creation
consumes an OCI bundle containing `config.json` and referencing a prepared
root filesystem. The intended inputs include supplied bundles and bundles
prepared from user settings or image defaults.

These responsibilities describe the platform target; they are not a list of
completed features.

## Current CLI foundation

The parser recognizes eight container lifecycle commands:

```text
state  create  start  run  pause  resume  terminate  remove
```

It provides top-level and command-specific help, version output, required-input
checks, and spelling suggestions. Integration tests cover these behaviors,
including output streams and exit codes. A successfully parsed request does
not currently dispatch a runtime operation.

## Try the CLI

Use a Rust toolchain with Rust 2024 edition support and Cargo. From the
repository root:

```sh
cargo run -q -- --help
cargo run -q -- help create
cargo run -q -- --version
```

These commands display the current interface. Bare invocation shows
introductory help on stderr and exits with code `2`; explicit help requests
write to stdout and exit with code `0`.

## Development

Run the existing tests and check formatting:

```sh
cargo test
cargo fmt --check
```

The current source layout is:

```text
src/
  main.rs        Startup and CLI parsing
  cli.rs         Commands and help metadata
  cli/args.rs    Shared and command-specific arguments
tests/
  cli_help.rs    CLI integration tests
```

## Documentation

- [CLI API](docs/cli-api.md): command contracts, help behavior, input modes,
  and lifecycle semantics.
- [Configuration mechanics](docs/configuration.md): OCI schemas, Rust data
  models, supplied and generated bundles, and validation responsibilities.

Specification references:

- [OCI Runtime Specification v1.3.0](https://github.com/opencontainers/runtime-spec/tree/v1.3.0)
- [OCI Image Specification](https://github.com/opencontainers/image-spec)
- [OCI Distribution Specification v1.1.1](https://github.com/opencontainers/distribution-spec/tree/v1.1.1)

## Feedback

Use [GitHub issues](https://github.com/ammartahamohamedy/capsa/issues) for bug
reports and discussions about the documented behavior.
