use diesel::{prelude::*};
use crate::schema::{pending_restaurants};

#[derive(Queryable, Selectable)]
pub struct PendingRestaurant {
    pub id: i32,
    pub name: String,
    pub information: String,
    pub isapproved: bool,
}