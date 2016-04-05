
extern crate xlsx;
extern crate clap;


use xlsx::read::Reader;
use clap::App;


fn main() {

	let matches = App::new("Xlsx Reader/Writer")
		.version("0.1.0")
		.author("Chris Shelton")
		.args_from_usage(
			"-c, --config=[FILE] 'Sets the JSON config file'
		")
		.get_matches();

	let config = matches.value_of("config").unwrap_or("default.json");
	println!("Value for config: {}", config);

}