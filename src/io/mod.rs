use crate::lib::Wav;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
pub enum Performance {
    DoubleSpeed,
}

#[derive(Clone)]
pub struct Operation<'a> {
    name: &'a str,
    description: &'a str,
    wav: Option<Wav<'a>>,
    performance: Option<Performance>,
}

impl<'a, 'b> Operation<'a> {
    pub fn new(name: &'a str, description: &'a str, performance: Performance) -> Operation<'a> {
        Operation {
            name,
            description,
            performance: Some(performance),
            wav: None,
        }
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn set_wav(&mut self, buf: &'a [u8]) {
        self.wav = Some(Wav::new(buf));
    }
    pub fn perform(&mut self) {
        // let mut wav = ;
        match self.performance {
            None => panic!("No performance loaded!"),
            Some(Performance::DoubleSpeed) => {
                let mut wav = self.wav.clone().unwrap();
                &wav.double_speed();
                self.wav = Some(wav.clone());
            }
        }
    }
    pub fn write_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let data = match &self.wav {
            None => panic!("No wav loaded to perform on!"),
            Some(wav) => wav.write_contents().unwrap(),
        };
        let mut file = File::create(filename)?;
        file.write_all(&data[..])?;
        Ok(())
    }
}

pub enum Mode<'a> {
    Simple(&'a Operation<'a>),
    Complex,
}

// struct App {
//     state: Box<dyn State>,
// }

// impl App {
//     pub fn new() -> App {
//         App {
//             state: Box::new(Welcome { message: "hello!" }),
//         }
//     }
// }

// trait State {
//     fn prompt(self: Self) -> String;
//     fn respond(self: Self) -> String;
// }

// struct Welcome<'a> {
//     message: &'a str,
// }

// impl<'a> State for Welcome<'a> {
//     fn prompt(self) -> String {
//         String::from(self.message)
//     }

//     fn respond(self: Self) -> String {
//         String::from("ok")
//     }
// }
