extern crate byteorder;
extern crate rand;

#[allow(dead_code)]
use notes_data;
use notes_tools;
use wav_tools;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn generate_ambient(sample_rate: u32, num_samples: u32, bpm: u32) -> std::io::Result<Vec<u8>> {

  use byteorder::{LittleEndian, WriteBytesExt};

  /* header */

  let mut buf: Vec<u8> = wav_tools::get_wav_header(sample_rate, num_samples);

  /* data */
  let num_beats = (bpm * num_samples) / 4 / 60;             // number of beats we need to generate
  let seconds_per_beat = num_samples / num_beats;           // number of seconds per beat
  let durations: [f32; 4] = [0.5, 0.25, 0.125, 0.0625];     // notes durations TODO: transfer it to notes_data
  let mut key: notes_tools::UserKey = notes_tools::init_notes();// our random key for the song
  let part = (0.0625 * (seconds_per_beat * sample_rate) as f32) as u32;
  for _beat in 0..(num_beats) {
      generate_beat(&mut key, durations, part, sample_rate, seconds_per_beat, &mut buf);
  }

  /* data size */

  let mut size_buf: Vec<u8> = Vec::new();
  size_buf.write_u32::<LittleEndian>(buf.len() as u32 - 8)?;
  buf.splice(4..8, size_buf);
  Ok(buf)
}

fn write_part(bufer: &mut Vec<u8>, part: u32, note: f32, chord: notes_data::Chord, y: f32, sample_rate: u32) {
  use std::f32::consts::PI;
  use byteorder::{LittleEndian, WriteBytesExt};
  for i in 0..part {
    let bass = 16.0 * (2.0 * PI * chord[0] * (i as f32) / (sample_rate as f32) as f32).sin();
    let bass_third = 16.0 * (2.0 * PI * chord[1] * (i as f32) / (sample_rate as f32) as f32).sin();
    let bass_quint = 16.0 * (2.0 * PI * chord[2] * (i as f32) / (sample_rate as f32) as f32).sin();
    let lead = ((y * 16.0) + 16.0) * (2.0 * PI * note * (i as f32) / (sample_rate as f32) as f32).sin() - 128.0;
    bufer.write_u8((lead + bass + bass_third + bass_quint) as u8);
  }
}

fn generate_beat(key: &mut notes_tools::UserKey, durations: [f32; 4], part: u32, sample_rate: u32, seconds_per_beat: u32, buf: &mut Vec<u8>) {
      let mut rng = thread_rng();                           // rand init
      let note_shift_range = rng.gen_range(2, 5);           // so we wouldnt have huge jumps
      let mut beat_remain = 1.0;                            // so we`ll know when beat is done
      let curr_chord: notes_data::Chord = **rng.choose(&key.chords).unwrap(); // current chord, one for beat
      let mut curr_index = rng.gen_range(0, key.lead.len()) as u32;// so we wouldnt have huge jumps
      while beat_remain != 0.0 {
          let avail_durations: Vec<f32> = durations.iter().filter(|&&n| n <= beat_remain).cloned().collect(); // calculate available durations
          let curr_duration = rng.choose(&avail_durations).unwrap(); // choose random duration
          curr_index = rng.gen_range(curr_index.checked_sub(note_shift_range).unwrap_or(0), std::cmp::min(curr_index + note_shift_range, key.lead.len() as u32)) as u32;  // so we woudnt have huge jumps
          let curr_note = key.lead[curr_index as usize]; // calculate current frequency
          let y = rng.gen::<f32>();
          let duration_in_parts = (curr_duration * (seconds_per_beat * sample_rate) as f32) as u32;
          for _step in 0..(duration_in_parts / part) as u32 {
              write_part(buf, part, curr_note, curr_chord, y, sample_rate);
          }
          beat_remain -= curr_duration;
      }
}
