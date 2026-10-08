#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20261008_054123_otp_codes;
mod m20261008_054228_categories;
mod m20261008_054359_transactions;
mod m20261008_054504_budgets;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20261008_054123_otp_codes::Migration),
            Box::new(m20261008_054228_categories::Migration),
            Box::new(m20261008_054359_transactions::Migration),
            Box::new(m20261008_054504_budgets::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
