[RUST Project]

# First step
First of all install rust from https://www.rust-lang.org/tools/install

# Simple setup
Create a `[file].rs`
To compile it simply: `rustc [file].rs`
After that you will see a new file in your folder, to run it simply:
- `./[file]`

# Starting a project
To start a new project `cargo new "project name"`
Your new project will contain:
  - src folder with a `main.rs` file
  - gitignore
  - toml file `Cargo.toml`:
    - Project name and meta
    - dependencies

# Build and Run
Using cargo, we may just `cargo build`
After that we will see a new `target` folder with all the generate assets

To just `run` your project you man `cargo run` that builds and immediately runs it

After your done with your testing and build locally you can just `cargo clean`