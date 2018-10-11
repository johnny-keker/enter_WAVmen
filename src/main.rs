#[macro_use] extern crate text_io;
extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
   // let a: (i32, i32) = (76, 42);
   // println!("{:?}", a);
   // let arr: Vec<u8> = vec![1, 2, 3, 4].into_iter().map(|x| x + 1).collect();
   // for c in arr {
   //     println!("{}", c);
   // }
   // let i: Result<u16, _> = try_read!();
   // match i {
   //     Ok(i_ok) => println!("{}", i_ok),
   //     _ => println!("u16, dummy!")
   // }
   // {
   //     let bytes: Vec<u8> = vec![0x36; 3];
   //     bytes.push(0x36);
   //     let mut file = File::create("example").unwrap();
   //     file.write(&bytes);
   // }
   // let mut file = File::open("example").unwrap();
   // let mut contents = String::new();
   // file.read_to_string(&mut contents)
   //     .expect("something went wrong reading the file");
   // println!("{}", contents);
   write_wav(44100, 10000);
}
fn write_wav(sample_rate: u32, num_samples: u32) {
    use byteorder::{WriteBytesExt, LittleEndian};
    let mut buf: Vec<u8> = b"RIFF".to_vec();
    buf.extend_from_slice(&[0; 4]); // there will be size of RIFF
    buf.extend_from_slice(b"WAVEfmt ");
    buf.extend_from_slice(&[16, 0, 0, 0]); // constant 16, little endian
    buf.extend_from_slice(&[1, 0]); // constant 1 for no compression
    buf.extend_from_slice(&[1, 0]); // constant 1 for mono
    buf.write_u32::<LittleEndian>(sample_rate).unwrap();    // sample_rate
    buf.write_u32::<LittleEndian>(sample_rate).unwrap(); // byterate = samplerate * numchannels * bitspersample / 8
    buf.write_u16::<LittleEndian>(1).unwrap(); // blockalign = numchannels * bitspersample / 8
    buf.write_u16::<LittleEndian>(8).unwrap(); // bitspersample
    buf.extend_from_slice(b"data"); // subcuhk2id
    buf.write_u32::<LittleEndian>(num_samples).unwrap(); // subchunk2size = num_samples * numchannels * bitspersample / 8
    // !data! 
    for i in 0..80000 {
    buf.write_u8((256.0 * (i as f32).sin()) as u8).unwrap();
    }
    let mut size_buf: Vec<u8> = Vec::new();
    size_buf.write_u32::<LittleEndian>(buf.len() as u32 - 8);
    buf.splice(4..8, size_buf);
    let mut file = File::create("music.wav").unwrap();
    file.write(&buf);
}
