# BioSpheres

A cellular simulation and evolution platform with GPU-accelerated physics.

## Project Structure

```
biospheres/
├── src/
│   ├── cell/              # Cell behavior and adhesion systems
│   ├── genome/            # Genome definitions and node graph
│   ├── input/             # User input handling
│   ├── rendering/         # Graphics and visualization
│   ├── simulation/        # Physics and simulation logic
│   ├── ui/                # ImGui-based user interface
│   ├── lib.rs            # Library root
│   └── main.rs           # Application entry point
├── assets/
│   ├── shaders/          # WGSL compute and render shaders
│   └── icon.ico          # Application icon
├── genomes/              # Genome JSON files
├── docs/                 # Documentation
├── examples/             # Example code
└── Cargo.toml           # Project configuration
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Genome Files

Genomes are stored as JSON files in the `genomes/` directory. See `GENOME_FORMAT.md` for details on the file format.

## Features

- GPU-accelerated physics simulation
- Cell division and adhesion systems
- Visual genome editor with node graph
- Real-time parameter tuning
- Save/load genome configurations
