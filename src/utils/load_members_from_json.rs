use std::fs;
use serde_json::Result as JsonResult;
use crate::event::Member;

pub fn load_members_from_json() -> JsonResult<Vec<Member>> {
    let file_content = fs::read_to_string("src/db_seed.json").expect("Failed to read db_seed.json");
    
    let members: Vec<Member> = serde_json::from_str(&file_content).expect("Failed to deserialize JSON");

    Ok(members)    
}
