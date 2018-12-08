extern crate byteorder;
extern crate rand;

pub fn get_wav_header(sample_rate: u32, num_samples: u32) -> Vec<u8> {
  use byteorder::{LittleEndian, WriteBytesExt};
  let mut buf: Vec<u8> = b"RIFF".to_vec();
  buf.extend_from_slice(&[0; 4]); // there will be size of RIFF
  buf.extend_from_slice(b"WAVEfmt ");
  buf.extend_from_slice(&[16, 0, 0, 0]); // constant 16, little endian
  buf.extend_from_slice(&[1, 0]); // constant 1 for no compression
  buf.extend_from_slice(&[1, 0]); // constant 1 for mono
  buf.write_u32::<LittleEndian>(sample_rate).unwrap(); // sample_rate
  buf.write_u32::<LittleEndian>(sample_rate).unwrap(); // byterate = samplerate * numchannels * bitspersample / 8
  buf.write_u16::<LittleEndian>(1).unwrap(); // blockalign = numchannels * bitspersample / 8
  buf.write_u16::<LittleEndian>(8).unwrap(); // bitspersample
  buf.extend_from_slice(b"data"); // subcuhk2id
  buf
    .write_u32::<LittleEndian>(num_samples * sample_rate)
    .unwrap(); // subchunk2size = num_samples * numchannels * bitspersample / 8
  return buf;
}
