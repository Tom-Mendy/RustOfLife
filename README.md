# RustOfLife 🦀

A high-performance implementation of Conway's Game of Life written in Rust with SDL2 graphics rendering.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SDL](https://img.shields.io/badge/SDL2-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

## 🌟 Features

- **Interactive Simulation**: Real-time Conway's Game of Life simulation
- **Mouse Controls**: Click and drag to create/remove cells
- **Keyboard Controls**: Space to play/pause, R to reset, Escape to quit
- **Performance Optimized**: Multi-threaded Game of Life algorithm
- **Dynamic UI**: Real-time display of iteration count, population, and iterations per second
- **Resizable Window**: Automatically adjusts grid to window size
- **Cross-platform**: Built with SDL2 for compatibility across different operating systems

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable version)
- `pkg-config` and a C toolchain (`gcc`, `make`)
- SDL2 development libraries
- SDL2_ttf development libraries
- CMake (only needed when SDL2 development packages are unavailable)

Run the helper script to install the dependencies on common Linux distributions:

```bash
./scripts/install-deps.sh
```

#### Ubuntu/Debian

```bash
sudo apt-get update
sudo apt-get install pkg-config cmake build-essential libsdl2-dev libsdl2-ttf-dev
```

#### macOS (Homebrew)

```bash
brew install pkg-config cmake sdl2 sdl2_ttf
```

#### Arch Linux

```bash
sudo pacman -S pkgconf cmake base-devel sdl2 sdl2_ttf
```

### Installation & Running

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Tom-Mendy/RustOfLife.git
   cd RustOfLife
   ```

1. **Run the game:**

   ```bash
   cargo run --release
   ```

   Or use the provided script:

   ```bash
   ./start.sh
   ```

## 🎮 Controls

| Control | Action |
|---------|--------|
| **Left Mouse Button** | Toggle cell state (click) or paint cells (drag) |
| **Right Mouse Button** | Erase cells (drag) |
| **Space** | Play/Pause simulation |
| **R** | Reset grid (clear all cells) |
| **Escape** | Exit application |

## 🏗️ Building

### Development Build

```bash
cargo build
```

### Optimized Release Build

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

Or with coverage:

```bash
./test.sh
```

## 🐳 Docker Support

Build and run using Docker:

```bash
# Build the image
docker-compose build

# Run the container
docker-compose up
```

## 🛠️ Development Environment

### Using Nix Flakes (Recommended)

This project includes a Nix flake for reproducible development environments:

```bash
# Enter the development shell
nix develop

# Or with direnv
echo "use flake" > .envrc
direnv allow
```

The Nix environment provides:

- Rust toolchain with rust-analyzer
- SDL2 and SDL2_ttf libraries
- Additional development tools (cargo-watch, cargo-edit, etc.)

## 📁 Project Structure

```text
RustOfLife/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library root
│   ├── game.rs          # Game state management
│   ├── sdl_lib.rs       # SDL2 wrapper functions
│   └── utils.rs         # Game logic and utilities
├── tests/               # Unit tests
├── assets/
│   └── Roboto-Medium.ttf # Font for UI text
├── Cargo.toml           # Project configuration
├── Dockerfile           # Docker configuration
├── flake.nix           # Nix flake for development
└── README.md           # This file
```

## 🧬 Game of Life Rules

Conway's Game of Life follows these simple rules:

1. **Birth**: A dead cell with exactly 3 live neighbors becomes alive
2. **Survival**: A live cell with 2 or 3 live neighbors stays alive
3. **Death**: A live cell with fewer than 2 or more than 3 neighbors dies

## 🔧 Configuration

The game uses a default configuration that can be modified in `src/game.rs`:

- **Grid Size**: 100x100 cells
- **Window Size**: 1000x1000 pixels
- **Max Iterations/Second**: 10 (configurable)

## 🎯 Performance

- **Multi-threaded**: Game of Life calculations use thread parallelization
- **Optimized Rendering**: Efficient SDL2 rendering with minimal allocations
- **Release Mode**: Compiled with aggressive optimizations (`opt-level = 3`, LTO)

## 🧪 Testing

The project includes comprehensive tests covering:

- Game logic and rules
- SDL2 integration
- Event handling
- UI components

Run tests with:

```bash
cargo test --verbose
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [Rust Programming Language](https://www.rust-lang.org/)
- [SDL2 Library](https://www.libsdl.org/)

## 🏆 Acknowledgments

- John Conway for creating the Game of Life cellular automaton
- The Rust community for excellent tooling and libraries
- SDL2 developers for the graphics framework
