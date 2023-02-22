# Composefilter

## What is it?
Composefilter is a simple rust cli tool that filters the output of `docker build` and `docker compose build` and adds
information about the current build step and the name of the currently built container.

## How to compile? 
Ensure you have rust installed and run `cargo build --release` in the root directory of the project, which produces a fully optimized binary.  
If you just want to test the tool or compile with debug symbols enabled, you can run `cargo build` which produces the unoptimized binary with debuginfo.

## How to use?
You can use the tool by running the binary, which is located in `target/release/composefilter` or `target/debug/composefilter` depending on the build type.  
Execute the binary like so `./composefilter -- <command>`, where `<command>` is the command you want to execute.