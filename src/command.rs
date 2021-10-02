use std::env::Args;
use std::option;

#[derive(Debug)]
enum Option {
    Help,
    Version,
    InputPath(String),
    Language(String),
}

#[derive(Debug)]
pub enum Command {
    PrintHelp,
    PrintVersion,
    GenerateSite {
        input_path: String,
        language_tag: String,
    },
}

fn categorize_arg_tokens(mut args: Args) -> Vec<Option> {
    let mut options = Vec::new();

    args.next();
    while let option::Option::Some(arg_token) = args.next() {
        if arg_token == "-v" || arg_token == "--version" {
            options.push(Option::Help);
        } else if arg_token == "-h" || arg_token == "--help" {
            options.push(Option::Version);
        } else if arg_token == "-i" || arg_token == "--input" {
            if let option::Option::Some(file_name) = args.next() {
                options.push(Option::InputPath(file_name));
            }
        } else if arg_token == "-l" || arg_token == "--lang" {
            if let option::Option::Some(language_tag) = args.next() {
                options.push(Option::Language(language_tag));
            }
        }
    }

    options
}

fn parse_opts_as_command(opts: Vec<Option>) -> Command {
    if opts.is_empty() {
        return Command::PrintHelp;
    }

    let mut opts = opts.into_iter();

    let first_option = opts.next();

    match first_option {
        Some(Option::Help) => Command::PrintHelp,
        Some(Option::Version) => Command::PrintVersion,
        Some(Option::InputPath(input_path)) => {
            let mut language_tag = String::from("en-CA");

            while let Some(option) = opts.next() {
                if let Option::Language(tag) = option {
                    language_tag = tag;
                    break;
                }
            }

            Command::GenerateSite {
                input_path,
                language_tag,
            }
        }
        Some(Option::Language(language_tag)) => {
            while let Some(option) = opts.next() {
                if let Option::InputPath(input_path) = option {
                    return Command::GenerateSite {
                        input_path,
                        language_tag,
                    };
                }
            }

            Command::PrintHelp
        }
        None => Command::PrintHelp,
    }
}

pub fn get_command_from_args(args: Args) -> Command {
    parse_opts_as_command(categorize_arg_tokens(args))
}
