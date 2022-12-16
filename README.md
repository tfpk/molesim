# Molesim

Molesim is a small utility which runs simulations of gas molecules.
It takes advantage of Rust's fearless concurrency and reasonable GUI 
to support at least 10000 molecules being simulated at once, with good
framerates.


## How To Use

1. [Install Rust](https://www.rust-lang.org/tools/install). 
2. Download the repository (easiest way is using the "code > Download Zip button at the 
top of the screen)
3. Run `cargo run --release`
4. Change parameters of the simulation with the variables in `src/lib.rs`
