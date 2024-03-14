use diesel::prelude::*;
use api_onlyone_admin::{
    models::models::{PendingRestaurant},
    *,
};pub fn home() -> &'static str {
    use api_onlyone_admin::schema::pending_restaurants::dsl::*;

    let connection = &mut establish_connection();

    pending_restaurants.load::<PendingRestaurant>(connection).expect("Error loading posts");
    // let results: Vec<PendingRestaurant> = pending_restaurants.load::<PendingRestaurant>(connection).expect("Error loading posts");
    "Hello, world!"
}
