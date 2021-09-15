mod html_page;
use html_page::HtmlPage;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::env;


#[derive(Debug)]
enum Command {
    Input { file_name: String },
    Version,
    Help
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
		    command = Option::Some(Command::Input{file_name});
		}
	    }
	}

	command
    }
}

fn main() -> std::io::Result<()> {
    let file_name = "test.txt";

    let page = HtmlPage::from(file_name).unwrap();

    page.write_to_file("test.html")?;

    Ok(())
}
