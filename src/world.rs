use plates::Plate;
use image::{ImageBuffer, Luma, save_buffer, ColorType};
use noise::{perlin2, Seed};
use rand::distributions::{IndependentSample, Range};
use rand::{StdRng, SeedableRng};

pub struct World {
    pub width: usize,
    pub height: usize,
    pub num_plates: usize,
    plates: Vec<Plate>,
    pub height_field: Vec<Vec<f64>>,
    seed: usize
}

#[derive(Debug)]
pub struct Coord {
    x: usize,
    y: usize
}

impl World {
    pub fn new(width: usize, height: usize, seed: usize, scale_factor: f64,
               num_plates: usize) -> World {
        let mut l_sphere: World = World {width: width, height: height, 
                                         height_field: Vec::with_capacity(height),
                                         seed: seed, num_plates: num_plates,
                                         plates: Vec::with_capacity(num_plates)};
        l_sphere.init_height_field(scale_factor);
        l_sphere.init_plates();
        l_sphere
    }
    
    
    // Initialize the height field with two different scales of perlin noise
    fn init_height_field(&mut self, scale_factor: f64) {
        let t = -scale_factor;
        let s = scale_factor / 3.0;
        let mut p_seed = Seed::new(self.seed as u32);
        for h in 0..self.height {
            let mut row: Vec<f64> = Vec::with_capacity(self.width);
            for w in 0..self.width {
                let mut z: f64 = 0.8 * perlin2(&p_seed, &[w as f64 * s, h as f64 * s]);
                z += 0.2 * perlin2(&p_seed, &[w as f64 * t, h as f64 * t]);
                row.push((z + 1.0)/2.0);
            }
            self.height_field.push(row);
        }
    }
    
    // Initialize the plates
    fn init_plates(&mut self) -> Vec<Coord>{
        // make sure that there are enough points in the world to support the number of 
        // plates
        // generate the seed points for each plate
        let height_range = Range::new(0, self.height);
        let width_range = Range::new(0, self.width);
        let mut seed_points: Vec<Coord> = Vec::with_capacity(self.num_plates);
        let mut rng = StdRng::from_seed(&[self.seed]);
        for i in 0..self.num_plates {
            let x = width_range.ind_sample(&mut rng);
            let y = height_range.ind_sample(&mut rng);
            seed_points.push(Coord { x: x, y:y });
        }
        println!("{:?}", seed_points);
        seed_points
    }
    
    pub fn map_height(&self){
        let mut raw_img: Vec<u8> = Vec::with_capacity(self.width * self.height);
        for row in self.height_field.iter() {
            for elt in row.iter() {
                raw_img.push((elt * 255.0).round() as u8);
            }
        }
        save_buffer("testing.png", &raw_img, self.width as u32, self.height as u32, ColorType::Gray(8));
    }
}

#[test]
fn create_world() {
    let mut world = World::new(2000, 1000, 100, 0.01, 10);
    assert_eq!(world.init_plates().len(), 
}