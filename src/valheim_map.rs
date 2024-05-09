use ril::{Draw, Image, Line, Polygon, Rgb};

#[derive(Debug)]
pub struct Base {
    pub name: String,
    pub x: i64,
    pub y: i64,
}

impl Base {
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
pub struct Road {
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