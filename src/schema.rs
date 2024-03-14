diesel::table! {
  pending_restaurants (id) {
      id -> Int4,
      name -> Varchar,
      information -> Varchar,
      isapproved -> Bool,
  }
}