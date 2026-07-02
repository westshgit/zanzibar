use jiff::Timestamp;
use typed_builder::TypedBuilder;
use uuid::Uuid;

pub mod rbac;

#[derive(Debug, Clone, toasty::Model, TypedBuilder, new)]
pub struct User {
    #[key]
    #[auto]
    id:         Uuid,
    name:       String,
    username:   String,
    #[auto]
    created_at: Timestamp,
    #[auto]
    updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Model, Deserialize, new, TypedBuilder)]
pub struct Server {
    #[key]
    #[auto]
    id:          Uuid,
    #[unique]
    name:        String,
    description: Option<String>,
    address:     String,
    #[auto]
    created_at:  Timestamp,
    #[auto]
    updated_at:  Timestamp,
}
