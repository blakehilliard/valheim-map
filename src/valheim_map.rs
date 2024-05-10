use ril::{Draw, Image, Line, Polygon, Rgb};

pub struct Map {
    pub islands: Vec<Island>,
    pub bases: Vec<Base>,
    pub roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            islands: Vec::<Island>::new(),
            bases: Vec::<Base>::new(),
            roads: Vec::<Road>::new(),
        }
    }

    pub fn set_islands(&mut self, islands: impl IntoIterator<Item = Island>) {
        self.islands = islands.into_iter().collect();
    }

    pub fn set_bases(&mut self, bases: impl IntoIterator<Item = Base>) {
        self.bases = bases.into_iter().collect();
    }

    pub fn set_roads(&mut self, roads: impl IntoIterator<Item = Road>) {
        self.roads = roads.into_iter().collect();
    }

    pub fn draw(&self) -> Image<Rgb> {
        // Create underlying image
        let water_rgb = Rgb::new(116, 204, 244);
        let map_radius = self.get_furthest_point_from_center() + 50;
        println!("Map radius: {}", map_radius);
        let mut image = Image::new(map_radius*2, map_radius*2, water_rgb);

        // Draw islands
        for island in &self.islands {
            island.draw(&mut image);
        }

        // Draw roads
        for road in &self.roads {
            road.draw(&mut image);
        }

        // Draw buildings
        for base in &self.bases {
            println!("Drawing {} at {},{}", base.name, base.x, base.y);
            base.draw(&mut image);
        }

        image
    }

    fn get_furthest_point_from_center(&self) -> u32 {
        let mut biggest: i64 = 0;
        for base in &self.bases {
            let num = std::cmp::max(base.x.abs(), base.y.abs());
            if num > biggest {
                biggest = num;
            }
        }
        biggest as u32
    }
}

pub struct Island {
    pub name: String,
    pub vertices: Vec<(i64, i64)>,
}

impl Island {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            vertices: Vec::<(i64, i64)>::new(),
        }
    }

    pub fn with_vertex(mut self, x: i64, y: i64) -> Self {
        self.vertices.push((x, y));
        self
    }

    pub fn draw(&self, image: &mut Image<Rgb>) {
        let color = Rgb::new(37, 255, 0);
        let mut poly = Polygon::new().
            with_fill(color);
        for vertex in &self.vertices {
            let coords = map_coords_to_image_coords(image, vertex.0, vertex.1);
            poly.push_vertex(coords.0, coords.1);
        }
        poly.draw(image);
    }
}

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
    pub vertices: Vec<(i64, i64)>,
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