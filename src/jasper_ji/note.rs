use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub content: String, 
    pub create_time: String
}