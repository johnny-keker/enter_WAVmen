extern crate byteorder;
extern crate rand;

#[allow(dead_code)]
mod notes;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
  print!("{:?}", notes::A0);
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];
  let wav: Vec<u8> = generate_wav(44100, 40, 120).unwrap();
  let mut file = File::create(format!("out/{}.wav", filename)).unwrap();
  file.write(&wav).unwrap();
}

fn generate_wav(sample_rate: u32, num_samples: u32, bpm: u32) -> std::io::Result<Vec<u8>> {
  use byteorder::{LittleEndian, WriteBytesExt};
  use std::f32::consts::PI;

  /* header */

  let mut buf: Vec<u8> = b"RIFF".to_vec();
  buf.extend_from_slice(&[0; 4]); // there will be size of RIFF
  buf.extend_from_slice(b"WAVEfmt ");
  buf.extend_from_slice(&[16, 0, 0, 0]); // constant 16, little endian
  buf.extend_from_slice(&[1, 0]); // constant 1 for no compression
  buf.extend_from_slice(&[1, 0]); // constant 1 for mono
  buf.write_u32::<LittleEndian>(sample_rate)?; // sample_rate
  buf.write_u32::<LittleEndian>(sample_rate)?; // byterate = samplerate * numchannels * bitspersample / 8
  buf.write_u16::<LittleEndian>(1)?; // blockalign = numchannels * bitspersample / 8
  buf.write_u16::<LittleEndian>(8)?; // bitspersample
  buf.extend_from_slice(b"data"); // subcuhk2id
  buf.write_u32::<LittleEndian>(num_samples * sample_rate)?; // subchunk2size = num_samples * numchannels * bitspersample / 8

  /* data */

  let num_beats = (bpm * num_samples) / 4 / 60;
  let secs_per_beat = num_samples / num_beats;
  let notes: [f32; 4] = [0.5, 0.25, 0.125, 0.0625];
  let am: [f32; 7] = [440.0, 493.88, 523.25, 587.33, 659.25, 698.46, 783.99];
  let am_bass: [f32; 7] = [220.0, 246.94, 261.63, 146.83, 164.81, 174.61, 196.00];
  let am_bass_third: [f32; 7] = [261.63, 146.83, 164.81, 174.61, 196.00, 220.00, 246.94];
  let am_bass_quint: [f32; 7] = [146.83, 174.61, 196.00, 220.00, 246.94, 261.63, 293.66];
  let mut rng = thread_rng();
  for step in 0..(num_beats) {
    let mut l = 1.0;
    let curr_bass = if step == 0 || step == num_beats { 220.0 } else { *rng.choose(&am_bass).unwrap() };
    let index = am_bass.iter().position(|&r| r == curr_bass).unwrap();
    let curr_bass_third = am_bass_third[index];
    let curr_bass_quint = am_bass_quint[index];
    while l != 0.0 {
      let avail_notes: Vec<f32> = notes.iter().filter(|&&n| n <= l).cloned().collect();
      let curr_note = rng.choose(&avail_notes).unwrap();
      let y = rng.gen::<f32>();
      let curr_am = rng.choose(&am).unwrap();
      for i in 0..(curr_note * (secs_per_beat * sample_rate) as f32) as u32 {
        let bass = 16.0 * (2.0 * PI * curr_bass * (i as f32) / (sample_rate as f32) as f32).sin();
        let bass_third = 16.0 * (2.0 * PI * curr_bass_third * (i as f32) / (sample_rate as f32) as f32).sin();
        let bass_quint = 16.0 * (2.0 * PI * curr_bass_quint * (i as f32) / (sample_rate as f32) as f32).sin();
        buf.write_u8(((((y * 16.0) + 16.0) * (2.0 * PI * curr_am * (i as f32) / (sample_rate as f32) as f32).sin()) - 128.0 + bass + bass_third + bass_quint) as u8)?;
      }
      l -= curr_note;
    }
  }

  /* data size */

  let mut size_buf: Vec<u8> = Vec::new();
  size_buf.write_u32::<LittleEndian>(buf.len() as u32 - 8)?;
  buf.splice(4..8, size_buf);
  Ok(buf)
}
