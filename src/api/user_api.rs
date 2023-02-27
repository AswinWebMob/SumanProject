extern crate lettre;  
extern crate lettre_email;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use actix_web::{web, App, HttpServer}; 
use crate::{jwt_sign, TempClientConn};   
use jwt_sign::{create_jwt};  
use jwt_simple::prelude::*;    
 
use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo, models::students_model::Student};   
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId; 

async fn mail() {  
    let smtp_address = "smtp.gmail.com";
    let username = "sumansaurabh1106@gmail.com"; 
    let password = "wfrv vypf kuaf hywm"; 
    let email = EmailBuilder::new()
        .to(username.to_owned())                // see email 
        .from("sumansaurabh1106@gmail.com")     
        .subject("MARKS ADDED!")  
        .text("Your teacher has added your marks")  
        .build()
        .unwrap()
        .into();
    let credentials = (username, password).into_credentials();
    let mut client = SmtpClient::new_simple(smtp_address)
        .unwrap()
        .credentials(credentials)
        .transport();
    let _result = client.send(email);

}

// #[post("/teacher/addmarks")]   
pub async fn add_marks(db: web::Data<crate::AppState>, new_student: Json<Student>) -> HttpResponse { 

    let db = db.client_conn.database("webmob") ;
    let collection = db.collection("Student");

    let service_conainer = TempClientConn::new(MongoRepo::new(collection.clone()).await);
    
    let data = Student { 
        id: None,
        physics: new_student.physics, 
        chemistry: new_student.chemistry,
        username: new_student.username.to_string(), 
    };

    let subject_marks = 
        service_conainer
            .client_conn
            .add_marks(data).await;

    // let subject_marks= db.add_marks(data).await; 
    mail();  
    
    match subject_marks { 
        Ok(subject_marks) => HttpResponse::Ok().body("Running".to_string()),  
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

// #[post("/register")]   
// pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse { 
//     let data = User {
//         id: None,
//         user_name: new_user.user_name.to_owned(),
//         password: new_user.password.to_owned(),
//         email: new_user.email.to_owned(), 
//     };

//     let user_detail = db.create_user(data).await;
//     mail();    
//     print!("Token Generated : {}",create_jwt("sumansaurabh1106@gmail.com".to_string())); // Tokn 



//     match user_detail {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }

// }

// #[get("/user/{id}")] 
// pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }
//     let user_detail = db.get_user(&id).await;

//     match user_detail {
//         Ok(user) => HttpResponse::Ok().json(user),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// } 

// // #[get("/student/{id}")] 
// pub async fn get_result(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }
//     let student_detail = db.get_result(&id).await; 

//     match student_detail {
//         Ok(student) => HttpResponse::Ok().json(student),                                      
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[put("/user/{id}")]
// pub async fn update_user(
//     db: Data<MongoRepo>,
//     path: Path<String>,
//     new_user: Json<User>,
// ) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     };
//     let data = User {
//         id: Some(ObjectId::parse_str(&id).unwrap()),  
//         user_name: new_user.user_name.to_owned(),
//         password: new_user.password.to_owned(),
//         email: new_user.email.to_owned(),
//     };

//     let update_result = db.update_user(&id, data).await;

//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let updated_user_info = db.get_user(&id).await;

//                 return match updated_user_info {
//                     Ok(user) => HttpResponse::Ok().json(user),
//                     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//                 };
//             } else {
//                 return HttpResponse::NotFound().body("No user found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// // #[put("/teacher/editmarks/{id}")]   
// pub async fn update_marks( 
//     db: Data<MongoRepo>,
//     path: Path<String>,
//     new_student: Json<Student>, 
// ) -> HttpResponse {
//     let id = path.into_inner(); 
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     }; 
//     let data = Student { 
//         id: Some(ObjectId::parse_str(&id).unwrap()), 
//         // id: None,    
//         physics: new_student.physics,
//         chemistry: new_student.chemistry,
//         username: new_student.username.to_owned() 
//     };

//     let update_marks = db.update_marks(&id, data).await; 
//     mail(); 
//     match update_marks { 
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let updated_student_marks = db.get_result(&id).await;   

//                 return match updated_student_marks {  
//                     Ok(student) => HttpResponse::Ok().json(student),   
//                     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//                 };
//             } else {
//                 return HttpResponse::NotFound().body("No user found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[delete("/user/{id}")]
// pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     };
//     let result = db.delete_user(&id).await;

//     match result {
//         Ok(res) => {
//             if res.deleted_count == 1 {
//                 return HttpResponse::Ok().json("User successfully deleted!");
//             } else {
//                 return HttpResponse::NotFound().json("User with specified ID not found!");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// // #[delete("/teacher/deletemarks/{id}")] 
// pub async fn delete_marks(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     };
//     let res = db.delete_marks(&id).await; 

//     match res {  
//         Ok(ans) => {
//             if ans.deleted_count == 1 {
//                 return HttpResponse::Ok().json("User successfully deleted!");
//             } else {
//                 return HttpResponse::NotFound().json("User with specified ID not found!");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// // #[get("/teacher/getallstudents")]   
// pub async fn get_all_students(db: Data<MongoRepo>) -> HttpResponse { 
//     let students = db.get_all_students().await; 

//     match students {
//         Ok(students) => HttpResponse::Ok().json(students),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }

// #[get("/users")]
// // #[only JWT token is authenticated .... ]
// pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
//     let users = db.get_all_users().await;

//     match users {
//         Ok(users) => HttpResponse::Ok().json(users),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
