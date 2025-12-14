# Yuno FAQ Manager
This is the refractored version of the REPL old-main branch.

It will be overall changed to a more modular architecture; expect some containers.

If you still want the original python, you can switch to the 'old-main' branch.

## Requisites

- Linux (Probably)
- Rust
- Cargo
- Docker

## Architecture

The REPL, which "acts as the frontend", will be written in Rust.
It simply takes and forwards whatever commands are written on the Terminal.

The content are send via an api call to the Go "backend".
This will simply write the data to the MongoDB. It also is responsible for creating the db schema.

The database will be MongoDB (8.2.2).

## Quick Start
If you installed the binary from the release, you can simply:

    chmod +x yuno-faqman
    ./yuno-faqman

You can also run it as dev:

1. Clone this repo
2. cargo build
3. cargo run

Optionally, build the binary

1. cargo build --release
2. cd target/release; chmod +x yuno-faqman
3. ./yuno-faqman

## DB Schema

### Thema

### Tag

### TranslationGroup

### QA
