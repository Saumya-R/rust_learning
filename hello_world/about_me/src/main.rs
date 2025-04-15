/*
    1.Create a new `about_me` project with the `cargo new` command.
    saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_worldmain $ cargo new about_me
    Creating binary (application) `about_me` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_worldmain $ cd about_me/
    
    2.Using the `println!` macro, output 3 sentences about yourself.
    Feel free to invoke the macro multiple times.

    3.From the Terminal, compile the `main.rs` file inside the `src`
    folder with the Rust compiler, then manually run the executable.
    saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cd src/
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_me/srcmain $ rustc main.rs
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_me/srcmain $ ./main
Hello, world!
I am a Rust Developer !
Rust is looking very difficult as of now ! :(
   
    4.From the Terminal, compile the project with the Cargo tool, then
    manually run the executable.
     saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_me/srcmain $ cd ..
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo build
   Compiling about_me v0.1.0 (/home/saumya/Documents/Learning/rust_learning/hello_world/about_me)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ ./target/debug/about_me
Hello, world!
I am a Rust Developer !
Rust is looking very difficult as of now ! :(
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo build --release
   Compiling about_me v0.1.0 (/home/saumya/Documents/Learning/rust_learning/hello_world/about_me)
    Finished `release` profile [optimized] target(s) in 0.12s
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ ./target/release/about_me
Hello, world!
I am a Rust Developer !
Rust is looking very difficult as of now ! :(
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo clean
     Removed 30 files, 8.5MiB total
    
    5.From the Terminal, compile and run the project with a single
    Cargo command.
        saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo run
   Compiling about_me v0.1.0 (/home/saumya/Documents/Learning/rust_learning/hello_world/about_me)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/about_me`
Hello, world!
I am a Rust Developer !
Rust is looking very difficult as of now ! :(
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo run --quiet
Hello, world!
I am a Rust Developer !
Rust is looking very difficult as of now ! :(
    
    6. Check your program for errors with `cargo check`.
saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo check
    Checking about_me v0.1.0 (/home/saumya/Documents/Learning/rust_learning/hello_world/about_me)
error: expected `;`, found `println`
  --> src/main.rs:70:30
   |
70 |     println!("Hello, world!")
   |                              ^ help: add `;` here
71 |     println!("I am a Rust Developer ! ");
   |     ------- unexpected token

error: could not compile `about_me` (bin "about_me") due to 1 previous error
    
    7.Add a comment at the top of your source code explaining how to
    compile the program for new Rust developers.

    8. Add some spaces and line breaks to the code so that it is formatted
    in an ugly manner. From the Terminal, style the code with the
    `cargo fmt` command.

    9.Replace the `println!` macro with `print!`. What happens?
    saumya@saumya-VB:~/Documents/Learning/rust_learning/hello_world/about_memain $ cargo run --quiet
Hello, world!
I am a Rust Developer ! Rust is looking very difficult as of now ! :(
    */

fn main() {
    println!("Hello, world!");

    print!("I am a Rust Developer ! ");
    println!("Rust is looking very difficult as of now ! :(");
}
