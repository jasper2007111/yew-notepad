use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: u32,
    pub content: String, 
    pub create_time: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WriteNote {
    pub content: String, 
    pub create_time: String
}