use juniper::{EmptyMutation, FieldError, FieldResult, RootNode};

use crate::models::members::{Member, NewMember};
use crate::services;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn members() -> FieldResult<Vec<Member>> {
        let all_members: Vec<Member> = services::members::get_members()?;
        Ok(all_members)
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_member(new_member: NewMember) -> FieldResult<Member> {
        let member: Member = services::members::create_member(new_member)?;
        Ok(member)
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
