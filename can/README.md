# Deprecated ⚠️

Note that this library is deprecated and replace by the new systemf of datapoints and `can-hal`.

This still exists in the repository as the recovery board, sensor carrier and power board retain binary compatiblity in the `0.3.*` version. This is checked using some unit tests, for which this library is used.

# HERMES CAN library

The HERMES CAN library provides a framework for defining and encoding messages for the HERMES CAN bus.
HERMES uses a CAN bus for communication between the onboard electronics and the flight computer.

The CAN bus connects the following onboard systems:
- Flight Computer (FC)
- RFS Computer (RFSC)
- Sensor Carrier Board
- Communication Board
- Power Board
- Recovery Board
- Fuel Control Board
- Oxidizer Control Board
- Engine Control Board

## Versioning

This crate follows a practical [SemVer](https://semver.org/) versioning scheme `major.minor.patch`, with the following rules:

- **Patch `0.x.(y+1)`:**
   - **Allowed:**
      - Bug fixes.
      - Adding new helper functions or convenience methods that do not change the semantics, layout, or bit-level serialization/deserialization of CAN messages.
      - Small performance or ergonomics improvements that do not affect semantics.
      - Documentation improvements.
      - Adding new CAN messages with IDs distinct from existing messages.
   - **Forbidden:**
      - Changing existing CAN message layouts, IDs, field meanings, or units.
      - Adding, removing or renaming fields in existing can messages.

- **Minor `0.(x+1).0`:**
   - May introduce **any** change, including:
      - Changing CAN message layouts, IDs, or field meanings.
      - Adding new fields to existing messages.
      - Adding or changing APIs (beyond helper functions).
      - Breaking changes to existing functionality.
   - Minor updates **may** include breaking changes.

- **Breaking changes:**
   - May be included in any new minor version `0.(x+1).0`.

Please ensure that any breaking changes are kept to a minimum and avoided where possible.

Seeing as this library may be used at different patch versions across the system at the same time, it is important that errors pertaining to unknown IDs and unhandled messages are handled gracefully.

### Release Process
Any new features or updates must be added in a Pull Request. These changes must be reviewed and approved by another person before being merged into the `main` branch.

Once merged, the following steps must be taken to release a new version (requires `cargo-release`):

```shell
# For a patch release (see above for rules)
cargo release patch --no-publish

# For a minor release (see above for rules)
cargo release minor --no-publish
```

This will bump the version in `Cargo.toml` and create a new Git tag `vX.Y.Z` with the updated version number. The resulting changes must then be pushed to the `main` branch.
From this point on, consumers of the library may depend on the updated tag.
See the [Using the Library](#using-the-library) section for more details on how to depend on this library.

## Messages & Priorities

The 11-bit CAN ID space (`0x000–0x7FF`, or 0–2047) is partitioned by **priority**, **function**, and **emission mode**. Lower CAN IDs have higher priority on the bus.
Each block of 256 IDs is reserved for a specific category of messages.

Messages are either:
- **Event-based**: Sent asynchronously in response to events, usually aperiodically. Used for coordination between boards.
- **Periodic**: Sent regularly at some interval. Used for monitoring, telemetry, or diagnostics. Losing a periodic message is less critical than losing an event-based message, so they have lower priority.

## CAN ID Allocation Table

| ID Range      | #   | Message Class                   | Priority    | Type (usually) | Description                                                   |
|---------------|-----|---------------------------------|-------------|----------------|---------------------------------------------------------------|
| `0x000–0x0FF` | 256 | **System Management**           | Very High   | Event-based    | Global commands like resets, aborts, or watchdog triggers.    |
| `0x100–0x2FF` | 512 | **Event Messages**              | High        | Event-based    | Inter-board coordination signals or other event messages.     |
| `0x300–0x3FF` | 256 | **Board Status & Heartbeats**   | Medium-High | Periodic       | Health monitoring and presence confirmation for each board.   |
| `0x400–0x5FF` | 512 | **Sensor Data**                 | Medium      | Periodic       | Data produced by sensors, including raw and processed values. |
| `0x600–0x6FF` | 256 | **Build Info / Debug Metadata** | Low         | Periodic       | Firmware version, debug flags, and similar diagnostics.       |
| `0x700–0x7FF` | 256 | **Reserved / Unused**           | Very Low    | —              | Reserved for future extensions or testing.                    |

## Documentation Format

Each message definition must be cleanly documented in the following format:

```
/// # `<MessageName>`
/// One-line description of what this message represents.
///
/// ## Type
/// - `Periodic @ <rate>`: emitted at a fixed interval
/// - OR `Event`: triggered by `<condition or system event>`
///
/// ## Direction
/// - **Emitted by:** `<Board or Component>`
/// - **Received by:** `<Target(s)>` or `unspecified`
///
/// ## Purpose
/// Describes the role of the message in the system (e.g. telemetry, coordination, control).
///
/// ## Reaction
/// Describes how receivers should respond. This can include state updates, triggering actions, logging, or no required action.
```

## Adding a New Message

To add a new message to the system, follow these steps:

1. **Pick the correct module**
   Based on the message category and ID range (e.g. `event_messages.rs`, `sensor_data.rs`), put your message definition in the appropriate module. See the [CAN ID Allocation Table](#can-id-allocation-table) for guidance.

2. **Define the message struct**
   Define a message struct as you would any other Rust struct. You can use any Rust types that implement `bincode::Encode` and `bincode::Decode`. This includes most Rust primitives, arrays, and tuples.

   If a field does not implement `Encode`/`Decode` but does implement `serde`, you can serialize it using `#[bincode(with_serde)]`:
   ```rust
   #[derive(Decode, Encode)]
   pub struct StructWithSerde {
      #[bincode(with_serde)]
      pub only_supports_serde: SerdeType,
   }
   ```
   Make sure the total encoded size is ≤ 64 bytes (this is automatically checked during fuzzing). Avoid variable-length messages if possible.

   Messages must be defined as `pub`, otherwise you won't see them when depending on this crate. Make sure your message has at least the following derive macros:
   ```rust
   #[derive(Clone, Debug, PartialEq)]
   #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
   #[cfg_attr(feature = "defmt", derive(defmt::Format))]
   #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
   ```
   These are used for fuzz testing, logging, and serialization. If they’re missing, the code won't compile and CI will fail. If you use the `can_message` attribute macro, **these are added automatically!**

3. **Assign a CAN ID** and implement required traits
   Use the allocation table to pick a valid and unique ID. CI will catch mistakes, but you must still ensure the ID is in the correct range and priority class.

    - If defining a family of similar messages (e.g. `BuildDataX`, `BuildDataY`, etc.), their IDs should be **consecutive** and assigned in the order the boards are listed:
      `[FlightComputer, RFSComputer, SensorCarrierBoard, CommunicationBoard, EngineControlUnit, PowerBoard, RecoveryBoard, PressureControlBoard1, PressureControlBoard2]`

    - Otherwise, generally try to **space out** the IDs. Look at existing messages for examples. This helps leave room for new messages with different priorities later.

   Remember that the **CAN ID directly determines priority**. Lower IDs have higher priority on the bus. That means:
   `A has higher priority than B` ⇔ `ID(A) < ID(B)`

   To assign an ID, implement the `CanMessage` trait. You can do this in several ways:

    - **fully manually**:
      ```rust
      impl CanMessage for MyMessage {
          const ID: u16 = 0x123;
      }
      ```

    - **with the derive macro**:
      ```rust
      #[derive(CanMessage)]
      #[can_id(0x123)]
      pub struct MyMessage {
          // ...
      }
      ```

    - **recommended: with the `can_message` attribute macro**:
      ```rust
      #[can_message(0x123)]
      pub struct MyMessage {
          // ...
      }
      ```

      This is the cleanest and also derives all required traits. It expands to:
      ```
      #[derive(
          Clone,
          Debug,
          PartialEq,
          ::bincode::Encode,
          ::bincode::Decode,
          ::hermes_can_derive::CanMessage
      )]
      #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
      #[cfg_attr(feature = "defmt", derive(defmt::Format))]
      #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
      #[can_id(0x123)]
      ```
      **These trait derivations and cfgs are mandatory** — they are used for fuzz testing, logging, serialization, and more. If they’re missing, things will break or CI will fail.

4. **Document your message** properly. Check the [documentation format](#documentation-format) for details on how to structure your message documentation.

5. **Add the defined message to the `Message` enum**
   To use your message in the system, add it to the main message enum in `src/messages.rs`:
   ```rust
   define_messages!(Message {
       // ... existing messages
       #[cfg(feature = "message-my-message")]
       MyMessage(my_module::MyMessage)
   });
   ```
   Note the feature flag! This is used to **conditionally compile** messages. That way we don’t need to match on messages we don’t care about and can stay within the hardware filter limits on the CAN interface.

   Also **add the feature flag** to `Cargo.toml`:
   ```toml
   [features]
   message-my-message = []
   ```

6. **Testing**
   Now make sure everything works. CI will check all of this, but you should verify locally:

    - Run `cargo fmt` to format your code
    - Run `cargo clippy` to check for mistakes
    - Run fuzz testing:
      ```sh
      cargo fuzz run roundtrip -- -jobs=2 -max_total_time=60
      ```
      (You might need to install it with `cargo install cargo-fuzz`)
      **Note**: this only works on **Unix systems**. If it fails, your message may be too large — try reducing the encoded size.

## Using the Library

To depend on this library, add the following to your `Cargo.toml`:
```toml
[dependencies.hermes-can]
path = "path/to/hermes-can"
features = [] # include any message feature flags you need
```

**NOTE:**
If using this library as a submodule, it is highly recommended to specify the version in `Cargo.toml`. This ensures that Cargo will raise an error if the version in the submodule does not match the expected one.

Additionally, prefer checking out version **tags** in the submodule rather than using raw commit hashes, to improve clarity and traceability.

In `Cargo.toml`, specify the dependency like this:

```toml
hermes-can = { path = "hermes-can", version = "X.Y.Z" } # Replace with the desired version tag.
```

See the [examples](examples/) directory for how to use the library:
- **[usage.rs](examples/usage.rs)**: Shows how to use the encoding and decoding functions, but isn't very useful on its own.
- **[embedded.rs](examples/embedded.rs)**: Demonstrates how to use the library in a (mock) embedded context.

## Features
The library is `#![no_std]` and does not require `alloc`. We include some optional features for testing and debugging:
- **`arbitrary`**: Enables the `arbitrary` crate for fuzz testing. This feature is not required for normal usage.
- **`std`**: Enables the `std` crate for testing. This has no effect on the library itself and is only used for testing.
- **`defmt`**: Enables the `defmt` crate for logging. This feature should be enabled in embedded contexts.
- **`serde`**: Implements `serde` serialization and deserialization for messages.
- **`message-<name>`**: Conditionally enables a message in the `Message` enum for decoding.
  All messages are always compiled and can be used (e.g. for sending), but only feature-enabled ones are included in the `define_messages!` enum.
  This controls which messages you can match on, and which appear in the auto-generated `ENABLED_IDS` list. We can use this to nicely generate and configure the hardware filters for the CAN interface.

## Committing and pushing

Like in all HERMES repos, the `main` branch is protected. You’ll need to create a branch and submit a pull request. CI tests will run automatically and check your changes. If everything is green, request a review from someone, and they can merge your changes.
