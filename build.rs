use clap::CommandFactory;
use clap_complete::generate_to;
use clap_mangen::Man;
use std::fs::{create_dir, write};
use std::io;

include!("src/cli.rs");

fn main() -> io::Result<()> {

    let out_dir = if cfg!(debug_assertions) {
        std::path::PathBuf::from("target/debug")
    } else {
        std::path::PathBuf::from("target/release")
    };

    let comp_dir = out_dir.join("completions");

    if let Err(e) = create_dir(&comp_dir) {
       println!("Failed to create completions directory: {}", e);
    }


    println!("OUT_DIR: {:?}", out_dir);

    let mut cli = Cli::command();

    let man = Man::new(cli.clone());

    let mut buffer: Vec<u8> = Vec::new();

    man.render(&mut buffer)?;

    write(out_dir.join("leech.1"), buffer)?;

    for shell in AvailableShells::value_variants(){
        generate_to(*shell, &mut cli, "leech", &comp_dir)?;
    }

    Ok(())
}