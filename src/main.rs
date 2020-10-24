use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = env::args().skip(1).next().unwrap_or_else(|| {
        println!("Please provide a filename!");
        process::exit(1);
    });
    let mut f = File::open(&file_name)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let mut wav = lib::Wav::new(&buffer);
    wav.double_speed();

    let mut new_file_name = adjust_file_path(&file_name[..]);

    new_file_name.push_str("_quikka.wav");
    wav.write_to_file(&new_file_name)?;

    Ok(())
}

fn adjust_file_path(file_name: &str) -> String {
    String::from(Path::new(file_name).file_stem().unwrap().to_str().unwrap())
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let buffers: Vec<Vec<u8>> = env::args()
//         .skip(1)
//         .map(|file_name| {
//             let mut f = File::open(&file_name).unwrap();
//             let mut buffer = Vec::new();
//             f.read_to_end(&mut buffer).unwrap();
//             buffer
//         })
//         .collect();

//     let mut wav1 = Wav::new(&buffers[0]);
//     let mut wav2 = Wav::new(&buffers[0]);
//     wav1.intersperse(wav2);

//     // let mut new_file_name = adjust_file_path(&file_name[..]);

//     // new_file_name.push_str("_quikka.wav");
//     wav1.write_to_file("mix.wav")?;

//     Ok(())
// }
