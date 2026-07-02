use jiff::Timestamp;
use typed_builder::TypedBuilder;
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize, Model, new, TypedBuilder)]
pub struct Resource {
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

#[cfg(test)]
mod tests {

    use jiff::Timestamp;
    use toasty::stmt::Value;
    use uuid::Uuid;

    use crate::{models::User, toasty_database::create_toasty_database};

    #[tokio::test]
    async fn test_psql_connection() -> anyhow::Result<()> {
        let mut db = create_toasty_database().await?;

        let row = toasty::sql::query("SELECT 1 as one").exec(&mut db).await?;
        println!("Row: {:?}", row);
        Ok(())
    }

    // #[tokio::test]
    // async fn test_user_posts() -> anyhow::Result<()> {
    //     let mut db = create_toasty_database().await?;

    //     let user_id = "019f1df6-7655-7a81-82ed-85f1531e47d7".parse::<Uuid>()?;

    //     // Load user with deferred posts relationship
    //     let user = toasty::query!(User FILTER .id == #user_id)
    //         .exec(&mut db)
    //         .await?
    //         .into_iter()
    //         .next()
    //         .unwrap();
    //     println!("User: {:?}", user);

    //     let post = Post::create()
    //         .content("Oh we got that working too!".to_string())
    //         .user(user)
    //         .exec(&mut db)
    //         .await?;

    //     println!("Created Post: {:?}", post);

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn perform_join_operation() -> anyhow::Result<()> {
    //     use crate::toasty::schema::Load;
    //     let mut db = create_toasty_database().await?;

    //     let user_id = "019f1df6-7655-7a81-82ed-85f1531e47d7".parse::<Uuid>()?;

    //     #[derive(Debug, Clone, toasty::Model)]
    //     struct UserWithPosts {
    //         #[key]
    //         id: Uuid,
    //         name: String,
    //         username: String,
    //         created_at: Timestamp,
    //         updated_at: Timestamp,
    //         post_id: Uuid,
    //         content: String,
    //         post_created_at: Timestamp,
    //         post_updated_at: Timestamp,
    //     }

    //     // Alternative: raw SQL query (if you need complex joins)
    //     let rows = toasty::sql::query(
    //             r#"
    //             SELECT u.id, u.name, u.username, u.created_at, u.updated_at, p.id as post_id,
    // p.content, p.created_at as post_created_at, p.updated_at as post_updated_at
    // FROM users AS u             LEFT JOIN posts AS p ON u.id = p.user_id
    //             WHERE u.id = $1
    //             "#,
    //         )
    //         .bind(user_id)
    //         .exec(&mut db)
    //         .await?;

    //     let user_posts: Vec<UserWithPosts> = Vec::load(Value::from(rows))?;
    //     println!("User with posts: {:#?}", user_posts);

    //     Ok(())
    // }
}
