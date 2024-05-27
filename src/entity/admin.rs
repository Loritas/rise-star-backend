use anyhow::Result;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "admins")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nickname: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No relation"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use sea_orm::ActiveValue::Set;
    use super::*;
    use crate::config::get_db_conn;
    use crate::entity::admin::Entity as Admin;
    use crate::init_config;
    use tracing::info;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_admin_select_all() {
        init_config().await.unwrap();
        let db = get_db_conn();

        let admins = Admin::find().all(db).await.unwrap();
        assert_ne!(admins.len(), 0);

        for ad in admins {
            info!("{:?}", ad);
        }
    }

    #[tokio::test]
    async fn test_admin_select_id() -> Result<()> {
        init_config().await.unwrap();
        let db = get_db_conn();

        let admin = Admin::find_by_id(1).one(db).await?;
        let admin = admin.unwrap();
        assert_eq!(admin.id, 1);
        assert_eq!(admin.nickname, "Loritas");
        Ok(())
    }

    #[traced_test]
    #[tokio::test]
    async fn test_admin_insert() -> Result<()> {
        init_config().await.unwrap();
        let db = get_db_conn();

        let admin = ActiveModel {
            nickname: Set("test_admin".to_string()),
            password: Set("test".to_string()),
            created_at: Set(chrono::Local::now().naive_local()),
            updated_at: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        };

        let admin = Entity::insert(admin).exec(db).await?;
        let id = admin.last_insert_id;
        assert!(id > 0);
        let admin = Admin::find_by_id(id).one(db).await?;
        assert_eq!(admin.unwrap().nickname, "test_admin");
        Ok(())
    }
}
