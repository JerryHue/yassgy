use std::env::Args;
use std::option;

#[derive(Debug)]
enum Option {
    Help,
    Version,
    InputPath(String),
    OutputPath(String),
}

#[derive(Debug)]
pub enum Command {
    PrintHelp,
    PrintVersion,
    GenerateSite {
        input_path: String,
        output_dir_path: String,
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
        } else if arg_token == "-o" || arg_token == "--output" {
            if let option::Option::Some(output_path) = args.next() {
                options.push(Option::OutputPath(output_path));
            }
        }
    }

    options
}

fn parse_opts_as_command(opts: Vec<Option>) -> Command {
    if opts.len() == 0 {
        return Command::PrintHelp;
    }

    let mut opts = opts.into_iter();

    let first_option = opts.next();

    match first_option {
        Some(Option::Help) => Command::PrintHelp,
        Some(Option::Version) => Command::PrintVersion,
        Some(Option::InputPath(input_path)) => {
            let mut output_dir_path = String::from("dist");

            while let Some(option) = opts.next() {
                if let Option::OutputPath(output_path) = option {
                    output_dir_path = output_path;
                    break;
                }
            }

            Command::GenerateSite {
                input_path,
                output_dir_path,
            }
        }
        Some(Option::OutputPath(output_dir_path)) => {
            while let Some(option) = opts.next() {
                if let Option::InputPath(input_path) = option {
                    return Command::GenerateSite {
                        input_path,
                        output_dir_path,
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
