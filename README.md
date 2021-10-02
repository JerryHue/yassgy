# Yet Another Static Site Generator

Yet Another Static Site Generator, Yassgy for short, is, as it name entails, a static site generator.

## Requirements

While Rust can compile programs into static executables, I have not tried to compile my projects in
that way, yet. Thus, yassgy can only be used in a development environment for now.

The tools you need are:

- `rustc version 1.54 or above`
- `cargo version 1.54 or above`

Both tools can be installed in a bundle using `rustup`, the recommended tool to manage Rust toolchains.
The tool can be found [here](https://www.rust-lang.org/tools/install).

### If you are a Windows user

Note that having installed the [`Microsoft C++ Build Tools`](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
is a requirement for `rustup`.

After you have installed Rust, run the following command in the project's root folder:

```cargo build```

and you should have the binary `yassgy` in the `yassgy/target/debug` directory.

## Usage

Yassgy can be invoked with `yassgy`, if found in your `PATH` variable. If one invokes `yassgy` without
any input or options, yassgy will provide the name and version of the program.

Yassgy accepts the following options:

* `-v` or `--version`: display the version of the build program. It does not accept any arguments.
* `-h` or `--help`: display all the options one may use in Yassgy. It does not accept any arguments.
* `-i` or `--input`: accepts a file name or a folder name as an argument to then generate the static site.
<<<<<<< HEAD
* `-o` or `--output`: accepts a name for the output directory.
=======
* `-l` or `--lang`: accepts a language tag to mark the HTML document with the proper language support.
>>>>>>> issue-9

## Features

Yassgy is currently on development phase, so several features may be added or scrapped in accordance to
the scope of the project. However, the core feature or Yassgy is to generate static webpages from text files.

As of release 0.1, yassgy can:

  - accept a nested structure of files and folders, and such structure will be kept for the output folder.
  - mark the first line as the title of the page, as long as the line is separated by two lines and
  there is more text after.

Stay tuned for other features!

## Licensing

This is a total Open Source tool. In general, you may do whatever you may please with this project, but
if you are interested on the nitty-gritty details, you can head over the LICENSE file, which details
the license this project uses.
