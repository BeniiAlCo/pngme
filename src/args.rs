use clap::{Arg, Command};
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Action {
    Encode,
    Decode,
    Remove,
    Print,
}

#[derive(Debug)]
pub struct Config {
    action: Action,
    file: String,
    chunk_type: Option<String>,
    chunk_data: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let matches = Command::new("PNGme")
        .version("0.1")
        .author("BeniiAlCo")
        .about("An implementation of PNGme: a way of encoding and decoding messages hidden in PNG files.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .args_conflicts_with_subcommands(true)
        .subcommand(Command::new("encode").arg(Arg::new("encode").required(true).num_args(3).value_names(["FILE", "TYPE", "DATA"])))
        .subcommand(Command::new("decode").arg(Arg::new("decode").required(true).num_args(2).value_names(["FILE", "TYPE"])))
        .subcommand(Command::new("remove").arg(Arg::new("remove").required(true).num_args(2).value_names(["FILE", "TYPE"])))
        .subcommand(Command::new("print").arg(Arg::new("print").required(true).num_args(1).value_name("FILE")))
        
       
        .get_matches();

        if let Some((sub_command, sub_matches)) = matches.subcommand() {
            let action = match sub_command {
                "encode" => {Action::Encode}
                "decode" => {Action::Decode}
                "remove" => {Action::Remove}
                "print" => {Action::Print}
                _ => {unreachable!()}
            };

            let mut values: VecDeque<String> = sub_matches.get_many(sub_command).expect("t").cloned().collect();

            let file = values.pop_front().unwrap();
            let chunk_type = values.pop_front();
            let chunk_data = values.pop_front();

            Ok(Config {
                action,
                file,
                chunk_type,
                chunk_data,
            })
        } else {
            panic!()
            //Err("e");
        }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Parse all input -- valid path to file; valid type length(4 bytes); data as u8
        // TODO: Read/Write input/output files
        match self.action {
            Action::Encode => {
                let chunk_type = crate::chunk_type::ChunkType::from_str(&self.chunk_type.unwrap()).unwrap();
                let chunk_data = self.chunk_data.unwrap().into_bytes();
               println!("{}", crate::png::Png::from_chunks(vec![crate::chunk::Chunk::new(chunk_type, chunk_data)]));
               Ok(())
            }
            Action::Decode => todo!(),
            Action::Remove => todo!(),
            Action::Print => todo!(),
        }
    }
}
