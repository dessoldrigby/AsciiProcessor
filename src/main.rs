use ascii_processor::{ImageProcessor};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut path = "./Veronika.jpg";
    let mut scale = 12;

    if args.len() as i32 > 1 {
        path = args[1].as_str();
    }
    if args.len() as i32 > 2 {
        let us = args[2].parse::<i32>().unwrap();
        if us >= 0 {
            scale = us;
        }
    }

    let imgp = 
    ImageProcessor::new(vec![' ', '.', '-', '~', '=', '+', '!', '?', 'W', 'A', '@']);

    let pimg = imgp.process_image_ss(path, scale as u32);
    imgp.print_image(pimg);
}
