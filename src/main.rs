mod command;
mod html_page;
mod static_site;
use command::Command;
use static_site::StaticSite;
use std::env;
use std::path;

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
            language_tag,
        } => Some((input_path, language_tag)),
    };

    if site_options == None {
        return Ok(());
    }

    let (file_name, language_tag) = site_options.unwrap();
    let input_path = path::Path::new(&file_name);

    if input_path.is_dir() {
        let site = StaticSite::from_directory(input_path)?;
        site.create(path::Path::new("dist"), &language_tag)?;
    } else if input_path.is_file() {
        let site = StaticSite::from_file(input_path);
        site.create(path::Path::new("dist"), &language_tag)?;
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
    println!("\t-i <PATH>, --input <PATH>\tGenerate HTML files from TXT files. PATH can be a path to an individual file, or to a folder");
    println!("\t-l <TAG>, --lang <TAG>\t\tMark HTML document with language TAG, i.e., en-CA.");
    println!("\n");
}

fn print_version() {
    println!("{} version {}", PKG_NAME, PKG_VERSION);
}
