use gridsystem::Grid;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Tile {
    walkable: bool,
    terrain_type: u8,
}

#[test]
fn test_new_map() {
    let map: Grid<i32> = Grid::new(10, 10);
    assert_eq!(map.width(), 10);
    assert_eq!(map.height(), 10);
    assert_eq!(map.area(), 100);
}

#[test]
fn test_with_value() {
    let map: Grid<i32> = Grid::with_value(5, 5, 42);

    assert_eq!(map.get(0, 0), Some(&42));
    assert_eq!(map.get(4, 4), Some(&42));
    assert_eq!(map.get(2, 3), Some(&42));
}

#[test]
fn test_get_set() {
    let mut map: Grid<i32> = Grid::new(5, 5);

    assert_eq!(map.get(2, 2), Some(&0));

    assert!(map.set(2, 2, 42).is_ok());
    assert_eq!(map.get(2, 2), Some(&42));
    assert!(map.set(4, 4, 99).is_ok());
    assert_eq!(map.get(4, 4), Some(&99));
}

#[test]
fn test_get_mut() {
    let mut map: Grid<i32> = Grid::new(5, 5);

    if let Some(cell) = map.get_mut(2, 2) {
        *cell = 42;
    }

    assert_eq!(map.get(2, 2), Some(&42));
}

#[test]
fn test_out_of_bounds() {
    let mut map: Grid<i32> = Grid::new(5, 5);

    // Get out of bounds
    assert!(map.get(10, 10).is_none());
    assert!(map.get(5, 0).is_none());
    assert!(map.get(0, 5).is_none());

    // Set out of bounds
    assert!(map.set(10, 10, 42).is_err());
    assert!(map.set(5, 0, 42).is_err());
    assert!(map.set(0, 5, 42).is_err());
}

#[test]
fn test_error_message() {
    let mut map: Grid<i32> = Grid::new(5, 5);
    let result = map.set(10, 10, 42);

    assert!(result.is_err());
    let Err(err) = result else {
        panic!("expected error, got: {result:?}");
    };
    assert!(err.contains("10, 10"));
    assert!(err.contains("5x5"));
}

#[test]
fn test_iter() {
    let mut map: Grid<i32> = Grid::new(3, 2);
    let _ = map.set(1, 0, 10);
    let _ = map.set(2, 1, 20);

    let items: Vec<_> = map.iter().collect();
    assert_eq!(items.len(), 6);

    // Check first row
    assert_eq!(items[0], (0, 0, &0));
    assert_eq!(items[1], (1, 0, &10));
    assert_eq!(items[2], (2, 0, &0));

    // Check second row
    assert_eq!(items[3], (0, 1, &0));
    assert_eq!(items[4], (1, 1, &0));
    assert_eq!(items[5], (2, 1, &20));
}

#[test]
fn test_with_custom_tile() {
    let mut map: Grid<Tile> = Grid::new(5, 5);

    // Default tile should have walkable=false, terrain_type=0
    let default_tile = map.get(0, 0);

    match default_tile {
        None => panic!("Expected a tile at (0,0)"),
        Some(t) => assert_eq!(*t, Tile::default()),
    }

    // Set a custom tile
    let custom_tile = Tile {
        walkable: true,
        terrain_type: 3,
    };
    let _ = map.set(2, 2, custom_tile);

    let retrieved = map.get(2, 2);

    match retrieved {
        None => panic!("Expected a tile at (2,2)"),
        Some(t) => {
            assert!(t.walkable);
            assert_eq!(t.terrain_type, 3);
        }
    }
}

#[test]
fn test_char_map() {
    let map: Grid<char> = Grid::with_value(3, 3, '.');

    for y in 0..3 {
        for x in 0..3 {
            assert_eq!(map.get(x, y), Some(&'.'));
        }
    }
}

#[test]
fn test_large_map() {
    let map: Grid<u8> = Grid::new(255, 255);
    assert_eq!(map.area(), 65025);
}
