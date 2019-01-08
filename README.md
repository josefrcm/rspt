# rspt

## Introduction

Toy path-tracer written in Rust, written as a learning exercise. My motivation is two-fold:

- I wanted to learn a new programming language. My favourite languages are C++ and Haskell, and Rust seems to take some of the best things of both.
- Computer graphics has always been my favourite programming topic, and I had been wanting to write a path-tracer for the last two years.

## Running the program

To run the renderer just run the _test.sh_ or _test.bat_ script. It will compile the program and render a sample scene.

## Future plans

- Write a proper documentation.
- Create a proper material definition format.
- Add support for texture mapping and skyboxes.
- Add support for geometry instancing.
- Use SIMD for intersection tests.
- Improve the BVH building code, the current one is excedingly naive.
- Improve the sampling strategy.
