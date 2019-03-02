use image::{GenericImageView, FilterType};

fn main() {

	let files = vec!["test-a", "test-b"];
	let filters = vec![("Triangle" ,FilterType::Triangle), ("CatmullRom" ,FilterType::CatmullRom), ("Gaussian" ,FilterType::Gaussian), ("Lanczos3" ,FilterType::Lanczos3)];

	for file in &files {
		for filter in &filters {
			let img = image::open(format!("{}.png", file)).unwrap();
	    	let (width, height) = img.dimensions();
	    	let resized = img.resize_exact(width-25, height-25, filter.1);
	    	resized.save(format!("./result/{}-{}.png", file, filter.0)).unwrap();
	    }
	}
}