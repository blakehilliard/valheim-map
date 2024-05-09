use ril::{Image, Rgb};
mod valheim_map;
use valheim_map::{Base, Road};

fn get_furthest_point_from_center(bases: &[Base]) -> u32 {
    let mut biggest: i64 = 0;
    for base in bases {
        let num = std::cmp::max(base.x.abs(), base.y.abs());
        if num > biggest {
            biggest = num;
        }
    }
    biggest as u32
}

fn main() -> ril::Result<()> {
    // Define roads
    let roads = vec![
        // Great Northern Path
        Road::new()
            .with_vertex(0, 7)
            .with_vertex(0, 263),
        // Great Eastern Path
        Road::new()
            .with_vertex(7, 0)
            .with_vertex(220, 0),
        // Great Southern Path
        Road::new()
            .with_vertex(-5, -7)
            .with_vertex(-5, -137)
            .with_vertex(82, -137)
            .with_vertex(82, -325) // NE corner of Skjor's Bay
            .with_vertex(82, -429) // SE corner of Skjor's Bay
            .with_vertex(-38, -429),
        // Great Western Path
        Road::new()
            .with_vertex(-7, 0)
            .with_vertex(-264, 0),
    ];

    // Define buildings
    let bases = vec![
        Base::new("Last Resort", -10, 3),
        Base::new("Grig's Boulderheim", -2, -10),
        Base::new("Hrogden's Hrarbor", -20, -137),
        Base::new("Timberhold", 82, -137),
        Base::new("Skjor's Landing", -38, -429),
    ];

    // Create underlying image
    let map_radius = get_furthest_point_from_center(&bases) + 50;
    println!("Map radius: {}", map_radius);
    let mut image = Image::new(map_radius*2, map_radius*2, Rgb::new(255, 255, 255));

    // Draw roads
    for road in roads {
        road.draw(&mut image);
    }

    // Draw buildings
    for base in bases {
        println!("Drawing {} at {},{}", base.name, base.x, base.y);
        base.draw(&mut image);
    }

    // Save image
    let path = "image.jpg";
    image.save_inferred(path)?;
    println!("Saved to {}", path);

    Ok(())
}
