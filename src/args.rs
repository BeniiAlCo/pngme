use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Encode,
    Decode,
    Remove,
    Print,
}

#[derive(Debug)]
pub struct Config {
    action: Action,
    file: PathBuf,
    chunk_type: Option<String>,
    chunk_data: Option<Vec<u8>>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let matches = Command::new("PNGme")
        .version("0.1")
        .author("BeniiAlCo")
        .about("An implementation of PNGme: a way of encoding and decoding messages hidden in PNG files.")
        .arg_required_else_help(true)
        .arg(Arg::new("Action")
            .required(true)
            .value_parser(["encode", "decode", "remove", "print"])
            .value_parser(Self::action_to_enum)
            .value_name("ACTION")
            .requires_ifs([("encode", "Type"), ("encode", "Data"), ("decode", "Type"), ("remove", "Type")]))
        .arg(Arg::new("File")
            .required(true)
            .value_parser(clap::value_parser!(PathBuf))
            .value_name("FILE"))
        .arg(Arg::new("Type")
            .value_parser(Self::type_is_4_bytes)
            .value_name("TYPE"))
        .arg(Arg::new("Data")
            .value_parser(Self::data_to_u8)
            .value_name("DATA")) 
        .get_matches();

        let action = matches.get_one::<Action>("Action").cloned().unwrap();
        let file = matches.get_one::<PathBuf>("File").cloned().unwrap();

        let (chunk_type, chunk_data) = match action {
            Action::Encode => (
                (matches.get_one::<Option<String>>("Type").cloned().unwrap()),
                matches.get_one::<Option<Vec<u8>>>("Data").cloned().unwrap(),
            ),
            Action::Decode | Action::Remove => (
                matches.get_one::<Option<String>>("Type").cloned().unwrap(),
                None,
            ),
            Action::Print => (None, None),
        };

        Ok(Config {
            action,
            file,
            chunk_type,
            chunk_data,
        })
    }

    fn type_is_4_bytes(s: &str) -> Result<Option<String>, String> {
        if s.len() == 4 {
            Ok(Some(s.to_string()))
        } else {
            Err("The type provided was not 4 bytes long".to_string())
        }
    }

    fn data_to_u8(s: &str) -> Result<Option<Vec<u8>>, String> {
        if !s.is_empty() {
            Ok(Some(String::into_bytes(s.to_string())))
        } else {
            Ok(None)
        }
    }

    fn action_to_enum(s: &str) -> Result<Action, String> {
        match s {
            "encode" => Ok(Action::Encode),
            "decode" => Ok(Action::Decode),
            "remove" => Ok(Action::Remove),
            "print" => Ok(Action::Print),
            _ => Err("Something went wrong!".to_string()),
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::chunk::Chunk;
        use crate::chunk_type::ChunkType;
        use crate::png::Png;

        let mut buffer = Vec::new();
        if self.action != Action::Encode {
            match Self::open(&self.file) {
                Err(err) => {
                    eprintln!("Failed to open {}: {err}", self.file.display());
                    panic!("file was missing")
                }
                Ok(mut file) => {
                    file.read_to_end(&mut buffer)?;
                }
            }
        }

        match self.action {
            Action::Encode => {
                let mut buf = File::create(&self.file).unwrap();
                let chunk_type = ChunkType::from_str(&self.chunk_type.unwrap()).unwrap();
                let chunk_data = self.chunk_data.unwrap();
                buf.write_all(
                    &Png::from_chunks(vec![Chunk::new(chunk_type, chunk_data)]).as_bytes(),
                )?;
                Ok(())
            }
            Action::Decode => {
                let chunk_type = &self.chunk_type.unwrap();
                let png: Png = TryFrom::try_from(buffer.as_ref()).unwrap();
                if let Some(chunk_data) = png.chunk_by_type(chunk_type) {
                    println!("{}", Chunk::data_as_string(chunk_data).unwrap());
                } else {
                    eprintln!("Something went wrong! The chunk type provided does not exist in the PNG file provided!")
                }
                Ok(())
            }
            Action::Remove => {
                let mut buf = File::create(&self.file).unwrap();
                let chunk_type = &self.chunk_type.unwrap();
                let mut png: Png = TryFrom::try_from(buffer.as_ref()).unwrap();
                png.remove_chunk(chunk_type).ok();
                buf.write_all(&png.as_bytes())?;
                Ok(())
            }
            Action::Print => {
                let png: Png = TryFrom::try_from(buffer.as_ref()).unwrap();
                println!("{}", png);
                Ok(())
            }
        }
    }

    fn open(file: &PathBuf) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
        Ok(Box::new(BufReader::new(File::open(file)?)))
    }
}
