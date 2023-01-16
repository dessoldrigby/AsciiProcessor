use image::{GenericImageView, DynamicImage};

pub struct SymbImage {
    grid: Vec<Vec<char>>,
    width: u32,
    height: u32
}

impl SymbImage {
    pub fn new(ingrid: Vec<Vec<char>>) -> Self {
        let w = ingrid.len() as u32;
        let h = ingrid[0].len() as u32;

        return Self {
            grid: (ingrid),
            width: (w),
            height: (h)
        };
    }

    pub fn dimenstions(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    pub fn get_grid(&self) -> Vec<Vec<char>> {
        return self.grid.clone();
    } 
}



pub struct ImageProcessor{
    buffered_int: u8,
    char_set: Vec<char>,
}

impl ImageProcessor {
    pub fn new(set: Vec<char>) -> Self {
        return Self { 
            buffered_int: (255 / (set.len() as u8)), 
            char_set: (set) 
        };
    }    
    
    fn get_char(&self, intensity: u8) -> char {
        let idx =  intensity / self.buffered_int;

        // if idx == self.char_set.len() as u8 { idx -= 1; }
        // return self.char_set[idx as usize];

        if idx == self.char_set.len() as u8 
            { self.char_set[(idx - 1) as usize] } 
        else 
            { self.char_set[idx as usize] }
    }

    fn load_image(dir: &str) -> DynamicImage {
        let image = image::open(dir);
        
        return match image {
            Ok(img) => img,
            Err(error) => panic!("Problem loading the image: {:?}", error),
        };
    }

    pub fn process_image_ds(&self, dir: &str, wscale: u32, hscale: u32) -> SymbImage {
        let img = ImageProcessor::load_image(dir);
        let (w, h) = img.dimensions();
        let (uw, uh) = ((w / wscale) as usize, (h / hscale) as usize);

        let mut grid = vec!(vec!(' '; uw); uh);

        for y in 0..(h - hscale) {
            for x in 0..(w - wscale) {
                if y % hscale != 0 || x % wscale != 0 { continue; }

                let px = img.get_pixel(x, y);
                let mut intent = px[0] / 3 + px[1] / 3 + px[2] / 3;

                if px[3] == 0 { intent = 0; }

                grid[(y / hscale) as usize][(x / wscale) as usize] = 
                    self.get_char(intent);
            }
        }

        return SymbImage::new(grid);
    }

    pub fn process_image_ss(&self, dir: &str, scale: u32) -> SymbImage {
        return self.process_image_ds(dir, scale, scale * 2);
    }

    pub fn print_image(&self, grid_img: SymbImage) {
        for x in grid_img.get_grid() {
            for y in x {
                print!("{}", y);
            }

            println!();
        }
    }
}
