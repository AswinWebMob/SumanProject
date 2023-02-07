// MAIL TASK
     
extern crate lettre;
extern crate lettre_email;
use lettre::smtp::authentication::IntoCredentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
fn main() {
    let smtp_address = "smtp.gmail.com";
    let username = "sumansaurabh1106@gmail.com"; 
    let password = "wfrv vypf kuaf hywm"; 
    let email = EmailBuilder::new()
        .to("dev.aswin.ai@gmail.com")  
        .from("sumansaurabh1106@gmail.com")    
        .subject("MAIL CHECKING USIING RUST")  
        .text("Mail Checking using rust by Suman Saurabh") 
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