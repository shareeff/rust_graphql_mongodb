use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize,)]
#[derive(juniper::GraphQLInputObject)]
pub struct NewMember {
  pub id: i32,
  pub name: String,
}

#[juniper::object(description = "A member of a team")]
impl Member {
  pub fn id(&self) -> i32 {
    self.id  
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }
}

