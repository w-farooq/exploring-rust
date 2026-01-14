/*
Create a new `about_me` project with the `cargo new` command.

Using the `println!` macro, output 3 sentences about yourself.
Feel free to invoke the macro multiple times.

From the Terminal, compile the `main.rs` file inside the `src`
folder with the Rust compiler, then manually run the executable.

From the Terminal, compile the project with the Cargo tool, then
manually run the executable.

From the Terminal, compile and run the project with a single
Cargo command.

Check your program for errors with `cargo check`.

Add a comment at the top of your source code explaining how to
compile the program for new Rust developers.

Add some spaces and line breaks to the code so that it is formatted
in an ugly manner. From the Terminal, style the code with the
`cargo fmt` command.

Replace the `println!` macro with `print!`. What happens?
*/

/*
    How to compile rust code: 
    1. rustc filename.ext will compile a single file to run it .\filename.exe
    2. cargo build builds all the file into a single binary. run from target\debug


*/

fn main() {
    print!("My name is Waqar Farooq. ");
    print!("I am a Robotics Engineer at Amazon,");
    print!("My net worth is 365.5 million.");
}
