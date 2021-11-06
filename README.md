# Yet Another Static Site Generator

Yet Another Static Site Generator, Yassgy for short, is, as it name entails, a static site generator.

## Setting up and development

To know more about setting up a development environment, check the 
[contribution guidelines](./CONTRIBUTING.md).

## Usage

Yassgy can be invoked with `yassgy`, if found in your `PATH` variable. If one invokes `yassgy` without
any input or options, yassgy will provide the name and version of the program.

Yassgy accepts the following options:

* `-v` or `--version`: display the version of the build program. It does not accept any arguments.
* `-h` or `--help`: display all the options one may use in Yassgy. It does not accept any arguments.
* `-i` or `--input`: accepts a file name or a folder name as an argument to then generate the static site.
* `-o` or `--output`: accepts a name for the output directory.
* `-l` or `--lang`: accepts a language tag to mark the HTML document with the proper language support.
* `-c` or `--config`: accepts a JSON file including the properties for input, lang, and output. 


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
