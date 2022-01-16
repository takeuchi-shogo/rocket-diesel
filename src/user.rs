use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::schema::users;
use crate::schema::users as users_schema;

use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug, Queryable, Serialize, Deserialize)]
// #[table_name = "users"]
pub struct User {
	pub id: u64,
	pub display_name: String,
	pub password: String,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "users"]

pub struct NewUser {
	pub display_name: String,
	pub password: String,
}


impl User {
	pub fn index(connection: &MysqlConnection) -> Vec<User> {
		users_schema::dsl::users
		.load::<User>(connection)
		.expect("Error loading users")
	}

	pub fn create(mut insert_user: NewUser, connection: &MysqlConnection) -> Vec<User> {
		let hash = password_hash(insert_user.password);
		insert_user.password = hash;
		diesel::insert_into(users_schema::dsl::users)
			.values(&insert_user)
			.execute(connection)
			.expect("Error inserting user");

			users_schema::dsl::users
			.load::<User>(connection)
			.expect("Error loading users")
	}
}

fn password_hash(pass: String) -> String {
	let mut hasher = Sha256::new();
	hasher.input_str(&pass);
	hasher.result_str()
}