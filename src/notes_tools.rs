extern crate rand;

#[allow(dead_code)]
use notes_data;
use rand::prelude::*;

pub struct UserKey<'a> {
    pub lead: Vec<f32>,
    pub chords: notes_data::ChordPool<'a>
}

pub fn get_vector_of_notes_from_lead(lead: notes_data::Lead) -> Vec<f32> {
    return lead.into_iter().map(|n| n.into_iter().map(|e| *e)).flatten().collect();
}

pub fn get_random_key<'a>() -> &'a notes_data::Key<'a> {
    let mut rng = thread_rng();
    return rng.choose(&notes_data::KEYS).unwrap();
}

pub fn init_notes<'a>() -> UserKey<'a> {
    let key: &notes_data::Key = get_random_key();
    let notes: Vec<f32> = get_vector_of_notes_from_lead(key.lead);
    return UserKey { lead: notes, chords: key.chords };
}
