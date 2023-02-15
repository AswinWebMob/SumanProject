use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, 
};

use crate::models::user_model::User;
use crate::models::students_model::Student; 

pub struct MongoRepo {
    col : Collection<User>, 
    coll: Collection<Student> 
}
impl MongoRepo {

     pub async fn init() -> Self {
        dotenv().ok();
        let uri = "mongodb+srv://webmob:webmob@cluster0.gdvte43.mongodb.net";   
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("webmob");   
        let col: Collection<User> = db.collection("User");
        let coll: Collection<Student> = db.collection("Student"); 
        MongoRepo { col , coll} 
    }
    //
    pub async fn add_marks(&self, new_student: Student) -> Result<InsertOneResult, Error> {
        let new_students = Student {
            id: None,
            physics: new_student.physics, 
            chemistry: new_student.chemistry,
            username: new_student.username,  

        };
        let student = self
            .coll 
            .insert_one(new_students, None) 
            .await
            .ok()
            .expect("Error creating user");
        Ok(student) 
    }
    //

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            user_name: new_user.user_name, 
            password: new_user.password,
            email: new_user.email,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }
// 
pub async fn get_result(&self, id: &String) -> Result<Student, Error> { 
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .coll 
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting student's detail"); 

        Ok(student_detail.unwrap()) 
    } 
//
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

//
pub async fn update_marks(&self, id: &String, new_student: Student) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap(); 
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    // "id": new_student.id, 
                    "physics": new_student.physics, 
                    "chemistry": new_student.chemistry, 
                    "username": new_student.username
                },   
        };
        let updated_doc = self  
            .coll 
            .update_one(filter, new_doc, None)
            .await
            .ok()   
            .expect("Error updating Marks"); 
        Ok(updated_doc)
    }
//
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "user_name": new_user.user_name,
                    "password": new_user.password, 
                    "email": new_user.email
                }, 
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok() 
            .expect("Error updating user");
        Ok(updated_doc)
    }
    
//
pub async fn delete_marks(&self, id: &String) -> Result<DeleteResult, Error> { 
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let student_detail = self
            .coll 
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(student_detail) 
    }
//
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    } 
// Teacher All Student
pub async fn get_all_students(&self) -> Result<Vec<Student>, Error> {   
        let mut point = self
            .coll 
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of Students"); 
        let mut students: Vec<Student> = Vec::new(); 
        while let Some(student) = point 
            .try_next()
            .await
            .ok()
            .expect("Error mapping through pointer") 
        {
            students.push(student) 
        }
        Ok(students) 
    }
//
    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}
