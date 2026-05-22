# Data-Definitions

This repository contains the message definitions that are used by the [firmware](http://github.com/aris-space/asteria-firmware/),
the [remote computers](http://github.com/aris-space/asteria-remote-computers/), and [Mission Control](http://github.com/aris-space/asteria-mission-control/).

> [!NOTE]
> This software was developed by the ASTERIA Software team specifically for our purposes at ARIS for EuRoC 2026.
> No guarantees are given, and external contributions are unlikely to be accepted.
> Additionally, this will very likely not be maintained beyond 2026, see other repositories.
>
> Point of contact: ASTERIA Software, e.g. cyrill.bucher@aris-space.ch

In order to use this framework, one should add messages in the various `datapoints/dp-*` crates, update `datatypes`,
and then either use the `messages` crate, or use `core` to generate a sparse message set.

Below follows the architectural documentation that is also available on the wiki:

## Introduction

This documentation serves to explain the communication protocol definition framework between software applications in the whole rocket system. This mainly includes the pipeline for [CAN-FD](https://en.wikipedia.org/wiki/CAN_FD) messages connecting the microcontrollers and the flight/rfs computer, as well as the flight/rfs computer via [MAVLink](https://mavlink.io) to the mission control. Secondarily, this is also used to export the data as CSV for the database.

## Architecture Overview

The architecture consists of 3 layers, firstly a datatype layer, then a datapoint layer and last a user layer. Those can be mostly individually developed and versioned.

The datatype layer provides the data types as rust types, such as a `NormallyClosedValve` state or a `Celsius` temperature. It contains relevant trait implementations and methods for the further layers to build upon.

The datapoint layer consists of multiple crates, one for each "data owner", so every software application such as a board firmware or flight computer. It then defines which messages it exposes, which can include both messages it sends and messages it receives. So e.g. `OxidizerVentValveControl` or `FirmwareInfo`.
These can be individually versioned and any software application only needs to depend on the ones they actually interact with.

The user layer then chooses the datapoints they need and can aggregate them into a complete or sparse message set to receive and send. By complete we mean having all the messages from a given data owner, and by sparse we mean only selecting a handful, which is useful for constrained firmware applications. A pre-defined user exists which includes all messages and does some consistency checking on them.

## Components

### CAN

CAN FD support is used between the avionics firmware and the remote computers. For this the datatypes derive [`bincode 2`](https://docs.rs/bincode/2.0.1/bincode/) encoding and decoding using a fixed size, little-endian integer encoding. This means basically the entire set of rust types is supported, but they aren't necessarily fixed size.

The _datapoint layer_ then defines the CAN IDs, which are carried over from HERMES. They use standard ids only and follow a pattern ordered by priority.

| ID Range      | #   | Message Class                   | Priority    | Kind        | Description                                  |
| ------------- | --- | ------------------------------- | ----------- | ----------- | -------------------------------------------- |
| `0x000–0x0FF` | 256 | **System Management**           | Very High   | Event-based | Global commands like resets.                 |
| `0x100–0x2FF` | 512 | **Event Messages**              | High        | Event-based | Commands and feedback.                       |
| `0x300–0x3FF` | 256 | **Board Status & Heartbeats**   | Medium-High | Periodic    | Presence and status messages for each board. |
| `0x400–0x5FF` | 512 | **Sensor Data**                 | Medium      | Periodic    | Data produced by sensors.                    |
| `0x600–0x6FF` | 256 | **Build Info / Debug Metadata** | Low         | Periodic    | Firmware version and diagnostics.            |
| `0x700–0x7FF` | 256 | **Reserved / Unused**           | Very Low    | —           | Reserved for future extensions or testing.   |

The CAN layer importantly supports _sparse_ encoding/decoding in the user layer, where a subset of the datapoints is chosen. These datapoints are then inherited and the subsystem only recognizes these messages.
This helps in the firmware to keep a slim code base, and give static safety to the hardware filter for received messages, as it is derived from the subset directly.

### MAVLink

MAVLink is used between the remote computers and mission control. We use a custom dialect that we derive from rust code. The derivation is built such that we can depend on the [`Mavio`](https://mavka.gitlab.io/home/projects/mavio/) library to provide the framing, but we have our own module to transport single messages as single UDP datagrams.

The _datatype layer_ requires the types to derive a trait that collects the list of fields this type contains along with name and description. This can be derived recursively, and one can choose to override the name or keep as it is in the child type when including a set of fields.
Other than recursive types, primitives such as `u8` or `f32` are also supported, and arrays of primitives as well `u8[20]`. Additionally, numeric enums can be defined that can then both act as a primitive, or a full field.
In addition to the field information, these types derive the encoding/decoding of the fields.

The _datapoint layer_ defines the messages by annotating the full message enum with IDs. Derived code then maps to the mavio notion of a message, which can be used to encode/decode whole payloads. The message information is also collected as static information.

A new crate `messages` is a _user-level_ crate embedded in the data definition. It includes all datapoint layers and derives the information necessary for mavio to use it a dialect. This full enum can then decode all messages, but to send messages, the subdialects can be used.

The crate `mavlink-hal` contains the various encoding/decoding traits that can be called recursively, and still provide the right field order. This is achieved by separating the 4 alignments (8, 4, 2, 1) into separate information flows.
It also contains the types that hold static information (using rust's `const` evalution) about fields, messages and enums, along with helper functions to be used in the `const` evaluation.

## Infrastructure

This software is provided as a library, and meant to be embedded as a submodule in the firmware, remote computers, mission control and data tools.
It uses the standard `cargo` build tool with some feature flags to prevent certain dependencies if not needed (e.g. support for MavLink or CAN can be modularly enabled).

CI is performed in GitHub Actions and runs `rustfmt`, `clippy` and `cargo test`. For this a workspace is provided for all crates in this repo, which isn't used in production.

## Verification

Verification is performed with code reviews on pull requests, CI to make sure the strong rust type system catches bugs early, and testing.
The main form of testing is manual explorative end-to-end testing, but some automated unit tests test vital encoding decoding components.

It is planned to further test the decoding/encoding using a fuzzer and also compare performance with existing libraries.

## Alternatives

The old HERMES system works mostly fine, but had some limitations:

- messages where types were directly coupled
- couldn't separate the different data owners
- cargo features were awkward to use

`mavspec` also provides derive macros to generate Ad-Hoc MAVLink dialects, but you cannot export them and they don't follow our design philosophy.

One could also think of an even more abstracted layer, where the datapoints are no longer message but instead act more akin to registers, and the updates are automatically broadcast and writes messages can be generated as well.
