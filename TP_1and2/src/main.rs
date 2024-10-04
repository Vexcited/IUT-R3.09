use clap::{Parser, ValueEnum};
use clio::*;
use std::{
    io::{Read, Result, Write},
    process::exit,
};

mod vigenere;
use vigenere::{vigenere_decrypt, vigenere_encrypt};

mod kasiski;
use kasiski::kasiski_analysis;

mod input;

/// CLI app that supports the kasiski and encrypt methods
#[derive(Parser)]
#[command(name = "TP_1and2")]
struct Cli {
    /// Method to use.
    #[arg(value_enum)]
    method: Method,

    /// Input file, use '-' for stdin.
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// Output file '-' for stdout.
    #[clap(long, short, value_parser, default_value = "-")]
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
    Decrypt,
}

fn main() -> Result<()> {
    let mut cli = Cli::parse();

    let mut input_data = String::new();
    cli.input.read_to_string(&mut input_data)?;

    let output_data = match cli.method {
        Method::Kasiski => {
            let key_length = kasiski_analysis(&input_data);

            match key_length {
                Ok(key_length) => {
                    println!("Key length is {}", key_length);
                    exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            };
        }

        Method::Encrypt => {
            if let Some(key) = cli.key {
                vigenere_encrypt(&input_data, &key)
            } else {
                eprintln!("Error: Key is required for the encrypt method.");
                exit(1);
            }
        }

        Method::Decrypt => {
            if let Some(key) = cli.key {
                vigenere_decrypt(&input_data, &key)
            } else {
                eprintln!("Error: Key is required for the decrypt method.");
                exit(1);
            }
        }
    };

    cli.output.write_all(output_data.as_bytes())?;

    Ok(())
}
