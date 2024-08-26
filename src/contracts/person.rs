
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person{
    pub name: String,
    pub age: i32,
}