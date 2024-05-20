use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "admins")]
pub struct Admin {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::entity::prelude::*;

    #[tokio::test]
    async fn test_admin_crud() {

    }
}