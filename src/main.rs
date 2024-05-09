#[derive(Debug)]
struct Building {
    name: String,
    x: i64,
    y: i64,
}

impl Building {
    pub fn new(name: &str, x: i64, y: i64) -> Self {
        Self {
            name: String::from(name),
            x, y,
        }
    }
}

fn get_furthest_point_from_center(buildings: &[Building]) -> u32 {
    let mut biggest: i64 = 0;
    for building in buildings {
        let num = std::cmp::max(building.x.abs(), building.y.abs());
        if num > biggest {
            biggest = num;
        }
    }
    biggest as u32
}

fn main() -> ril::Result<()> {
    // Define buildings
    let buildings = vec![
        Building::new("Last Resort", -10, 3),
        Building::new("Grig's Boulderheim", -2, -10),
        Building::new("Hrogden's Hrarbor", 0, -137),
        Building::new("Timberhold", 77, -137),
        Building::new("Skjor's Landing", -43, -429),
    ];

    // Create underlying image
    let map_radius = get_furthest_point_from_center(&buildings) + 50;
    println!("Map radius: {}", map_radius);
    let image = ril::Image::new(map_radius, map_radius, ril::Rgb::new(255, 255, 255));

    // Draw
    for building in buildings {
        println!("TODO: Draw {} at {},{}", building.name, building.x, building.y)
    }

    // Save image
    let path = "image.jpg";
    image.save_inferred(path)?;
    println!("Saved to {}", path);

    Ok(())
}
