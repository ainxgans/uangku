use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "transactions",
            &[
                ("id", ColType::PkAuto),
                ("transaction_type", ColType::String),
                ("amount", ColType::BigInteger),
                ("description", ColType::StringNull),
                ("date", ColType::Date),
            ],
            &[("user", ""), ("category", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "transactions").await
    }
}
