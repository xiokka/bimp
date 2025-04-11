use rustic_bitmap::*;
use std::fs::File;
use std::env;
use std::io::Read;
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        eprintln!("Invalid arguments. Usage: bimp [-e|-d] [input] [output]");
        return Ok(());
    }
    let operation = &args[1];
    let input = &args[2];
    let output = &args[3];

    if operation != "-e" && operation != "-d" {
        eprintln!("Invalid arguments. Usage: bimp [-e|-d] [input] [output]");
        return Ok(());
    }

    if operation == "-e" {
	let width = 500;
	let mut binary_data_file = File::open(input)?;
        let mut binary_data = Vec::new();
        binary_data_file.read_to_end(&mut binary_data)?;
	println!("Original binary file size: {} bytes", binary_data.len());

	// Zero-pad the binary data to make its length divisible by 3 (makes it easier to divide into 3-byte triplets later on just trust me)
	let padding_len = (3 - (binary_data.len() % 3)) % 3;  // Calculate how many bytes to pad
	binary_data.extend(vec![0; padding_len]);  // Pad with zeros
        println!("Zero-padded binary file size: {} bytes", binary_data.len());

	let height: usize = (binary_data.len() + width * 3 - 1) / (width * 3);
	println!("Height: {} pixels", height);
	println!("Width: {} pixels", width);
	let mut bmp:Vec<u8> = Vec::<u8>::new_bitmap(width.try_into().unwrap(), height.try_into().unwrap(), 24);
	for i in 0..height {
		for j in 0..width {
			let walk = (i * width + j) * 3;  // Calculate the starting index for the pixel
			if walk + 2 < binary_data.len() {
				let color = Rgb {r: binary_data[walk + 2], g: binary_data[walk + 1], b: binary_data[walk]};
                		let position: Point = Point {x: j as u32, y: i as u32};
				bmp.draw_point(&position, &color);
			}
		}
    	}
	let mut file_output = File::create(output).unwrap();
	file_output.write_all(&bmp).unwrap();
    }

    if operation == "-d" {
        let mut bitmap_file = File::open(input)?;
        let mut bitmap = Vec::new();
        bitmap_file.read_to_end(&mut bitmap)?;
	let pixel_array_offset = bitmap.get_pixel_array_offset();
	let slice = &bitmap[pixel_array_offset as usize..bitmap.len()].to_vec();
        let mut file_output = File::create(output).unwrap();
        file_output.write_all(&slice).unwrap();
    }
    return Ok(());
}
