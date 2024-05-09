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

fn main() {
    // Define buildings
    let buildings = vec![
        Building::new("Last Resort", -10, 3),
        Building::new("Grig's Boulderheim", -2, -10),
        Building::new("Hrogden's Hrarbor", 0, -137),
        Building::new("Timberhold", 77, -137),
        Building::new("Skjor's Landing", -43, -429),
    ];

    // Draw
    for building in buildings {
        println!("TODO: Draw {} at {},{}", building.name, building.x, building.y)
    }
}
