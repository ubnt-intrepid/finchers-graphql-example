use super::schema::users;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify(&self, password: &str) -> bool {
        bcrypt::verify(&password, &self.password).unwrap_or(false)
    }
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
