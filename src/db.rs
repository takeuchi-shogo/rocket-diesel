use diesel::{r2d2::{Pool, ConnectionManager, PooledConnection}, mysql::MysqlConnection};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

static DATABASE_URL: &str = "mysql://root:@localhost/diesel_test";


pub fn connect() -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}


pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}