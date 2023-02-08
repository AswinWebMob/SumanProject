/* 
Actix Web    - API             
*/ 
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};   
// #[get("/")]  
// async fn hello() -> impl Responder { 
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body) 
// }  

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// } 

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }




// #[warn(unused_variables)]

// extern crate bcrypt;
// use bcrypt::{hash, DEFAULT_COST};
// extern crate argon2rs;  
// use argon2rs::argon2i_simple;
// MongoDB Connection   
use mongodb::{ 
    bson::doc,
    sync::Client, 
};

use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]  
struct User{  
    name: String, 
    password: String,
    email: String, 
}

fn add_user(client: mongodb::sync::Client){   
    let db = client.database("test"); 
    let coll = db.collection("users");   

    // let password = "crypto".to_string();   
    // let hashed_password = hash(password, DEFAULT_COST).unwrap(); 

    // let password = "password123".as_bytes();
    // let salt = [0u8; 8];
    // let hashed_password = argon2i_simple(password, &salt).unwrap(); 
    //  let hashed_password =argon2i_simple(password, &salt.unwrap()); 

    coll.insert_one(doc!("UserName" : "suman", "email" : "imsumansaurabh@gmail.com", "password": "password" ), None);     
}

fn find_user(coll: mongodb::sync::Collection<User>){  
    let cursor = coll.find(doc!{"name" : "suman"},None).unwrap();    
    for result in cursor {
        println!("{:?}", result)  
    }
} 

fn remove_user(coll: mongodb::sync::Collection<User>){
    let result = coll.delete_one(doc!{"name" : "suman"},None).unwrap();
    println!("{:?}", result);  
} 

fn main(){
    println!("Hello DB User");  
    let client = Client::with_uri_str("mongodb+srv://suman:12345@cluster0.hrqi2li.mongodb.net/test").unwrap();  
    add_user(client);   
    // let db = client.database("test");
    // let coll = db.collection::<User>("users");  
    // find_user(coll);
    // remove_user(coll);  
}    