//! This module contains the models for Role-Based Access Control (RBAC) in the application. It
//! defines the structures for roles, permissions, and their relationships with users and servers.
//! there's no zanzibar demonstration here this is just traditional RBAC implementation with roles,
//! permissions, and their relationships with users and servers.
//!
use jiff::Timestamp;
use uuid::Uuid;

use crate::models::{Server, User};

#[derive(Debug, Clone, Serialize, Model, Deserialize, new, TypedBuilder)]
pub struct Role {
    #[auto]
    #[key]
    pub id:          Uuid,
    #[unique]
    pub identifier:  String,
    pub description: Option<String>,
    #[auto]
    created_at:      Timestamp,
    #[auto]
    updated_at:      Timestamp,
}

#[derive(Debug, Clone, Serialize, Model, Deserialize, new, TypedBuilder)]
pub struct Permission {
    // product.read
    #[auto]
    #[key]
    pub id:          Uuid,
    pub resource:    String,
    #[unique]
    pub action:      String,
    pub description: Option<String>,
    #[auto]
    created_at:      Timestamp,
    #[auto]
    updated_at:      Timestamp,
}

// Role Permissions mapping
#[derive(Debug, Clone, Model, new, TypedBuilder)]
pub struct RolePermission {
    #[auto]
    #[key]
    pub id:            Uuid,
    #[index]
    pub role_id:       Uuid,
    #[unique]
    #[index]
    pub permission_id: Uuid,
    #[auto]
    created_at:        Timestamp,
    #[auto]
    updated_at:        Timestamp,
    // Relationships
    #[belongs_to(key = role_id, references = id)]
    role:              toasty::Deferred<Role>,
    #[belongs_to(key = permission_id, references = id)]
    permission:        toasty::Deferred<Permission>,
}

#[derive(Debug, Clone, Model, new, TypedBuilder)]
pub struct SubjectRole {
    #[auto]
    #[key]
    pub id:         Uuid,
    #[index]
    pub subject_id: Uuid,
    #[unique]
    #[index]
    pub role_id:    Uuid,
    #[auto]
    created_at:     Timestamp,
    #[auto]
    updated_at:     Timestamp,
    // Relationships
    #[belongs_to(key = role_id, references = id)]
    role:           toasty::Deferred<Role>,
    #[belongs_to(key = subject_id, references = id)]
    subject:        toasty::Deferred<User>,
}

#[derive(Debug, Clone, Model, new, TypedBuilder)]
pub struct ServerRole {
    #[auto]
    #[key]
    pub id:                 Uuid,
    pub description:        Option<String>,
    #[index]
    pub role_id:            Uuid,
    #[belongs_to(key = role_id, references = id)]
    pub role:               toasty::Deferred<Role>,
    #[index]
    pub server_id:          Uuid,
    #[belongs_to(key = server_id, references = id)]
    pub server:             toasty::Deferred<Server>,
    #[index]
    pub subject_id:         Uuid,
    #[belongs_to(key = subject_id, references = id)]
    pub subject:            toasty::Deferred<User>,
    #[index]
    pub granted_by:         Uuid,
    #[belongs_to(key = granted_by, references = id)]
    pub granted_by_subject: toasty::Deferred<User>,
}

#[cfg(test)]
mod tests {
    use anyhow::{Result, anyhow};
    use toasty::{query,
                 schema::{Load, Model}};

    use super::*;
    use crate::models::Server;

