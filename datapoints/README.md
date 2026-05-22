# Datapoints

The crates in this folder define the messages sorted by which component they belong to.

## Versioning

Each crate follows a practical [SemVer](https://semver.org/) versioning scheme `major.minor.patch`, with rules for binary compatibility:

- **Patch `0.x.(y+1)`:**

  - **Allowed:**
    - Changes to the macro invocations if necessary.
    - Documentation improvements.
    - Adding new CAN messages with IDs distinct from existing messages.
  - **Forbidden:**
    - Changing existing CAN message layouts, IDs, field meanings, or units.
    - Adding, removing or renaming fields in existing can messages.

- **Minor `0.(x+1).0`:**
  May introduce **any** change, including:

  - Changing CAN message layouts, IDs, or field meanings.
  - Adding new fields to existing messages.
  - Removing messages.

Please ensure that any breaking changes are kept to a minimum and avoided where possible.

## Adding a New Message

To add a new message to the system, follow these steps:

1. Pick the right datapoint crate (or create a new one):
   The question essentially is: which component could you remove such that having this message becomes useless.
   If the answer is none: then it is a `systems-management` messages, otherwise it should likely belong to your piece of hardware.

1. Choose or add a data type:
   If the type is only relevant for this datapoint crate, then add it into `dp-*/src/dptypes.rs`, else add it to `../datatypes`.
   All types must implement a bunch of gated derives, best just copy from an existing one.

1. Add the defined message to the `Message` enum

1. Assign a CAN and/or Mavlink IDs:
   Without a ID, the message cannot be sent.
   If it might be sent over CAN, add a CAN ID, if it might be sent over Mavlink (i.e. if it informs the MC or is a command sent from MC), add a MavLink ID. See `../messages/src/lib.rs` for some comments.

1. Document your message properly.
   In particular, for CAN messages, state who the unique sender is!
   If there are multiple, duplicate the message with the same type, but different IDs.
