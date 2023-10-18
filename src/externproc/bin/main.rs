use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let data = "Hello, world!";
    let mut f = File::create("hello.txt").expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");

    // Open the file in Notepad
    Command::new("notepad.exe").arg("hello.txt").spawn().expect("Notepad failed to start");
}