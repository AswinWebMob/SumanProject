use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)] 
pub struct Student { 
    pub id: Option<ObjectId>,
    pub physics: i32,
    pub chemistry: i32,
    pub username: String,  
}


#[derive(Debug, Serialize, Deserialize)] 
pub struct StudenMock {
    pub physics: i32,
    pub chemistry: i32,
    pub username: String,  
}