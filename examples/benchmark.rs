use gridsystem::Grid;
use std::time::Instant;

fn main() {
    // Use a large enough grid to make parallelization overhead negligible
    // and show the benefits of multi-threading.
    let width = 4000;
    let height = 4000;
    println!(
        "Creating a {}x{} grid ({} elements)...",
        width,
        height,
        (width as usize) * (height as usize)
    );

    // Initialize grid with some values
    // We use a simple calculation to populate the grid
    let mut grid = Grid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            // Just some arbitrary data
            if let Err(e) = grid.set(x, y, (x + y) as i32) {
                println!("Failed to set grid value at ({x:?}, {y:?}): {e:?}");
            }
        }
    }

    println!("Grid created. Starting benchmarks...");

    // ---------------------------------------------------------
    // Sequential Benchmark
    // ---------------------------------------------------------
    // Since Grid doesn't have a built-in sequential map that returns a new grid,
    // we simulate it by iterating and setting values in a new grid.
    let start_seq = Instant::now();

    let seq_result: Grid<i64> = {
        let mut result = Grid::new(width, height);
        for (x, y, &val) in grid.iter() {
            // Heavy calculation: square the number
            // In a real scenario, this might be a complex simulation step
            let squared = (val as i64) * (val as i64);
            if let Err(e) = result.set(x, y, squared) {
                println!(
                    "Failed to set grid value when calculating square at ({x:?}, {y:?}): {e:?}"
                );
            };
        }
        result
    };

    let duration_seq = start_seq.elapsed();
    println!("Sequential time: {duration_seq:?}");

    // ---------------------------------------------------------
    // Parallel Benchmark
    // ---------------------------------------------------------
    // We use the `par_map` feature provided by the library
    let start_par = Instant::now();

    let par_result: Grid<i64> = grid.par_map(|_x, _y, &val| {
        // Same heavy calculation
        (val as i64) * (val as i64)
    });

    let duration_par = start_par.elapsed();
    println!("Parallel time:   {duration_par:?}");

    // ---------------------------------------------------------
    // Verification & Results
    // ---------------------------------------------------------
    println!("Verifying results...");
    // Check a few random points to ensure correctness
    assert_eq!(seq_result.get(0, 0), par_result.get(0, 0));
    assert_eq!(seq_result.get(100, 100), par_result.get(100, 100));
    assert_eq!(
        seq_result.get(width - 1, height - 1),
        par_result.get(width - 1, height - 1)
    );

    println!("Results match!");

    let speedup = duration_seq.as_secs_f64() / duration_par.as_secs_f64();
    println!("Speedup: {speedup:.2}x");

    if speedup > 1.0 {
        println!("Parallel implementation is faster!");
    } else {
        println!("Parallel implementation is slower (grid might be too small).");
    }
}
