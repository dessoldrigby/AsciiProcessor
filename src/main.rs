use ascii_processor::{ImageProcessor};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut scale: i32 = 12;

    if args.len() < 2 {
        panic!("{}", usage());
    }

    if args.len() == 3 {
        scale = args[2].parse::<i32>().unwrap();
    }

    let path = args[1].as_str();

    let imgp = 
    ImageProcessor::new(vec![' ', '.', '-', '~', '=', '+', '!', '?', 'W', 'A', '@']);

    let pimg = imgp.process_image_ss(path, scale as u32);
    imgp.print_image(pimg);
}

fn usage() -> &'static str { 
    "./ascii_processr [image path] [(opt, def = 12) scale]" 
}
