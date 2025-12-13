# Yuno AI
This is the refractores version of the REPL project.

It will be overall refractored to a more modular architecture, expect some containers.

If you still want the original python, you can switch to the 'old-main' branch.

## Requisites

- Rust on your local machine
- Docker/Nerdctl

## Architecture

The REPL, which "acts as the frontend", will be written in Rust.
It simply takes and forwards whatever commands are written on the Terminal.

The content are send via an api call to the Go "backend".
This will simply write the data to the MongoDB, as well as create the db schemas.

The database will be MongoDB (8.2.2).

## Quick Start

1. Clone this repo
2. rustc main.rs
3. ./main

## DB Schema

### Thema

### Tag

### TranslationGroup

### QA
