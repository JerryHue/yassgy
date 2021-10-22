mod command;
use command::Command;
use std::env;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> std::io::Result<()> {
    let command = command::get_command_from_args(env::args());

    let site_options = match command {
        Command::PrintHelp => {
            print_help();
            None
        }
        Command::PrintVersion => {
            print_version();
            None
        }
        Command::GenerateSite {
            input_path,
            output_dir_path,
            language_tag,
        } => Some((input_path, output_dir_path, language_tag)),
    };

    if site_options == None {
        return Ok(());
    }

    let (file_name, output_dir, language_tag) = site_options.unwrap();
    yassgy::generate(file_name, output_dir, language_tag)
}

fn print_help() {
    print_version();

    println!("USAGE:");
    println!("\t{} [OPTIONS]\n", PKG_NAME);

    println!("OPTIONS:");
    println!("\t-v, --version\t\t\tPrint the version of the compiled package");
    println!("\t-h, --help\t\t\tPrint this screen");
    println!("\t-i <PATH>, --input <PATH>\tGenerate HTML files from TXT files. PATH can be a path to an individual file, or to a folder");
    println!("\t-o <PATH>, --output <PATH>\tName the output directory. Default is './dist'.");
    println!("\t-c <PATH>, --config <PATH>\tPass in config JSON file with properties for input, output, and lang, i.e., config.JSON.");
    println!("\t-l <TAG>, --lang <TAG>\t\tMark HTML document with language TAG, i.e., en-CA.");
    println!("\n");
}

fn print_version() {
    println!("{} version {}", PKG_NAME, PKG_VERSION);
}
