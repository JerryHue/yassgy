# Contributing to `yassgy`

## Setting up the project

### Prerequisites

You would need to download the following:

- `rustc` version 1.54.0 or greater,
- `cargo` version 1.54.0 or greater,
- `rustfmt` version 1.4.37 or greater,
- `rust-clippy` version 0.1.55 or greater.

This also entails that you'd need to download the dependencies that Rust
requires. Such dependencies may differ if you are developing on Linux, or Windows.

All tools can be installed in a bundle using `rustup`, the recommended tool 
to manage Rust toolchains. The tool can be found
[here](https://www.rust-lang.org/tools/install).

Note: for Windows users, installing `rustup` requires
[`Microsoft C++ Build Tools`](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
These can be download either by downloading Visual Studio and installing the
C++ SDK, or just downloading the build tools separately.

### Building the project

After you have cloned the project, you may invoke the following command at
the root directory of the project:

```bash
cargo build
```

After building is finished, you will find a `target` directory. The binary
executable is found at `target/debug/yassgy`.

## Starting to contribute

If you want to contribute, you can always file an issue, or make a PR.

### Filing an issue

`yassgy` is just starting, so there is a lot of room to improve upon it!

If you are filing a bug, please describe the steps you took to replicate. If
you cannot replicate, we won't be able to know how to fix it.

If you are filing an improvement, always describe what you would like to
improve, and how you would like to improve it. We may also use the issue
to discuss how this improvement can be carried out.

If you are filing a new feature, describe what the feature is supposed to
be like, and then we can discuss a plan on how to implement it.

### Making a PR

To create a PR, you may fork your project and add that as your `origin`
remote repo reference on your local git repository. If you already
this repo as your `origin` remote, you may rename it to `upstream`.

All changes must be done on a separate topic branch that addresses the
issue at hand (or part of an issue), and a PR merge must be done
from the topic branch on your fork to the upstream's master branch.

If your PR addresses the issue satisfactorily, a review process will
start. If your PR does not address the issue, then your PR will be closed.

You may also create a draft PR if you would like to provide some progress
to your PR, and to also provide some discussion with respect to your PR.

#### Formatting and Linting

All the code added must adhere to the coding style set by `rustfmt`.
Before committing your code, you may format you code with the following command:

```bash
cargo fmt
```

If you prefer, you may set a configuration in your editor of choice
to run this command every time you save a file.

Also, the code must be run through `clippy`, Rust's official linter.
You may run the linter with the following command:

```bash
cargo clippy
```

However, since `clippy` produces a compilation of the project, I wouldn't
recommend running every time you save a file, since it would slow down
your productivity.