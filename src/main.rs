use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;
mod io;
mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    let operations: [io::Operation; 1] = [io::Operation::new(
        "2x",
        "Apply 'chipmunk' transformation to wav, doubling speed and raising pitch",
        io::Performance::DoubleSpeed,
    )];
    // define operations
    // collect args
    // if no arg, open a complex operation
    // if arg, check collected operations for one matching name
    // if not found, abort
    // if found, iterate to next arg - file to use
    // if no arg, ask for one
    // if found, resolve process
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mode: io::Mode = match args.get(0) {
        None => io::Mode::Complex,
        Some(arg) => {
            let op: &io::Operation<'_> = operations.iter().find(|op| op.name() == arg).unwrap();
            io::Mode::Simple(op)
        }
    };
    match mode {
        io::Mode::Simple(op) => {
            let file_name = args.get(1).unwrap_or_else(|| {
                println!("Please provide a filename!");
                process::exit(1);
            });
            let mut f = File::open(&file_name)?;
            let mut buffer = Vec::new();

            f.read_to_end(&mut buffer)?;
            let mut wav_op = op.clone();
            wav_op.set_wav(&buffer[..]);
            wav_op.perform();
            let mut new_file_name = adjust_file_path(&file_name[..]);

            new_file_name.push_str("_quikka.wav");
            wav_op.write_to_file(&new_file_name)?;

            Ok(())
        }
        _ => panic!("TODO! not yet implemented"),
    }
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
