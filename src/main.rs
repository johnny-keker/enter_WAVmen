extern crate byteorder;
extern crate rand;

#[allow(dead_code)]
mod notes_data;
mod notes_tools;
mod wav_tools;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
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

  let mut buf: Vec<u8> = wav_tools::get_wav_header(sample_rate, num_samples);

  /* data */

  let num_beats = (bpm * num_samples) / 4 / 60;
  let secs_per_beat = num_samples / num_beats;
  let notes: [f32; 4] = [0.5, 0.25, 0.125, 0.0625];
  let key: notes_tools::UserKey = notes_tools::init_notes();
  let mut rng = thread_rng();
  for step in 0..(num_beats) {
    let note_shift_range = rng.gen_range(2, 5);
    let mut l = 1.0;
    let cur_chord: notes_data::Chord = **rng.choose(&key.chords).unwrap();
    let mut curr_i = rng.gen_range(0, key.lead.len());
    while l != 0.0 {
      let avail_notes: Vec<f32> = notes.iter().filter(|&&n| n <= l).cloned().collect();
      let curr_note = rng.choose(&avail_notes).unwrap();
      let y = rng.gen::<f32>();
      curr_i = rng.gen_range(curr_i.checked_sub(note_shift_range).unwrap_or(0), std::cmp::min(curr_i + note_shift_range, key.lead.len()));
      let curr_n = key.lead[curr_i];
      for i in 0..(curr_note * (secs_per_beat * sample_rate) as f32) as u32 {
        let bass = 16.0 * (2.0 * PI * cur_chord[0] * (i as f32) / (sample_rate as f32) as f32).sin();
        let bass_third = 16.0 * (2.0 * PI * cur_chord[1] * (i as f32) / (sample_rate as f32) as f32).sin();
        let bass_quint = 16.0 * (2.0 * PI * cur_chord[2] * (i as f32) / (sample_rate as f32) as f32).sin();
        buf.write_u8(((((y * 16.0) + 16.0) * (2.0 * PI * curr_n * (i as f32) / (sample_rate as f32) as f32).sin()) - 128.0 + bass + bass_third + bass_quint) as u8)?;
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
