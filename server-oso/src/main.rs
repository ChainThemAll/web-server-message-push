extern crate oso;

use std::{env, path::Path};

use oso::{Oso, PolarClass, ToPolar};
use server_oso::{permission::Permission, role::Role, Repository, User};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建 Oso 实例
    let mut oso = Oso::new();

    // 注册 Rust 结构
    oso.register_class(
        oso::Class::builder::<Repository>()
            .name("Repository")
            .add_attribute_getter("permissions", |receiver: &Repository| {
                receiver.permissions.clone()
            })
            .build(),
    )?;
    oso.register_class(
        oso::Class::builder::<User>()
            .name("User")
            .add_attribute_getter("role", |receiver: &User| receiver.role.clone())
            .build(),
    )?;
    // 获取 Cargo manifest 文件的目录
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // 构建 policy.polar 文件的路径
    let policy_path = format!("{}/src/policy.polar", manifest_dir);
    oso.load_files(vec![Path::new(&policy_path)])?;

    // 创建 User 和 Repository 实例
    let user = User {
        name: String::from("Alice"),
        role: Role::Administrator.to_string(),
    };
    let repo = Repository {
        name: String::from("my-repo"),
        permissions: vec![Permission::Read.to_string(), Permission::Write.to_string()],
    };

    // 检查权限
    assert!(oso.is_allowed(user.clone(), Permission::Read.to_string(), repo.clone())?);
    assert!(oso.is_allowed(user.clone(), Permission::Write.to_string(), repo.clone())?);
    assert!(!oso.is_allowed(user.clone(), Permission::Delete.to_string(), repo.clone())?);

    println!("Permission checks passed.");
    Ok(())
}

pub fn oso_init() -> Result<Oso, Box<dyn std::error::Error>> {
    // 创建 Oso 实例
    let mut oso = Oso::new();

    // 注册 Rust 结构
    oso.register_class(
        oso::Class::builder::<Repository>()
            .name("Repository")
            .add_attribute_getter("permissions", |receiver: &Repository| {
                receiver.permissions.clone()
            })
            .build(),
    )?;
    oso.register_class(
        oso::Class::builder::<User>()
            .name("User")
            .add_attribute_getter("role", |receiver: &User| receiver.role.clone())
            .build(),
    )?;
    // 获取 Cargo manifest 文件的目录
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // 构建 policy.polar 文件的路径
    let policy_path = format!("{}/src/policy.polar", manifest_dir);
    oso.load_files(vec![Path::new(&policy_path)])?;
    Ok(oso)
}
