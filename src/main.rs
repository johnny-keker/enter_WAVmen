#[macro_use] extern crate text_io;
extern crate byteorder;
extern crate rand;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;

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
   write_wav(44100, 4, 120);
}
fn write_wav(sample_rate: u32, num_samples: u32, bpm: u32) {
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
    buf.write_u32::<LittleEndian>(num_samples * sample_rate).unwrap(); // subchunk2size = num_samples * numchannels * bitspersample / 8
    // !data!
    let num_beats = (bpm * num_samples) / 4 / 60;
    println!("num_beats = {}", num_beats);
    let secs_per_beat = num_samples / num_beats;
    println!("secs_per_beat = {}", secs_per_beat);
    let notes: [f32; 5] = [0.5, 0.25, 0.125, 0.0625, 0.03125];
    let mut rng = thread_rng();

    for _ in 0..(num_beats) {
      let mut l = 1.0;
      while l != 0.0 {
        let avail_notes: Vec<f32> = notes.iter().filter(|&&n| n <= l).cloned().collect();
        let curr_note = rng.choose(&avail_notes).unwrap();

        let x = rand::random::<f32>();
        for i in 0..(curr_note * (secs_per_beat * sample_rate) as f32) as u32 {
          buf.write_u8(((128.0 * (x * i as f32).sin()) + 64.0) as u8).unwrap();
        }
        l -= curr_note;
      }
    }
    let mut size_buf: Vec<u8> = Vec::new();
    size_buf.write_u32::<LittleEndian>(buf.len() as u32 - 8);
    buf.splice(4..8, size_buf);
    let mut file = File::create("music.wav").unwrap();
    file.write(&buf);
}
