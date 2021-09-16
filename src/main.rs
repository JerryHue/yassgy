mod html_page;
use html_page::HtmlPage;
use std::env;
use std::fs;
use std::path;

enum Command {
	Input { file_name: String },
	Version,
	Help,
}

impl Command {
	fn from(mut args: env::Args) -> Option<Command> {
		// Skip the first argument, which may not be the command name. See env::Args docs.
		args.next();

		let mut command = Option::None;

		while let Option::Some(arg_token) = args.next() {
			if arg_token == "-v" || arg_token == "--version" {
				command = Option::Some(Command::Version);
			} else if arg_token == "-h" || arg_token == "--help" {
				command = Option::Some(Command::Help);
			} else if arg_token == "-i" || arg_token == "--input" {
				if let Option::Some(file_name) = args.next() {
					command = Option::Some(Command::Input { file_name });
				}
			}
		}

		command
	}
}

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
	let command = Command::from(env::args());

	let file_name = match command {
		None | Some(Command::Help) => {
			print_help();
			None
		}
		Some(Command::Version) => {
			print_version();
			None
		}
		Some(Command::Input { file_name }) => Some(file_name),
	};

	if file_name == None {
		return Ok(());
	}

	let file_name = file_name.unwrap();
	let input_path = path::Path::new(&file_name);

	let path_to_input_files: Vec<path::PathBuf> = if input_path.is_dir() {
		let input_dir_entries = input_path.read_dir()?;

		input_dir_entries
			.map(|d_entry| {
				d_entry.map_or_else(
					|e| {
						println!(
							"There was an error when accessing a file. See below for details.\n{}",
							e
						);
						None
					},
					|v| Some(v.path()),
				)
			})
			.filter_map(|p| p)
			.filter(|p| p.is_file())
			.collect()
	} else {
		vec![input_path.to_path_buf()]
	};

	let output_folder_path = path::Path::new("dist");

	if output_folder_path.exists() {
		fs::remove_dir_all(output_folder_path)?;
	}

	let dir_builder = fs::DirBuilder::new();
	dir_builder.create(output_folder_path)?;

	for path_to_input in path_to_input_files {
		let page = HtmlPage::from(&path_to_input)?;

		let mut output_file_name: path::PathBuf =
			output_folder_path.join(path_to_input.file_stem().unwrap());

		output_file_name.set_extension("html");

		page.write_to_file(output_file_name)?;
	}

	Ok(())
}

fn print_help() {
	print_version();

	println!("USAGE:");
	println!("\t{} [OPTIONS]\n", PKG_NAME);

	println!("OPTIONS:");
	println!("\t-v, --version\t\t\tPrint the version of the compiled package");
	println!("\t-h, --help\t\t\tPrint this screen");
	println!("\t-i <FILENAME, --input <FILENAME>\tGenerate HTML files from TXT files. FILENAME can be a path to an individual file, or to a folder");
	println!("\n");
}

fn print_version() {
	println!("{} version {}", PKG_NAME, PKG_VERSION);
}
