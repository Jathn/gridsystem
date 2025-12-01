/// A generic 2D grid structure using a flat vector with row-major order.
///
/// The `Grid` stores elements of type `T` in a contiguous vector, providing
/// efficient memory access and cache locality. Elements are stored in row-major
/// order, meaning rows are stored sequentially in memory.
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the grid. Must implement `Default` and `Clone`.
///
/// # Examples
///
/// ```
/// use gridsystem::Grid;
///
/// let mut grid: Grid<i32> = Grid::new(10, 10);
/// let _ = grid.set(5, 3, 42);
/// assert_eq!(grid.get(5, 3), Some(&42));
/// ```
pub struct Grid<T> {
    width: u16,
    height: u16,
    tiles: Vec<T>, // Flat vector in row-major order
}

impl<T: Default + Clone> Grid<T> {
    /// Creates a new grid with the given dimensions, filled with default values.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::new(10, 10);
    /// assert_eq!(grid.width(), 10);
    /// assert_eq!(grid.height(), 10);
    /// ```
    pub fn new(width: u16, height: u16) -> Grid<T> {
        let capacity = (width as usize) * (height as usize);
        Grid {
            width,
            height,
            tiles: vec![T::default(); capacity],
        }
    }

    /// Creates a new grid with the given dimensions, filled with clones of the provided value.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    /// * `value` - The value to fill the grid with
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<char> = Grid::with_value(5, 5, '.');
    /// assert_eq!(grid.get(0, 0), Some(&'.'));
    /// ```
    pub fn with_value(width: u16, height: u16, value: T) -> Grid<T> {
        let capacity = (width as usize) * (height as usize);
        Grid {
            width,
            height,
            tiles: vec![value; capacity],
        }
    }

    /// Converts (x, y) coordinates to a flat index.
    ///
    /// This uses row-major order: index = y * width + x
    #[inline]
    fn index(&self, x: u16, y: u16) -> usize {
        (y as usize) * (self.width as usize) + (x as usize)
    }

    /// Converts a flat index to (x, y) coordinates.
    #[inline]
    fn coords(&self, index: usize) -> (u16, u16) {
        let x = (index % self.width as usize) as u16;
        let y = (index / self.width as usize) as u16;
        (x, y)
    }

    /// Gets an immutable reference to the element at (x, y).
    ///
    /// Returns `None` if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::new(5, 5);
    /// assert!(grid.get(0, 0).is_some());
    /// assert!(grid.get(10, 10).is_none());
    /// ```
    pub fn get(&self, x: u16, y: u16) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.tiles[self.index(x, y)])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the element at (x, y).
    ///
    /// Returns `None` if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let mut grid: Grid<i32> = Grid::new(5, 5);
    /// if let Some(cell) = grid.get_mut(2, 2) {
    ///     *cell = 42;
    /// }
    /// assert_eq!(grid.get(2, 2), Some(&42));
    /// ```
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut T> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            Some(&mut self.tiles[idx])
        } else {
            None
        }
    }

    /// Sets the element at (x, y) to the provided value.
    ///
    /// Returns `Ok(())` if successful, or an `Err` with a descriptive message
    /// if the coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let mut grid: Grid<i32> = Grid::new(5, 5);
    /// assert!(grid.set(2, 2, 42).is_ok());
    /// assert!(grid.set(10, 10, 42).is_err());
    /// ```
    pub fn set(&mut self, x: u16, y: u16, tile: T) -> Result<(), String> {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.tiles[idx] = tile;
            Ok(())
        } else {
            Err(format!(
                "Coordinates ({}, {}) out of bounds (grid is {}x{})",
                x, y, self.width, self.height
            ))
        }
    }

    /// Returns the total area (width × height) of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::new(10, 10);
    /// assert_eq!(grid.area(), 100);
    /// ```
    pub fn area(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    /// Returns the width of the grid.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns an iterator over all elements with their coordinates.
    ///
    /// Each item is a tuple of (x, y, &T).
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::new(3, 3);
    /// let coords: Vec<_> = grid.iter().map(|(x, y, _)| (x, y)).collect();
    /// assert_eq!(coords.len(), 9);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (u16, u16, &T)> {
        self.tiles.iter().enumerate().map(move |(i, tile)| {
            let (x, y) = self.coords(i);
            (x, y, tile)
        })
    }

    /// Returns a mutable iterator over all elements with their coordinates.
    ///
    /// Each item is a tuple of (x, y, &mut T).
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u16, &mut T)> {
        let width = self.width;
        self.tiles.iter_mut().enumerate().map(move |(i, tile)| {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            (x, y, tile)
        })
    }

    /// Generates a new grid mutated by function f.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::with_value(10, 10, 5);
    /// let doubled = grid.map(|x, y, &value| value * 2);
    /// assert_eq!(doubled.get(0, 0), Some(&10));
    /// ```
    pub fn map<F, R>(&self, f: F) -> Grid<R>
    where
        F: Fn(u16, u16, &T) -> R,
        R: Default + Clone,
    {
        let width = self.width;
        let tiles: Vec<R> = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let x = (i % width as usize) as u16;
                let y = (i / width as usize) as u16;
                f(x, y, tile)
            })
            .collect();

        Grid {
            width: self.width,
            height: self.height,
            tiles,
        }
    }

    /// Modifies each tile in the grid, replacing its previous value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let mut grid: Grid<i32> = Grid::with_value(10, 10, 5);
    /// grid.map_inplace(|x, y, value| {
    ///     *value = (x as i32) + (y as i32);
    /// });
    /// assert_eq!(grid.get(3, 4), Some(&7));
    /// ```
    pub fn map_inplace<F>(&mut self, f: F)
    where
        F: Fn(u16, u16, &mut T),
    {
        let width = self.width;
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            f(x, y, tile);
        }
    }

    /// Returns a slice of the underlying data.
    pub fn as_slice(&self) -> &[T] {
        &self.tiles
    }

    /// Returns a mutable slice of the underlying data.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.tiles
    }
}

