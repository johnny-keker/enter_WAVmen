extern crate byteorder;
extern crate rand;

#[allow(dead_code)]
mod notes_data;
mod notes_tools;
mod wav_tools;
mod wavman;

use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let wav: Vec<u8> = wavman::generate_ambient(44100, 40, 120).unwrap();
  let mut file = File::create(format!("out/{}.wav", filename)).unwrap();
  file.write(&wav).unwrap();
}
