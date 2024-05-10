mod valheim_map;

fn main() -> ril::Result<()> {
    // Build map with data from our world
    let map = valheim_map::Map::from_toml_file("map.toml");

    // Draw the map into a ril::Image.
    let image = map.draw();

    // Save the image
    let path = "image.jpg";
    image.save_inferred(path)?;
    println!("Saved to {}", path);

    Ok(())
}
