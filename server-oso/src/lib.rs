use oso::PolarClass;

use permission::Permission;
use role::Role;
pub mod permission;
pub mod register;
pub mod role;

#[derive(Clone, PolarClass)]
pub struct Repository {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Clone, PolarClass)]
pub struct User {
    pub name: String,

    pub role: String,
}
