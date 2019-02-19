

#![feature(proc_macro_hygiene, decl_macro,plugin)]

#[macro_use] 
extern crate serde_derive;

#[macro_use] 
extern crate rocket;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate reqwest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name:String,
    pub age:i32
}


use rocket_contrib::json::Json;
#[get("/person" )]
fn person() -> Json<Person> {
    let Bob:Person = Person{name:"Bob".to_string(),age:32};
    
    Json(Bob)
        
}

#[get("/" )]
fn index() ->  &'static str {
   "Use localhost:8000/person 
to retrieve a json person "
}

fn main() {
    rocket::ignite().mount("/", routes![index,person]).launch();
    
}
