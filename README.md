# NoiseLib-Rust
Perlin Noise library for Rust.

## What is NoiseLib-Rust? (And what is Perlin Noise?)
NoiseLib-Rust is a pure Rust implementation of [Perlin Noise](https://en.wikipedia.org/wiki/Perlin_noise) (aka Fractal Brownian Motion), a mathmatical representation of randomness in continuous space. Common uses for Perlin Noise include generating mountainous terrain in computer simulations, creating cloud-like graphical effects, and various texture effects such as marble patterns.

## Why Rust?
I have already created a [Perlin Noise library for Java](https://github.com/DrPlantabyte/Cyanos-Noise-Library), but it is difficult to integrate Java code with modern WASM-based web apps, so I'm creating a Rust version that can be used in WASM as well as anywhere else that Rust can be compiled. If you wanted to use my noise libraries in C++, for example, interfacing the Rust binary would be much easier than using Java's JNI system.

This project is similar to the [noise](https://crates.io/crates/noise) crate, but is much smaller in scope and has no dependencies. The key difference is that NoiseLib-Rust lets you provide your own random number hashing algorithm, in case you need to mirror another random number implementation or need to make performance optimizations.

## Examples
TODO
