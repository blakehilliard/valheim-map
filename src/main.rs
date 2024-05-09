use ril::{Draw, Image, Line, Polygon, Rgb};

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

    pub fn draw(&self, image: &mut Image<Rgb>) {
        // We store coords as if center is 0,0.
        // Image has 0,0 has upper left corner.
        // Do conversion.
        let center = map_coords_to_image_coords(&image, self.x, self.y);
        
        let poly = Polygon::new()
            .with_fill(Rgb::black())
            .with_vertex(center.0-5, center.1+5)
            .with_vertex(center.0-5, center.1-5)
            .with_vertex(center.0+5, center.1-5)
            .with_vertex(center.0+5, center.1+5);
        poly.draw(image);
    }
}

#[derive(Debug)]
struct Road {
    vertices: Vec<(i64, i64)>,
}

impl Road {
    pub fn new() -> Self {
        Self {
            vertices: Vec::<(i64, i64)>::new(),
        }
    }

    pub fn with_vertex(mut self, x: i64, y: i64) -> Self {
        self.vertices.push((x, y));
        self
    }

    pub fn draw(&self, image: &mut Image<Rgb>) {
        for idx in 0..self.vertices.len()-1 {
            let a = map_coords_to_image_coords(image, self.vertices[idx].0, self.vertices[idx].1);
            let b = map_coords_to_image_coords(image, self.vertices[idx+1].0, self.vertices[idx+1].1);
            let brown = Rgb::new(0x96, 0x4b, 0x00);
            Line::new(a, b, brown)
                .with_thickness(3)
                .draw(&mut *image);
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

// We store coords for Valheim map as if center is 0,0.
// Image has 0,0 has upper left corner.
// Do conversion from former to latter.
fn map_coords_to_image_coords(image: &Image<Rgb>, map_x: i64, map_y: i64) -> (u32, u32) {
    let center = image.center();
    println!("img center {:?} map coords {} {}", center, map_x, map_y); //FIXME
    let mut img_x = center.0;
    if map_x < 0 {
        img_x -= map_x.abs() as u32;
    } else {
        img_x += map_x.abs() as u32;
    }
    println!("img_x: {}", img_x);
    let mut img_y = center.1;
    if map_y > 0 {
        img_y -= map_y.abs() as u32;
    } else {
        img_y += map_y.abs() as u32;
    }
    println!("img_y: {}", img_y);
    (img_x, img_y)
}

fn main() -> ril::Result<()> {
    // Define roads
    let roads = vec![
        Road::new() // Great Southern Path
            .with_vertex(-5, -7)
            .with_vertex(-5, -137)
            .with_vertex(82, -137)
            .with_vertex(82, -325) // NE corner of Skjor's Bay
            .with_vertex(82, -429) // SE corner of Skjor's Bay
            .with_vertex(-38, -429),
    ];

    // Define buildings
    let buildings = vec![
        Building::new("Last Resort", -10, 3),
        Building::new("Grig's Boulderheim", -2, -10),
        Building::new("Hrogden's Hrarbor", -20, -137),
        Building::new("Timberhold", 82, -137),
        Building::new("Skjor's Landing", -38, -429),
    ];

    // Create underlying image
    let map_radius = get_furthest_point_from_center(&buildings) + 50;
    println!("Map radius: {}", map_radius);
    let mut image = Image::new(map_radius*2, map_radius*2, Rgb::new(255, 255, 255));

    // Draw roads
    for road in roads {
        road.draw(&mut image);
    }

    // Draw buildings
    for building in buildings {
        println!("Drawing {} at {},{}", building.name, building.x, building.y);
        building.draw(&mut image);
    }

    // Save image
    let path = "image.jpg";
    image.save_inferred(path)?;
    println!("Saved to {}", path);

    Ok(())
}
