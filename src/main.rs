mod valheim_map;
use valheim_map::{Base, Road};

fn build_map() -> valheim_map::Map {
    let mut map = valheim_map::Map::new();

    // Define roads
    map.set_roads(vec![
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
        // Road to Smokehaus
        Road::new()
            .with_vertex(-30, -429) // Intersection with road to Timberhold
            .with_vertex(-30, -545) // North side of bridge
            .with_vertex(-33, -567) // South side of bridge
            .with_vertex(-23, -577)
            .with_vertex(-23, -708), // Smoke sign
    ]);

    // Define buildings
    map.set_bases(vec![
        Base::new("Last Resort", -10, 3),
        Base::new("Grig's Boulderheim", -2, -10),
        Base::new("Hrogden's Hrarbor", -20, -137),
        Base::new("Timberhold", 82, -137),
        Base::new("Skjor's Landing", -38, -429),
        Base::new("Smokehaus", 15, -708), // FIXME
    ]);

    map
}

fn main() -> ril::Result<()> {
    // Build map with data from our world
    let map = build_map();

    // Draw the map into a ril::Image.
    let image = map.draw();

    // Save the image
    let path = "image.jpg";
    image.save_inferred(path)?;
    println!("Saved to {}", path);

    Ok(())
}
