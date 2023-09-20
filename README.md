# SIMD-Tree

This repository offers a high-performance KD-tree data structure, optimized using Single Instruction, Multiple Data (SIMD) operations, implemented in Rust.

![Build Status Badge](BUILD_BADGE_URL)
![License Badge](LICENSE_BADGE_URL)

## Features 🌟

- **SIMD Optimization**: KD-Tree operations that leverage the power of SIMD instructions.
- **Memory Efficient**: Utilizes memory arenas to keep memory allocations to a minimum, leading to faster performance.
  
## Installation 📦

1. **Add Dependency**:
   Include this in your `Cargo.toml`:

    ```toml
    [dependencies]
    simd-tree = { git = "https://github.com/bcmcmill/simd-tree" }
    ```

2. **Use in Your Rust Project**:

    ```rust
    use simd_tree::{Node, Point, search, insert};

    let mut points: Arena<Node> = Arena::new();
    let root = insert(&mut points, None, [2.0, 3.0]);

    assert_eq!(search(&points, Some(root), [2.0, 3.0]), true);
    ```

## Documentation 📚

Visit our [documentation page](DOCS_RS_LINK) for in-depth guides and API details.

## Benchmarking 🚀

See the performance benchmarks of our KD-Tree against other implementations [here](BENCHMARK_LINK).

## Contributing 💡

We welcome all contributors, whether you're looking to fix bugs, improve docs, or propose new features.

- **Issues**: Feel free to open issues for any feedback, bugs, or feature requests.
- **Pull Requests**: Submit PRs for enhancements, bug fixes, or documentation updates. Please ensure you follow our coding standards.

Before contributing, kindly read our [Contribution Guidelines](CONTRIB_LINK).

## License 📄

This library is distributed under the MIT License. See the [LICENSE](./LICENSE) file for more details or check the license [here](http://opensource.org/licenses/mit).
