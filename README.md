# NoiseLib-Rust
Perlin Noise library for Rust.

## What is NoiseLib-Rust? (And what is Perlin Noise?)
NoiseLib-Rust is a pure Rust implementation of [Perlin Noise](https://en.wikipedia.org/wiki/Perlin_noise) (aka fractal brownian motion), a mathmatical representation of randomness in continuous space. Common uses for Perlin Noise include generating mountainous terrain in computer simulations, creating cloud-like graphical effects, and various texture effects such as marble patterns.

## Why Rust?
I have already created a [Perlin Noise library for Java](https://github.com/DrPlantabyte/Cyanos-Noise-Library), but it is difficult to integrate Java code with modern WASM-based web apps, so I'm creating a Rust version that can be used in WASM as well as anywhere else that Rust can be compiled. If you wanted to use my noise libraries in C++, for example, interfacing the Rust binary would be much easier than using Java's JNI system.

## Examples
TODO
