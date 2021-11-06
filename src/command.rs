use std::env::Args;
use std::fs;
use std::option;

#[derive(Debug)]
enum Option {
    Help,
    Version,
    InputPath(String),
    OutputPath(String),
    Language(String),
}

#[derive(Debug)]
pub enum Command {
    PrintHelp,
    PrintVersion,
    GenerateSite {
        input_path: String,
        output_dir_path: String,
        language_tag: String,
    },
}

fn parse_config(config: &str) -> Vec<Option> {
    let mut options = Vec::new();

    let file = fs::File::open(config).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::de::from_reader(file).expect("unable to read to json");

    if let option::Option::Some(file_name) = json["input"].as_str() {
        options.push(Option::InputPath(file_name.to_owned()));
    }
    if let option::Option::Some(output_path) = json["output"].as_str() {
        options.push(Option::OutputPath(output_path.to_owned()));
    }
    if let option::Option::Some(language_tag) = json["lang"].as_str() {
        options.push(Option::Language(language_tag.to_owned()));
    }
    options
}

fn categorize_arg_tokens(mut args: Args) -> Vec<Option> {
    let mut options = Vec::new();

    args.next();
    while let option::Option::Some(arg_token) = args.next() {
        if arg_token == "-v" || arg_token == "--version" {
            options.push(Option::Help);
        } else if arg_token == "-h" || arg_token == "--help" {
            options.push(Option::Version);
        } else if arg_token == "-c" || arg_token == "--config" {
            if let Some(file_config) = args.next() {
                options = parse_config(&file_config);
            }
        } else if arg_token == "-i" || arg_token == "--input" {
            if let option::Option::Some(file_name) = args.next() {
                options.push(Option::InputPath(file_name));
            }
        } else if arg_token == "-o" || arg_token == "--output" {
            if let option::Option::Some(output_path) = args.next() {
                options.push(Option::OutputPath(output_path));
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
            let mut output_dir_path = None;
            let mut language_tag = None;

            for option in &mut opts {
                if let Option::OutputPath(output_path) = option {
                    if output_dir_path == None {
                        output_dir_path = Some(output_path);
                    }
                } else if let Option::Language(tag) = option {
                    if language_tag == None {
                        language_tag = Some(tag);
                    }
                }
            }

            let output_dir_path =
                output_dir_path.unwrap_or_else(|| String::from("dist"));
            let language_tag =
                language_tag.unwrap_or_else(|| String::from("en-CA"));

            Command::GenerateSite {
                input_path,
                output_dir_path,
                language_tag,
            }
        }
        Some(Option::OutputPath(output_dir_path)) => {
            let mut input_path = None;
            let mut language_tag = None;

            for option in &mut opts {
                if let Option::InputPath(input_pathname) = option {
                    if input_path == None {
                        input_path = Some(input_pathname);
                    }
                } else if let Option::Language(tag) = option {
                    if language_tag == None {
                        language_tag = Some(tag);
                    }
                }
            }

            let language_tag =
                language_tag.unwrap_or_else(|| String::from("en-CA"));

            if let Some(input_path) = input_path {
                Command::GenerateSite {
                    input_path,
                    output_dir_path,
                    language_tag,
                }
            } else {
                Command::PrintHelp
            }
        }
        Some(Option::Language(language_tag)) => {
            let mut input_path = None;
            let mut output_dir_path = None;

            for option in opts {
                if let Option::InputPath(input_pathname) = option {
                    if input_path == None {
                        input_path = Some(input_pathname);
                    }
                } else if let Option::OutputPath(output_path) = option {
                    if output_dir_path == None {
                        output_dir_path = Some(output_path);
                    }
                }
            }

            let output_dir_path =
                output_dir_path.unwrap_or_else(|| String::from("dist"));

            if let Some(input_path) = input_path {
                Command::GenerateSite {
                    input_path,
                    output_dir_path,
                    language_tag,
                }
            } else {
                Command::PrintHelp
            }
        }
        None => Command::PrintHelp,
    }
}

pub fn get_command_from_args(args: Args) -> Command {
    parse_opts_as_command(categorize_arg_tokens(args))
}
