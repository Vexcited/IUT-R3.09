use std::io::{self, Read, Write};
use clap::{Parser, ValueEnum};
use clio::*;

mod vigenere;
use vigenere::{vigenere_encrypt, vigenere_decrypt};

mod kasiski;
use kasiski::kasiski_examination;

/// CLI app that supports the kasiski and encrypt methods
#[derive(Parser)]
#[command(name = "TP_1and2")]
struct Cli {
  /// Method to use.
  #[arg(value_enum)]
  method: Method,

  /// Input file, use '-' for stdin.
  #[clap(value_parser, default_value="-")]
  input: Input,

  /// Output file '-' for stdout.
  #[clap(long, short, value_parser, default_value="-")]
  output: Output,

  /// The encryption key (used only if method is encrypt or decrypt)
  #[arg(long)]
  key: Option<String>,
}

/// Enum representing available methods
#[derive(Copy, Clone, ValueEnum)]
enum Method {
  Kasiski,
  Encrypt,
  Decrypt
}

fn main() -> io::Result<()> {
  let mut cli = Cli::parse();

  let mut input_data = String::new();
  cli.input.read_to_string(&mut input_data)?;

  let output_data = match cli.method {
    Method::Kasiski => {
      // let key_length = kasiski_examination(&input_data);
      // println!("The most probable key length: {}", key_length);
      std::process::exit(0);
    },
    Method::Encrypt => {
      if let Some(key) = cli.key {
        vigenere_encrypt(&input_data, &key)
      }
      else {
        eprintln!("Error: Key is required for the encrypt method.");
        std::process::exit(1);
      }
    }
    Method::Decrypt => {
      if let Some(key) = cli.key {
        vigenere_decrypt(&input_data, &key)
      }
      else {
        eprintln!("Error: Key is required for the decrypt method.");
        std::process::exit(1);
      }
    }
  };

  cli.output.write_all(output_data.as_bytes())?;

  Ok(())
}
