use image::*;
use std::fs;

fn main() {

	let files = vec!["test-a", "test-b"];
	let filters = vec![("Triangle" ,FilterType::Triangle), ("CatmullRom" ,FilterType::CatmullRom), ("Gaussian" ,FilterType::Gaussian), ("Lanczos3" ,FilterType::Lanczos3)];
	fs::create_dir("./result").unwrap();
	for file in &files {
		for filter in &filters {
			let img = image::open(format!("{}.png", file)).unwrap();
	    	let (width, height) = img.dimensions();
	    	let resized = img.resize_exact(width-25, height-25, filter.1);

	    	// Increase contrast and brightness so we can see the generated artifacts
	    	let resized = if *file=="test-a" {
	    		resized.brighten(115).adjust_contrast(250.0)
	    	} else {
	    		resized.brighten(105).adjust_contrast(250.0)
	    	};

	    	resized.save(format!("./result/{}-{}.png", file, filter.0)).unwrap();
	    }
	}
}