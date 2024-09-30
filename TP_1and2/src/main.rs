use std::{io::{Result, Read, Write}, process::exit};
use clap::{Parser, ValueEnum};
use clio::*;

mod vigenere;
use vigenere::{vigenere_encrypt, vigenere_decrypt};

mod kasiski;
use kasiski::kasiski_analysis;

mod input;
mod maths;

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
  key: Option<String>
}

/// Enum representing available methods
#[derive(Copy, Clone, ValueEnum)]
enum Method {
  Kasiski,
  Encrypt,
  Decrypt
}

fn main() -> Result<()> {
  let mut cli = Cli::parse();

  let mut input_data = String::new();
  cli.input.read_to_string(&mut input_data)?;

  let output_data = match cli.method {
    Method::Kasiski => {
      let result = kasiski_analysis(&input_data);
      
      if result.is_empty(){
        println!("Aucune hypothèse possible : ?");
      }
      else {
        println!("Taille(s) de clé potentielle(s) :");
        for key_length in result {
          println!("=> {}", key_length);
        }
      }

      exit(0);
    }

    Method::Encrypt => {
      if let Some(key) = cli.key {
        vigenere_encrypt(&input_data, &key)
      }
      else {
        eprintln!("Error: Key is required for the encrypt method.");
        exit(1);
      }
    }
    
    Method::Decrypt => {
      if let Some(key) = cli.key {
        vigenere_decrypt(&input_data, &key)
      }
      else {
        eprintln!("Error: Key is required for the decrypt method.");
        exit(1);
      }
    }
  };

  cli.output.write_all(output_data.as_bytes())?;

  Ok(())
}
