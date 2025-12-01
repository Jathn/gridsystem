# GridSystem

A high-performance, parallel-capable 2D grid library for Rust.

## Features

- **Efficient Storage**: Uses a flat vector with row-major ordering for cache locality.
- **Parallel Processing**: Built-in support for `rayon` to perform parallel operations on grid cells.
- **Type Safety**: Generic implementation working with any type that implements `Default` and `Clone`.
- **Easy API**: Simple get/set methods with bounds checking, plus iterators and map functions.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gridsystem = "0.1.0"
```

### Basic Example

```rust
use gridsystem::Grid;

fn main() {
    let mut grid = Grid::new(10, 10);
    grid.set(5, 5, 42).unwrap();
    assert_eq!(*grid.get(5, 5).unwrap(), 42);
}
```

### Parallel Processing

```rust
use gridsystem::Grid;

fn main() {
    let grid: Grid<i32> = Grid::with_value(100, 100, 1);
    
    // Create a new grid with values doubled in parallel
    let doubled = grid.par_map(|x, y, &val| val * 2);
    
    assert_eq!(*doubled.get(0, 0).unwrap(), 2);
}
```

## License

This project is licensed under the MIT License.
