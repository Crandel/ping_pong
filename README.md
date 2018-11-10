# Ping pong game

### Purposes

This test game was written on Rust, to learn leanguage and have a funny time.

## Development

#### Game manager

Contains map, all objects, run in infinite loop, send signal to objects

#### Ball

The Ball has a coordinates on map, has a valid directions of movement and functions, that let it move

#### Paddle

The Paddle has a coordinates on map, has a valid directions of movement and functions, that let it move

## Installation

### Preparation

You need to have a fresh version of **rust** and **cargo**

Clone this repository and build release version

```bash
$ git clone https://github.com/Crandel/rust_gmail_checker.git

$ cd rust_gmail_checker

$ cargo build --release
```

### Run

To test the game just run the command

```bash
$ target/release/ping_pong
```
