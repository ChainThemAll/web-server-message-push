use std::{fmt, str::FromStr};

use oso::PolarClass;

#[derive(Clone, Debug, PartialEq, PolarClass)]
pub enum Permission {
    Read,
    Write,
    Create,
    Delete,
    Manage,
    Execute,
    Audit,
    Grant,
}
impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_str = match self {
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::Create => "create",
            Permission::Delete => "delete",
            Permission::Manage => "manage",
            Permission::Execute => "execute",
            Permission::Audit => "audit",
            Permission::Grant => "grant",
        };
        write!(f, "{}", lowercase_str)
    }
}

impl FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Read" => Ok(Permission::Read),
            "Write" => Ok(Permission::Write),
            "Create" => Ok(Permission::Create),
            "Delete" => Ok(Permission::Delete),
            "Manage" => Ok(Permission::Manage),
            "Execute" => Ok(Permission::Execute),
            "Audit" => Ok(Permission::Audit),
            "Grant" => Ok(Permission::Grant),
            _ => Err(format!("Invalid permission: {}", s)),
        }
    }
}
