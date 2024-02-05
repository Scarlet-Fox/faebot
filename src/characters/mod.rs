use std::collections::HashMap;

pub struct CharacterSheet {
    name: String,
    description: String,
    refresh: u8,
    high_concept: String,
    trouble: String,
    aspect_three: String,
    aspect_four: String,
    aspect_five: String,
    skills: HashMap<String, u8>,
    extras: String,
    stunts: String,
    physical_stress_boxes: u8,
    mental_stress_boxes: u8,
}

impl CharacterSheet {

}