# Capsa CLI API

## Platform and specification scope

Capsa is a container platform targeting full implementation of the OCI
Runtime, Image, and Distribution specifications for Linux containers.
[OCI specifications overview](https://opencontainers.org/about/overview/).

Linux workloads execute natively on Linux and inside a Linux VM on macOS and
Windows, with native host integration on those platforms. Native Windows
containers and non-Linux container backends are outside Capsa's platform scope.

The commands defined here form the container lifecycle portion of the
platform-wide CLI. Image workflows and registry interfaces also belong to
Capsa's overall scope; this document covers the container lifecycle contract.
The runtime operation and configuration references below remain pinned to
OCI Runtime Specification v1.3.0.

## Input and responsibility boundaries

See [Configuration mechanics](configuration.md) for the schema/model/document
distinction, configuration flow diagram, loading and validation responsibilities,
and the generated-bundle serialization boundary.

The runtime consumes an OCI bundle with `config.json` at its root and a prepared
filesystem identified by `root.path`. On Linux, that directory must exist; its
path may be absolute or relative to the bundle. An empty mountpoint alone does
not supply a workload filesystem.
[Bundle](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/bundle.md),
[Root](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/config.md#root).

A prepared rootfs is needed, and configuration reaches the runtime through the
bundle. Configuration generation does not acquire or populate that filesystem;
image acquisition and filesystem preparation are separate responsibilities
within the wider platform.

Capsa's CLI can accept an existing bundle or prepare one from settings, workload
arguments, and a prepared rootfs. These are two input paths to the same lifecycle
operations, not additional lifecycle commands.

Image-based workflows feed the generated-bundle path by preparing the rootfs
and supplying image defaults. The relationship is:

| Input path | API usage | Preparation before runtime creation |
| --- | --- | --- |
| User-supplied bundle | `create <container-id> <bundle-path>`, then `start <container-id>`; or `run <container-id> <bundle-path>` for a new instance. | Read and validate its existing configuration and filesystem references. |
| Settings + prepared rootfs[^bundle-generation] | Invoke `create` with a container ID, settings, workload arguments, and rootfs input; use `run` to compose creation and start. | Generate the runtime configuration and assemble the bundle. |
| Image-based workflow | The image workflow supplies its generated bundle to `create`; `start` executes the prepared workload, or `run` composes both operations. | Prepare the filesystem from the image, combine image defaults with user settings, and assemble the bundle. |

For generated and image-based input, the API usage column describes operation
flow rather than prescribing CLI flag spellings.

This provides flexibility and interoperability: users can bring bundles
prepared by other tools, exercise precise runtime configurations, or run a
filesystem without first packaging it as an image.

Accepting bundles implements the standard OCI runtime input boundary.
Generating them from convenient CLI inputs adds Capsa's user-facing workflow.
Exposing both through one CLI preserves those choices while keeping the same
underlying lifecycle operations. Every path converges on a validated OCI bundle
before runtime creation.
[OCI filesystem bundle](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/bundle.md),
[Image-to-runtime workflow](https://opencontainers.org/about/overview/).

For generated input, the CLI validates settings, applies documented Capsa
defaults, builds a structured configuration, and serializes `config.json`.
The completed bundle is then passed to runtime `create`. For supplied input,
`create` reads the existing configuration; it does not generate or silently
rewrite it. The supplied configuration is authoritative for that instance.

For generated input, bundle preparation is the first phase of the human-facing
`create` invocation. The OCI create operation consumes the prepared bundle.
Mounting the configured rootfs belongs to environment preparation after this
input boundary.
[Runtime caller](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/glossary.md#runtime-caller),
[Lifecycle](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#lifecycle).

### Bundle-generation input model

> **API expansion under design**
>
> Bundle generation exposes a deliberately selected set of runtime settings
> through CLI inputs. Capsa completes the configuration using documented
> defaults and policies. This establishes the input model for expanding
> `create` and the creation path of `run`.
>
> | Category | User input | Capsa's responsibility |
> | --- | --- | --- |
> | Required inputs | Supply values the selected workflow cannot derive or default appropriately. | Validate and use those values to generate the configuration. |
> | Optional inputs | Supply an override when desired. | Use the documented default when the input is omitted. |
> | Internal configuration | No CLI input is exposed. | Select the configuration through documented defaults and policies. |
>
> Requiredness depends on the workflow. A prepared-rootfs workflow needs a
> filesystem location; an image workflow obtains the filesystem from the
> image. An image may also supply a default command. A property required in
> the resulting `config.json` therefore need not be a required CLI argument.
>
> The specific exposed properties, their required or optional classification,
> flag spellings, default values, and argument relationships are to be decided
> before defining the generation parser. No fixed number of inputs is implied.
> Generation inputs belong to bundle preparation; supplied bundles retain
> their own authoritative configuration.

The [generated-bundle workflow](configuration.md#generated-bundles) explains
how those inputs become Rust values and the required `config.json` artifact.

## Public operations

The container lifecycle API consists of the following
operations. Signatures describe operation inputs; Clap spelling is a separate
CLI concern.

| Operation | Purpose | Basis |
| --- | --- | --- |
| `state <container-id>` | Inspect an existing execution instance. | [OCI Query State](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#query-state) |
| `create <container-id> <path-to-bundle>` | Prepare an instance from the bundle. | [OCI Create](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#create) |
| `start <container-id>` | Execute the prepared workload. | [OCI Start](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#start) |
| `run <container-id>` with bundle input when creating | Select `start` or `create` + `start` from instance lookup. | Capsa composition |
| `pause <container-id>` / `resume <container-id>` | Suspend and continue the same live workload. | Capsa extensions |
| `terminate <container-id>` | Request shutdown, wait, and escalate if needed. | Capsa composition over [OCI Kill](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#kill) |
| `remove <container-id>` | Check eligibility and delete a stopped instance. | Capsa composition over [OCI Delete](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#delete) |

`kill` and `delete` are underlying operation semantics used by these compositions;
this contract does not add them as separate public CLI commands.

Creation cannot reuse an existing instance's ID. Removal makes that ID available
for a new instance; reusing retained storage while the old instance still exists
requires a distinct ID.
[Create/Delete](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#create).

## Help and usage

Help describes the CLI at two levels. Top-level help gives a short summary of
each command, while individual command help explains its inputs and behavior
and includes practical examples. This applies to `state`, `create`, `start`,
`run`, `pause`, `resume`, `terminate`, and `remove`.

| Invocation | Behavior | Output | Exit code |
| --- | --- | --- | --- |
| `capsa` | Display introductory help with the command list, existing flags, an example, and a hint for full help. | stderr | `2` |
| `capsa -h`, `capsa --help`, or `capsa help` | Display full top-level help, including examples and the documentation/feedback link. | stdout | `0` |
| `capsa <command> -h` or `capsa <command> --help` | Display full help for the selected command. | stdout | `0` |
| `capsa help <command>` | Display the same full help for the selected command. | stdout | `0` |
| Missing required input or invalid syntax | Explain the error, show relevant usage, and provide a help hint. | stderr | `2` |
| Misspelled command or option | Report invalid input and suggest a likely correction when available, with a help hint. | stderr | `2` |

Bare `capsa` is treated as missing a required command, so its help is written
to stderr with a usage-error exit code. Explicit help requests are successful
queries and write to stdout. A command's required operational arguments are
not needed to ask for its help: `capsa create --help`, for example, needs
neither a container ID nor a bundle path. Help requests do not execute lifecycle
operations or prompt for confirmation.

Spelling suggestions never execute a corrected command. For example,
`capsa startt demo` reports the invalid command and suggests `start`; the user
must issue the corrected invocation. Container IDs, filesystem paths, and
workload arguments are user data rather than command names to autocorrect.

These behaviors follow the [CLI Guidelines on help](https://clig.dev/#help).
The exit codes above describe CLI help and syntax handling; runtime operation
and workload results are separate concerns.

Clap's [Parser::parse](https://docs.rs/clap/4.6.7/clap/trait.Parser.html#method.parse)
handles parsing, help, usage errors, and process exits. The top-level
[arg_required_else_help](https://docs.rs/clap/4.6.7/clap/struct.Command.html#method.arg_required_else_help)
setting supplies missing-command help, while
[ArgAction::HelpLong](https://docs.rs/clap/4.6.7/clap/enum.ArgAction.html#variant.HelpLong)
makes both `-h` and `--help` request full help. Command and argument doc comments
provide brief descriptions; `long_about`, `after_help`, and `after_long_help`
provide detailed explanations and examples.
[Clap help metadata](https://docs.rs/clap/4.6.7/clap/_derive/index.html#command-attributes).

## State, create, and start

The lifecycle statuses are `creating`, `created`, `running`, and `stopped`.
An unknown ID is a lookup error, not an `absent` status. `created` has a live
container process before workload execution; `running` has executed it;
`stopped` means that process exited. Linux state includes its PID when created
or running. Suspension is tracked separately from lifecycle status.
[State](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#state).

`create` applies non-process configuration without executing `process.args`;
other process settings may be applied then. Later configuration edits cannot
change the instance. `start` requires `created` and process configuration.
[Create](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#create),
[Start](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#start).

Capsa's state output should make instance identity, lifecycle status, and pause
condition unambiguous. A numeric PID is not the authoritative instance lookup.

## Run

`run` is Capsa convenience behavior. It makes its choice from instance lookup
and lifecycle state, never from PID existence alone.

| Lookup result | Action |
| --- | --- |
| No instance with this ID | Require bundle input; call `create`, then `start`. |
| Existing `created` instance | Call `start` using its frozen configuration. |
| Existing `creating`, `running`, or `stopped` instance | Return an error without relaunching or replacing it. |
| Any lookup failure other than instance-not-found | Return that error; do not attempt creation. |

A paused instance is already running for this contract, so `run` does not resume
it. Use `resume`. `run` does not modify a prepared instance's configuration,
implicitly remove an old instance, or add its own hook executions.

## Hook phases

Configured hooks belong to their primitive lifecycle phase, including when
`run` composes those phases:

| Phase | Hook sequence |
| --- | --- |
| Create | `prestart` (deprecated), `createRuntime`, `createContainer` |
| Start | `startContainer`, execute workload, `poststart` |
| Delete through remove | Resource destruction, `poststop` |

Hooks in each array retain their configured order and receive state on stdin.
The configuration reference defines their execution namespaces and exact
timing. Resuming is not a second start-hook phase. `poststop` belongs to
deletion, rather than merely observing workload exit.
[OCI hooks](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/config.md#posix-platform-hooks),
[Hook summary](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/config.md#summary).

## Pause and resume

`pause` suspends a running workload's processes and threads while retaining its
memory, descriptors, and execution environment. `resume` continues that same
workload. Neither creates a new process or restores a terminated one.

The intended Linux mechanism is cgroup freezing. Completion must reflect the
observed freeze/thaw result; requesting a freeze is not proof it completed.
Host time and external clients continue while the workload is paused.
[Linux cgroup freezer](https://docs.kernel.org/admin-guide/cgroup-v2.html#core-interface-files).

## Terminate

Capsa termination has the following behavior:

1. Obtain CLI confirmation before sending signals, unless explicitly bypassed.
2. Request shutdown with SIGTERM.
3. Wait for exit until the grace deadline.
4. Send SIGKILL if the workload is still alive after that deadline.
5. Observe termination, collect its result, and ensure descendants do not survive.

A paused workload needs to be allowed to execute its shutdown handler for the
graceful phase. Successful signal delivery is not successful termination. Exit
results and the `stopped` lifecycle status remain separate observations.

Termination leaves the instance and retained storage available for inspection.
Reusing storage requires a new execution, not resuming the exited process.
The stop signal's forwarding and process-tree ownership follow the supervision
contract when that mechanism is studied.

The CLI confirmation explains escalation and storage retention, and offers a
default No plus an explicit bypass flag. Runtime operations never prompt.
The bypass flag only bypasses confirmation, not validation or the grace period.
Its exact spelling and the timeout default are not fixed here.

Target prompt:

> Terminate container <id>? Capsa will send SIGTERM, wait up to <timeout>, then
> send SIGKILL if necessary. After exit, this workload cannot be resumed.
> Container storage will remain and can be reused for a new execution with
> create + start, or Capsa run. This launches a fresh workload; it does not
> restore the terminated process's memory. [y/N]

Storage reuse depends on releasing conflicting live references before preparing
the new execution. Storage selection and ownership must support this promise
before it is presented by an implemented CLI.

## Remove

`remove` checks state and invokes deletion only for a stopped instance. Deletion
must validate eligibility and the same instance identity within a serialized
transition; the initial query does not reserve an instance.

Cleanup releases remaining instance-owned resources, verifying that they are no
longer in use. It does not imply that every namespace or mount still exists.
External rootfs data and shared image resources are not incidental deletion
targets. The ownership of runtime-created writable layers must be defined
before their removal is implemented; this contract does not promise that such
layers survive `remove`.

## Validation and errors

The detailed [configuration validation responsibilities](configuration.md#loading-and-validation-responsibilities)
apply to both input paths; parsing into Rust values alone does not establish
that an input can be applied by the runtime.

Both generated and user-supplied bundles go through the same input validation:

- Check layout, file readability, JSON syntax, UTF-8 encoding, absence of
  duplicate JSON member names, and version compatibility.
- Use the version-pinned JSON Schema as a structural aid, then apply the
  canonical specification's semantic and Linux-specific requirements.
- Resolve paths using each field's specified base, and check required resources.
- Check the applicable configuration requirements and the implemented runtime
  capabilities and target Linux facilities. Accepting a JSON field is not
  evidence that its requested behavior can be provided.
- Apply field-specific error or warning rules; do not silently discard requested
  isolation or limits, or replace those rules with a universal rejection policy.

[Configuration](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/config.md),
[Linux configuration](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/config-linux.md),
[Config schema](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/schema/config-schema.json),
[JSON rules](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/glossary.md#json),
[Specification conventions](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/spec.md#notational-conventions).

Ordinary operation errors restore the prior environment unless a clause
specifies otherwise. Create/start hook failures require stopping and destruction;
poststop failure warns and continues.
[Errors](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#errors),
[Warnings](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#warnings),
[Lifecycle](https://github.com/opencontainers/runtime-spec/blob/v1.3.0/runtime.md#lifecycle).

Parsing captures CLI inputs. Bundle loading and conversion to validated runtime
configuration are separate responsibilities.

[^bundle-generation]: See [Bundle-generation input model](#bundle-generation-input-model) for the required inputs, optional overrides, and internally managed defaults.
