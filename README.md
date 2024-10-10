# Paddles: A Minimal Ping-Pong Example

This crate demonstrates a minimal ping-pong interaction using the [Flawless](https://flawless.dev) framework for durable execution and memory passing over HTTP. It features two actors, **Pinger** and **Ponger**, that send a "ball" back and forth via HTTP requests until the number of rallies is reached.

## Features

- **Durable Execution**: Uses Flawless workflows for reliable, long-running tasks.
- **Memory Passing Over HTTP**: Communication between actors via HTTP post requests.
- **Idempotent Actions**: Ensures durable message handling in case of failure.

## Quick Start

1. Clone the repository and navigate to the project directory.
2. Ensure [Flawless CLI](https://flawless.dev/docs/cli/) is installed and running:
   ```bash
   flawless up
   ```
3. Run the example:
   ```bash
   cargo run
   ```

The Pinger and Ponger actors will start, and they will ping-pong a ball back
and forth with a 1 second delay over HTTP until the rally count is reached.

## Durability Test

1. Kill the flawless server while the rally is going.
2. Start it back up again and observe the power of durable execution:
   ```bash
   flawless up
   ```

## Requirements
- Rust
- Flawless CLI for execution orchestration
