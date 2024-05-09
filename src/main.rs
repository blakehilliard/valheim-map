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
        Building::new("Hrogden's Hrarbor", 0, -130),
        Building::new("Timberhold", 77, -130),
        Building::new("Skjor's Landing", -43, -422),
    ];

    // Draw
    for building in buildings {
        println!("TODO: Draw {:?}", building)
    }
}
