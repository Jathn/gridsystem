use rand::random;

use gridsystem::Grid;

#[derive(Clone, Debug, Default, PartialEq)]
enum TerrainType {
    #[default]
    Grass,
    Water,
    Mountain,
    Forest,
    Sand,
}

#[derive(Clone, Debug, Default)]
struct Tile {
    terrain: TerrainType,
    elevation: u8,
    is_walkable: bool,
    movement_cost: u8,
}

fn main() {
    let width = 20;
    let height = 10;

    println!("Creating a {width:?}x{height:?} terrain map...");
    let mut map: Grid<Tile> = Grid::new(width, height);

    println!("Generating terrain using parallel processing...");

    map.par_map_inplace(|_x, _y, tile| {
        let rand_val: u8 = random::<u8>() % 100;
        match rand_val {
            0..=10 => {
                tile.terrain = TerrainType::Water;
                tile.elevation = 0;
                tile.is_walkable = false;
                tile.movement_cost = 255;
            }
            11..=30 => {
                tile.terrain = TerrainType::Sand;
                tile.elevation = 1;
                tile.is_walkable = true;
                tile.movement_cost = 2;
            }
            31..=60 => {
                tile.terrain = TerrainType::Grass;
                tile.elevation = 1;
                tile.is_walkable = true;
                tile.movement_cost = 1;
            }
            61..=85 => {
                tile.terrain = TerrainType::Forest;
                tile.elevation = 2;
                tile.is_walkable = true;
                tile.movement_cost = 3;
            }
            _ => {
                tile.terrain = TerrainType::Mountain;
                tile.elevation = 10;
                tile.is_walkable = false;
                tile.movement_cost = 255;
            }
        }
    });

    println!("\nMap Visualization:");
    println!("Legend: W Water, M Mountain, F Forest, G Grass, S Sand");
    println!("-------------------------------------------------------");

    for y in 0..height {
        for x in 0..width {
            let tile = map.get(x, y);
            match tile {
                None => {
                    print!("?");
                    continue;
                }
                Some(t) => {
                    let symbol = match t.terrain {
                        TerrainType::Water => 'W',
                        TerrainType::Mountain => 'M',
                        TerrainType::Forest => 'F',
                        TerrainType::Grass => 'G',
                        TerrainType::Sand => 'S',
                    };
                    print!("{symbol}");
                }
            }
        }
        println!();
    }

    println!("\nInspecting specific tiles:");
    let center_tile = map.get(width / 2, height / 2);
    match center_tile {
        None => println!("Center tile is out of bounds."),
        Some(t) => println!(
            "Center Tile Terrain: {:?}, Elevation: {}, Walkable: {}, Movement Cost: {}",
            t.terrain, t.elevation, t.is_walkable, t.movement_cost
        ),
    }

    let corner_tile = map.get(0, 0);
    match corner_tile {
        None => println!("Corner tile is out of bounds."),
        Some(t) => println!(
            "Corner Tile Terrain: {:?}, Elevation: {}, Walkable: {}, Movement Cost: {}",
            t.terrain, t.elevation, t.is_walkable, t.movement_cost
        ),
    }
}
