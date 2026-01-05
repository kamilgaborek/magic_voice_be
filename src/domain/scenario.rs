use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    pub id: i64,
    pub name: String,
    pub template_text: String,
    pub active: bool,
}

pub fn example_scenario() -> Scenario {
    Scenario {
        id: 1,
        name: "Birthday Greeting".to_string(),
        template_text: "Happy Birthday, {name}! Wishing you a wonderful year ahead!".to_string(),
        active: true,
    }
}

