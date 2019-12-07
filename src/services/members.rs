use mongodb::{Bson, bson, doc};

use crate::models::members::{Member, NewMember};
use crate::collection;
use juniper::FieldError;
use std::error::Error;


pub fn get_members() ->  Result<Vec<Member>,FieldError>{
    let coll = collection("member");
    let cursor = coll.find(None, None).unwrap();
    let mut resutlts: Vec<Member> = vec![];
    for result in cursor {
        match result {
            Ok(doc) => {
               let member = bson::from_bson(bson::Bson::Document(doc))?;
               resutlts.push(member);
            },
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
            

        }
       
    }
    Ok(resutlts)
}

pub fn create_member(new_member: NewMember) -> Result<Member,FieldError>{
    let coll = collection("member");
    let serialized_member = bson::to_bson(&new_member)?; 

    if let bson::Bson::Document(document) = serialized_member {
        coll.insert_one(document, None)?;  // Insert into a MongoDB collection
        let member_document = coll.find_one(Some(doc! { "id" => new_member.id }), None)?
                .expect("Document not found");

        let member = bson::from_bson(bson::Bson::Document(member_document))?;
        Ok(member)
        
        
    } else {
        println!("Error converting the BSON object into a MongoDB document");
        Err("Error converting the BSON object into a MongoDB document")?
    }



}