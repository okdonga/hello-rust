# Make a guessing game

How to run:

```
$ cd guessing_game
$ cargo run
$ quit // to exit the program
```

# New concepts covered:

## `Crates`

A crate is a collection of Rust source code files. You can use crates to get more functionality. For example, the `rand` crate contains code intended to be used in other programs.

Cargo fetches the latest versions of everything from the registry, which is a copy of data from [Crates.io](https://crates.io/). `Crates.io` is where people in the Rust ecosystem post their open source Rust projects for others to use.

When you add `rand` as a dependency in `Cargo.toml`, Cargo also grabbed other crates that rand depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

```
$ cargo build
>  Updating crates.io index
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
  Downloaded getrandom v0.2.3
  Downloaded ppv-lite86 v0.2.15
  Downloaded cfg-if v1.0.0
  Downloaded rand v0.8.4
  Downloaded libc v0.2.105
  Downloaded 7 crates (734.9 KB) in 1.01s
   Compiling libc v0.2.105
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.15
   Compiling getrandom v0.2.3
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.4
   Compiling guessing_game v0.1.0 (/Users/user/Documents/PERSONAL/rust-project/2.Programming a Guessing Game/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 24.47s

```

## `Cargo.lock`

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the `Cargo.lock` file. When you build your project in the future, Cargo will see that the `Cargo.lock` file exists and use the versions specified there rather than doing all the work of figuring out versions again.

## `cargo update`

This will ignore the Cargo.lock file and update the dependencies to the latest versions accroding to the specifications in `Cargo.toml`. Then, this will make updates to the `Cargo.lock` file.

## `cargo doc --open`

This will build documentation provided by all of your dependencies locally and open it in your browser. From here, you can look up detailed methods and how to use utilities.
