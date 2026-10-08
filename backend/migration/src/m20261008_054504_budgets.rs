use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "budgets",
            &[
                ("id", ColType::PkAuto),
                ("month", ColType::Date),
                ("limit_amount", ColType::BigInteger),
            ],
            &[("user", ""), ("category", "")],
        )
        .await?;

        m.create_index(
            Index::create()
                .name("idx_budgets_unique_user_category_month")
                .table(Alias::new("budgets"))
                .col(Alias::new("user_id"))
                .col(Alias::new("category_id"))
                .col(Alias::new("month"))
                .unique()
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "budgets").await
    }
}
