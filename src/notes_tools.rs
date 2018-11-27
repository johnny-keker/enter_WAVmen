#[allow(dead_code)]
use notes_data;

pub fn get_vector_of_notes_from_lead(lead: notes_data::lead_t) -> Vec<f32> {
    return lead.into_iter().map(|n| n.into_iter().map(|e| *e)).flatten().collect();
}
