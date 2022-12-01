use clap::{Arg, Command};
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Copy, Debug)]
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

        Ok(Config {
            action: matches.get_one::<Action>("Action").cloned().unwrap(),
            file: matches.get_one::<PathBuf>("File").cloned().unwrap(),
            chunk_type: matches.get_one::<Option<String>>("Type").cloned().unwrap(),
            chunk_data: matches.get_one::<Option<Vec<u8>>>("Data").cloned().unwrap(),
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
        // TODO: Read/Write input/output files

        eprintln!("{self:?}");
        unimplemented!();

        match self.action {
            Action::Encode => {
                let chunk_type =
                    crate::chunk_type::ChunkType::from_str(&self.chunk_type.unwrap()).unwrap();
                let chunk_data = self.chunk_data.unwrap();
                println!(
                    "{}",
                    crate::png::Png::from_chunks(vec![crate::chunk::Chunk::new(
                        chunk_type, chunk_data
                    )])
                );
                Ok(())
            }
            Action::Decode => todo!(),
            Action::Remove => todo!(),
            Action::Print => todo!(),
        }
    }
}
