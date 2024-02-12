use std::collections::HashMap;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteJournalMode};

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
        let superb_one = get_or_blank_str(&character_code, 16);
        if superb_one != "" { &character_skills.insert(superb_one.to_string(), 5); }
        let superb_two = get_or_blank_str(&character_code, 17);
        if superb_two != "" { &character_skills.insert(superb_two.to_string(), 5); }
        let superb_three = get_or_blank_str(&character_code, 18);
        if superb_three != "" { &character_skills.insert(superb_three.to_string(), 5); }
        let superb_four = get_or_blank_str(&character_code, 19);
        if superb_four != "" { &character_skills.insert(superb_four.to_string(), 5); }

        let great_one = get_or_blank_str(&character_code, 20);
        if great_one != "" { &character_skills.insert(great_one.to_string(), 4); }
        let great_two = get_or_blank_str(&character_code, 21);
        if great_two != "" { &character_skills.insert(great_two.to_string(), 4); }
        let great_three = get_or_blank_str(&character_code, 22);
        if great_three != "" { &character_skills.insert(great_three.to_string(), 4); }
        let great_four = get_or_blank_str(&character_code, 23);
        if great_four != "" { &character_skills.insert(great_four.to_string(), 4); }

        let good_one = get_or_blank_str(&character_code, 24);
        if good_one != "" { &character_skills.insert(good_one.to_string(), 3); }
        let good_two = get_or_blank_str(&character_code, 25);
        if good_two != "" { &character_skills.insert(good_two.to_string(), 3); }
        let good_three = get_or_blank_str(&character_code, 26);
        if good_three != "" { &character_skills.insert(good_three.to_string(), 3); }
        let good_four = get_or_blank_str(&character_code, 27);
        if good_four != "" { &character_skills.insert(good_four.to_string(), 3); }

        let fair_one = get_or_blank_str(&character_code, 28);
        if fair_one != "" { &character_skills.insert(fair_one.to_string(), 2); }
        let fair_two = get_or_blank_str(&character_code, 29);
        if fair_two != "" { &character_skills.insert(fair_two.to_string(), 2); }
        let fair_three = get_or_blank_str(&character_code, 30);
        if fair_three != "" { &character_skills.insert(fair_three.to_string(), 2); }
        let fair_four = get_or_blank_str(&character_code, 31);
        if fair_four != "" { &character_skills.insert(fair_four.to_string(), 2); }

        let average_one = get_or_blank_str(&character_code, 32);
        if average_one != "" { &character_skills.insert(average_one.to_string(), 1); }
        let average_two = get_or_blank_str(&character_code, 33);
        if average_two != "" { &character_skills.insert(average_two.to_string(), 1); }
        let average_three = get_or_blank_str(&character_code, 34);
        if average_three != "" { &character_skills.insert(average_three.to_string(), 1); }
        let average_four = get_or_blank_str(&character_code, 35);
        if average_four != "" { &character_skills.insert(average_four.to_string(), 1); }

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

    pub async fn save_to_db(&self, pool: &SqlitePool, guild_id: String, owner_id: String) -> i64 {
        let character_id = sqlx::query!(
            r#"
INSERT INTO characters (
    guild_id,
    owner_id,
    name,
    description,
    refresh,
    high_concept,
    trouble,
    aspect_three,
    aspect_four,
    aspect_five,
    extras,
    stunts,
    consequence_one,
    consequence_two,
    consequence_three,
    consequence_four,
    physical_capacity,
    mental_capacity
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18
)
            "#,
            guild_id,
            owner_id,
            self.name,
            self.description,
            self.refresh,
            self.high_concept,
            self.trouble,
            self.aspect_three,
            self.aspect_four,
            self.aspect_five,
            self.extras,
            self.stunts,
            self.consequence_one,
            self.consequence_two,
            self.consequence_three,
            self.consequence_four,
            self.physical_stress_boxes,
            self.mental_stress_boxes
        ).execute(pool)
            .await.expect("Failed to add character to database.")
            .last_insert_rowid();

        for (key, value) in &self.skills {
            println!("{:}, {:}", key, value);
            sqlx::query!(
                r#"
INSERT INTO character_skills (
    character_id,
    level,
    name
) VALUES (
    ?1, ?2, ?3
)
                "#,
                character_id,
                value,
                key
            ).execute(pool).await.expect("Failed to add skill to database.");
        }

        return character_id;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use sqlx;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqliteJournalMode};
    use std::str::FromStr;
    use crate::characters::CharacterSheet;

    #[test]
    fn character_from_code() {
        let code = "Name§This is a description.⏎⏎Trust me.§0§Highest of Concepts§Afoot§Grand Ambitions§Friends in Low Places§Unceasing Bookworm§Lorem ipsum dolor sit amet, consectetur adipiscing elit.⏎⏎Morbi in neque tincidunt leo facilisis facilisis a id lorem.§• This is a list.⏎• Of stunts.§Stubbed Toe§Languishing Life§Decimating Ennui§Empty Bank Account§2§2§§§§§Investigate§§§§Contacts§Notice§§§Lore§Resources§Empathy§§Will§Stealth§Craft§Rapport";
        let character_sheet = CharacterSheet::from_code(String::from(code));
        println!("{:#?}", character_sheet);
    }

    #[test]
    fn character_from_blank_code() {
        let code = "";
        CharacterSheet::from_code(String::from(code));
    }

    #[tokio::test]
    async fn character_and_db() {
        let _ = fs::remove_file("faebot-test.db");
        let _ = fs::remove_file("faebot-test.db-shm");
        let _ = fs::remove_file("faebot-test.db-wall");

        let db_url = "sqlite://faebot-test.db";
        let options = SqliteConnectOptions::from_str(db_url).expect("Unable to read database.")
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePool::connect_with(options).await.expect("Database connection failed!");
        sqlx::migrate!().run(&pool).await.expect("Unable to run migrations.");

        let code = "Name§This is a description.⏎⏎Trust me.§0§Highest of Concepts§Afoot§Grand Ambitions§Friends in Low Places§Unceasing Bookworm§Lorem ipsum dolor sit amet, consectetur adipiscing elit.⏎⏎Morbi in neque tincidunt leo facilisis facilisis a id lorem.§• This is a list.⏎• Of stunts.§Stubbed Toe§Languishing Life§Decimating Ennui§Empty Bank Account§2§2§§§§§Investigate§§§§Contacts§Notice§§§Lore§Resources§Empathy§§Will§Stealth§Craft§Rapport";
        let character_sheet = CharacterSheet::from_code(String::from(code));

        let character_id = character_sheet.save_to_db(&pool, "test".to_string(), "test".to_string()).await;
        println!("{:#?}", character_id);
    }
}