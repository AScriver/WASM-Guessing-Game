# WASM Guessing Game

A simple number guessing game built using Rust and WebAssembly (WASM).

## Description

In this game, the computer generates a random number between 1 and 100. The player's task is to guess this number. The game provides feedback on whether the player's guess is too low, too high, or correct.

## Prerequisites

Before you start, ensure you have the following installed:

- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- wasm-pack: `cargo install wasm-pack`
- [Node.js](https://nodejs.org/) (Optional for the http-server utility)
  - http-server: `npm install -g http-server`
## Getting Started

To get the project up and running on your local machine, follow these steps:

1. **Clone the repository**:
```
git clone https://github.com/AScriver/WASM-Guessing-Game.git wasm_guessing_game
```

2. **Navigate to the project directory**:
```
cd wasm_guessing_game
```

3. **Build the WASM module**:
```
wasm-pack build --release --target web
```

4. **Serve the project locally**:
Navigate to the root directory of your project and start the server with CORS enabled:
```
http-server . -p 8000 --cors
```

5. **Access the game**:
Open a web browser and navigate to `http://localhost:8000` to play the game.