    #[tokio::test]
    async fn walk_through() -> Result<()> {
        let mut db = crate::toasty_database::connect_toasty_database().await?;

        // Given a role
        let role = create!(Role {
            identifier:  "admin".to_string(),
            description: Some("Administrator role".to_string()),
        })
        .exec(&mut db)
        .await?;
        println!("Role: {:?}", role);

        // Given some permission
        let permission = create!(Permission::[
            {
                resource:    "server".to_string(),
                action:      "read".to_string(),
                description: Some("Read server permission".to_string()),
            },
            {
                resource:    "server".to_string(),
                action:      "write".to_string(),
                description: Some("Write server permission".to_string()),
            },            {
                resource:    "server".to_string(),
                action:      "update".to_string(),
                description: Some("Update server permission".to_string()),
            }
        ])
        .exec(&mut db)
        .await?;

        println!("Permissions: {:?}", permission);

        // Let assign permissions to role
        let role_permissions = permission
            .into_iter()
            .fold(RolePermission::create_many(), |b, p| b.with_item(|c| RolePermission::create().role(role.clone()).permission(p)))
            .exec(&mut db)
            .await?;

        println!("Role Permissions: {:?}", role_permissions);

        // let create a user and a server and resources
        let user = create!(User { name: "John Doe".to_string(), username: "johndoe".to_string() })
            .exec(&mut db)
            .await?;

        println!("User: {:?}", user);

        // let give our user this role
        let subject_role = SubjectRole::create()
            .role(role.clone())
            .subject(user.clone())
            .exec(&mut db)
            .await?;

        println!("Subject Role: {:?}", subject_role);

        // Given a server
        let server = create!(Server { name: "My Server".to_string(), address: "127.0.0.1:3000".to_string() })
            .exec(&mut db)
            .await?;

        println!("Server: {:?}", server);

        // let give this user this role on this server
        let server_role = ServerRole::create()
            .role(role)
            .server(server)
            .granted_by(user.id.clone())
            .subject(user)
            .exec(&mut db)
            .await?;

        println!("Server Role: {:?}", server_role);

        Ok(())
    }

    // Let get a user permissions on a server
    #[tokio::test]
    async fn get_user_permissions_on_server() -> Result<()> {
        let mut db = crate::toasty_database::connect_toasty_database().await?;
        // Let check if this user has the admin permision
        let user_id = Uuid::parse_str("019f234e-303b-7301-aa0e-b1821114b58e")?;
        let server_name = "My Server".to_string();

        let user = query!(User FILTER .id == #user_id)
            .exec(&mut db)
            .await?
            .into_iter()
            .next()
            .ok_or(anyhow!("User not found"))?;

        let server = query!(Server FILTER .name == #server_name)
            .exec(&mut db)
            .await?
            .into_iter()
            .next()
            .ok_or(anyhow!("Server not found"))?;
        let server_id = server.id;

        println!("User: {:#?} - Server: {:#?}", user, server);

        #[derive(Debug, Clone, Serialize, Deserialize, Model, new, TypedBuilder)]
        pub struct UserPermissionOnServer {
            #[key]
            pub permission_id:          Uuid,
            pub role_id:                Uuid,
            pub role_identifier:        String,
            pub role_description:       Option<String>,
            pub resource:               String,
            pub action:                 String,
            pub permission_description: Option<String>,
        }

        fn user_has_permission_on_server(user_server_permissions: &[UserPermissionOnServer], resource: &str, action: &str) -> bool {
            user_server_permissions
                .iter()
                .any(|perm| perm.resource == resource && perm.action == action)
        }

        // Using join we want to get the row where it's possible that we have the user_id, server_id and the
        // role_id then join to get all the permission of that role
        let rows = toasty::sql::query(
            r#"
    SELECT
        p.id          AS permission_id,
        r.id          AS role_id,
        r.identifier  AS role_identifier,
        r.description AS role_description,
        p.resource    AS resource,
        p.action      AS action,
        p.description AS permission_description
    FROM server_roles ser
    JOIN roles r             ON r.id = ser.role_id
    JOIN role_permissions rp ON rp.role_id = r.id
    JOIN permissions p       ON p.id = rp.permission_id
    WHERE ser.subject_id = $1
      AND ser.server_id = $2
    "#,
        )
        .bind(user_id)
        .bind(server_id)
        .exec(&mut db)
        .await?;

        let user_server_permissions: Vec<UserPermissionOnServer> = Vec::load(rows.into())?;
        println!("User Permissions on Server: {:#?}", user_server_permissions);

        if user_has_permission_on_server(&user_server_permissions, "server", "read")
        {
            println!("User has read permission on server");
        }
        else
        {
            println!("User does not have read permission on server");
        }
        Ok(())
    }
}
