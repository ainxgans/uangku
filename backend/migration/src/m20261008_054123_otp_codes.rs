use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "otp_codes",
            &[
                ("id", ColType::PkAuto),
                ("code", ColType::String),
                ("expires_at", ColType::TimestampWithTimeZone),
                ("used", ColType::Boolean),
            ],
            &[("user", "")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "otp_codes").await
    }
}
