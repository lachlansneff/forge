# Forge

[![Crates.io](https://img.shields.io/crates/v/forge.svg)](https://crates.io/crates/forge)
[![Crates.io](https://img.shields.io/crates/l/forge.svg)](https://github.com/metal-os/metal)

Forge is a tool for creating bare-metal applications with [`metal`](https://github.com/metal-os/metal).

## Installation:

Install with `cargo install forge`.

## Usage:

1. Create a new project with `forge new <project-name>`

2. Build with `forge build`.

3. Run with `qemu-system-x86_64 -cdrom build/x86_64.iso`.

If I have time, I'll add more architectures.

Caveats:

1. Currently, you must have `grub-mkrescue` (`apt install xorriso`), `ld`, and `xargo` installed into the path.
