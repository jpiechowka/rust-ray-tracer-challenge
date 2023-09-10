# Rust ray tracer challenge
Rust ray tracer implementation, based on The Ray Tracer Challenge book by Jamis Buck.

## Building

Install Rust (https://www.rust-lang.org/tools/install), then run the commands below:

```
git clone https://github.com/jpiechowka/rust-ray-tracer-challenge.git
cd rust-ray-tracer-challenge
cargo build --release
```

### Using RUSTFLAGS env variable

If you do not care that much about the compatibility of your binary on older (or other types of) processors, you can tell the compiler to generate the newest (and potentially fastest) instructions specific to a certain CPU architecture by using `RUSTFLAGS`environment variable (https://nnethercote.github.io/perf-book/build-configuration.html#cpu-specific-instructions)

```
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

The full command to clone and build will be:

```
git clone https://github.com/jpiechowka/rust-ray-tracer-challenge.git
cd rust-ray-tracer-challenge
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

On Windows you need to follow this pattern: https://superuser.com/a/1049433

## Running examples

To run examples execute the following command:

```
cargo run --release --example {example_name}
```

For example to run projectile example run:

```
cargo run --release --example projectile
```

You can also build all of the examples and run them from the `examples` directory:

```
cargo build --release --examples
cd target/release/examples
./projectile
```

## Progress tracker based on the book chapters

Currently the following features (or code) are implemented:

- [X] Chapter 01: Tuples, Points, and Vectors
- [X] Chapter 02: Drawing on a Canvas
- [X] Chapter 03: Matrices
- [X] Chapter 04: Matrix Transformations
- [X] Chapter 05: Ray-Sphere Intersections
- [ ] Chapter 06: Light and Shading
- [ ] Chapter 07: Making a Scene
- [ ] Chapter 08: Shadows
- [ ] Chapter 09: Planes
- [ ] Chapter 10: Patterns
- [ ] Chapter 11: Reflection and Refraction
- [ ] Chapter 12: Cubes
- [ ] Chapter 13: Cylinders
- [ ] Chapter 14: Groups
- [ ] Chapter 15: Triangles
- [ ] Chapter 16: Constructive Solid Geometry (CSG)

## Gallery

### Chapter 1 and 2: Projectile

<p align="center">
  <img src="/examples/projectile/projectile.png" width="800" title="projectile">
</p>

### Chapter 3 and 4: Clock

<p align="center">
  <img src="/examples/clock/clock.png" width="800" title="clock">
</p>

### Chapter 5: Circle

<p align="center">
  <img src="/examples/circle/circle.png" width="800" title="circle">
</p>

## License

Rust ray tracer is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License (`LICENSE-MIT` file or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 (`LICENSE-APACHE` file or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
