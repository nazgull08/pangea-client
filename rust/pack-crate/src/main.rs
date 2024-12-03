use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{exit, Command};

fn modify_readme() -> io::Result<()> {
    let file_path = "README.md";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines = reader.lines().take(28).collect::<Result<Vec<_>, _>>()?;
    let mut file = File::create(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = modify_readme() {
        eprintln!("Failed to modify README.md: {}", err);
        exit(1);
    }

    // package
    let package_status = Command::new("cargo")
        .arg("package")
        .arg("--allow-dirty")
        .status()
        .expect("Failed to execute cargo package");

    if !package_status.success() {
        eprintln!("Failed to package the crate");
        exit(1);
    }

    // restore readme
    let restore_status = Command::new("git")
        .args(&["restore", "README.md"])
        .status()
        .expect("Failed to execute git restore");

    if !restore_status.success() {
        eprintln!("Failed to restore Readme.md");
        exit(1);
    }
}
