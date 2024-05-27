use sea_orm::{RelationDef, RelationTrait};
use sea_orm_macros::{DeriveEntityModel, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for crate::entity::admin::Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No relation"),
        }
    }
}