mod html_page;
mod static_site;
use static_site::StaticSite;
use std::env;
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

    if input_path.is_dir() {
	let site = StaticSite::from_directory(input_path)?;
	site.create(path::Path::new("dist"))?;
    } else if input_path.is_file() {
	let site = StaticSite::from_file(input_path);
	site.create(path::Path::new("dist"))?;
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
    println!("\t-i <PATH, --input <PATH>\tGenerate HTML files from TXT files. PATH can be a path to an individual file, or to a folder");
    println!("\n");
}

fn print_version() {
	println!("{} version {}", PKG_NAME, PKG_VERSION);
}