// Parallel implementations using the rayon data-parallelism library.
impl<T: Send + Sync> Grid<T> {
    /// Returns a parallel iterator over all elements with their coordinates.
    ///
    /// This method requires `T` to implement `Send` and `Sync` for safe parallel access.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    /// use rayon::prelude::*;
    ///
    /// let grid: Grid<i32> = Grid::with_value(100, 100, 5);
    /// let sum: i32 = grid.par_iter()
    ///     .map(|(_, _, &value)| value)
    ///     .sum();
    /// assert_eq!(sum, 50000);
    /// ```
    pub fn par_iter(&self) -> impl rayon::iter::ParallelIterator<Item = (u16, u16, &T)> {
        use rayon::prelude::*;
        let width = self.width;
        self.tiles.par_iter().enumerate().map(move |(i, tile)| {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            (x, y, tile)
        })
    }

    /// Returns a parallel mutable iterator over all elements with their coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    /// use rayon::prelude::*;
    ///
    /// let mut grid: Grid<i32> = Grid::new(10, 10);
    /// grid.par_iter_mut().for_each(|(x, y, cell)| {
    ///     *cell = (x as i32) * 10 + (y as i32);
    /// });
    /// assert_eq!(grid.get(5, 3), Some(&53));
    /// ```
    pub fn par_iter_mut(
        &mut self,
    ) -> impl rayon::iter::ParallelIterator<Item = (u16, u16, &mut T)> {
        use rayon::prelude::*;
        let width = self.width;
        self.tiles.par_iter_mut().enumerate().map(move |(i, tile)| {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            (x, y, tile)
        })
    }
}

impl<T: Default + Clone + Send + Sync> Grid<T> {
    /// Generates a new grid mutated by function f.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let grid: Grid<i32> = Grid::with_value(10, 10, 5);
    /// let doubled = grid.par_map(|x, y, &value| value * 2);
    /// assert_eq!(doubled.get(0, 0), Some(&10));
    /// ```
    pub fn par_map<F, R>(&self, f: F) -> Grid<R>
    where
        F: Fn(u16, u16, &T) -> R + Send + Sync,
        R: Default + Clone + Send,
    {
        use rayon::prelude::*;
        let width = self.width;
        let tiles: Vec<R> = self
            .tiles
            .par_iter()
            .enumerate()
            .map(|(i, tile)| {
                let x = (i % width as usize) as u16;
                let y = (i / width as usize) as u16;
                f(x, y, tile)
            })
            .collect();

        Grid {
            width: self.width,
            height: self.height,
            tiles,
        }
    }

    /// Modifies each tile in the grid, replacing its previous value.
    ///
    /// # Examples
    ///
    /// ```
    /// use gridsystem::Grid;
    ///
    /// let mut grid: Grid<i32> = Grid::with_value(10, 10, 5);
    /// grid.par_map_inplace(|x, y, value| {
    ///     *value = (x as i32) + (y as i32);
    /// });
    /// assert_eq!(grid.get(3, 4), Some(&7));
    /// ```
    pub fn par_map_inplace<F>(&mut self, f: F)
    where
        F: Fn(u16, u16, &mut T) + Send + Sync,
    {
        use rayon::prelude::*;
        let width = self.width;
        self.tiles.par_iter_mut().enumerate().for_each(|(i, tile)| {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            f(x, y, tile);
        });
    }
}
