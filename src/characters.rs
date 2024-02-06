use std::collections::HashMap;

fn get_or_blank_str<'a>(character_code: &'a Vec<&'a str>, index: usize) -> &'a str {
    let mut result= "";
    if let Some(code) = character_code.get(index) {
        result = code;
    } else {
        result = "";
    }
    return result;
}

#[derive(Debug)]
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
    consequence_one: String,
    consequence_two: String,
    consequence_three: String,
    consequence_four: String,
    physical_stress_boxes: u8,
    mental_stress_boxes: u8,
}

impl CharacterSheet {
    pub fn from_code(code_string: String) -> Self {
        // Parses code from the google sheet at https://docs.google.com/spreadsheets/d/11hNuxQps6VYNa8Vo2YGe-jYQpKtRpyChx-i6LO9B0Kc/edit?usp=sharing
        let mut character_code: Vec<&str> = code_string.rsplitn(36, "§").collect();
        character_code.reverse();

        let character_name = get_or_blank_str(&character_code, 0);
        let character_description = get_or_blank_str(&character_code, 1);
        let character_refresh = get_or_blank_str(&character_code, 2);
        let character_high_concept = get_or_blank_str(&character_code, 3);
        let character_trouble = get_or_blank_str(&character_code, 4);
        let character_aspect_three = get_or_blank_str(&character_code, 5);
        let character_aspect_four = get_or_blank_str(&character_code, 6);
        let character_aspect_five = get_or_blank_str(&character_code, 7);
        let character_extras = get_or_blank_str(&character_code, 8);
        let character_stunts = get_or_blank_str(&character_code, 9);
        let character_consequence_one = get_or_blank_str(&character_code, 10);
        let character_consequence_two = get_or_blank_str(&character_code, 11);
        let character_consequence_three = get_or_blank_str(&character_code, 12);
        let character_consequence_four = get_or_blank_str(&character_code, 13);
        let character_physical_stress_capacity = get_or_blank_str(&character_code, 14);
        let character_mental_stress_capacity = get_or_blank_str(&character_code, 15);

        let converted_refresh: u8 = character_refresh.parse().unwrap_or(0);
        let converted_physical_stress: u8 = character_physical_stress_capacity.parse().unwrap_or(0);
        let converted_mental_stress_stress: u8 = character_mental_stress_capacity.parse().unwrap_or(0);

        let mut character_skills:HashMap<String, u8> = HashMap::new();

        let character_sheet = Self {
            name: character_name.to_string(),
            description: character_description.to_string(),
            refresh: converted_refresh,
            high_concept: character_high_concept.to_string(),
            trouble: character_trouble.to_string(),
            aspect_three: character_aspect_three.to_string(),
            aspect_four: character_aspect_four.to_string(),
            aspect_five: character_aspect_five.to_string(),
            skills: character_skills,
            extras: character_extras.to_string(),
            stunts: character_stunts.to_string(),
            consequence_one: character_consequence_one.to_string(),
            consequence_two: character_consequence_two.to_string(),
            consequence_three: character_consequence_three.to_string(),
            consequence_four: character_consequence_four.to_string(),
            physical_stress_boxes: converted_physical_stress,
            mental_stress_boxes: converted_mental_stress_stress,
        };

        return character_sheet;
    }
}

#[cfg(test)]
mod tests {
    use crate::characters::CharacterSheet;

    #[test]
    fn character_from_code() {
        let code = "Name§This is a description.⏎⏎Trust me.§0§Highest of Concepts§Afoot§Grand Ambitions§Friends in Low Places§Unceasing Bookworm§Lorem ipsum dolor sit amet, consectetur adipiscing elit.⏎⏎Morbi in neque tincidunt leo facilisis facilisis a id lorem.§• This is a list.⏎• Of stunts.§Stubbed Toe§Languishing Life§Decimating Ennui§Empty Bank Account§2§2§§§§§Investigate§§§§Contacts§Notice§§§§§§§Will§Stealth§Craft§Rapport";
        let character_sheet = CharacterSheet::from_code(String::from(code));
        println!("{:#?}", character_sheet);
    }

    #[test]
    fn character_from_blank_code() {
        let code = "";
        CharacterSheet::from_code(String::from(code));
    }
}

