extern crate rand;

#[allow(dead_code)]
use notes_data;
use rand::prelude::*;

pub struct UserKey {
  pub lead: Vec<f32>,
  pub chords: notes_data::ChordPool,
}

pub fn get_vector_of_notes_from_lead(lead: notes_data::Lead) -> Vec<f32> {
  return lead
    .into_iter()
    .map(|n| n.into_iter().map(|e| *e))
    .flatten()
    .collect();
}

pub fn get_random_key() -> &'static notes_data::Key {
  let mut rng = thread_rng();
  return rng.choose(&notes_data::KEYS).unwrap();
}

pub fn init_notes() -> UserKey {
  let key: &notes_data::Key = get_random_key();
  let notes: Vec<f32> = get_vector_of_notes_from_lead(key.lead);
  return UserKey {
    lead: notes,
    chords: key.chords,
  };
}
