pub use super::_entities::otp_codes::{ActiveModel, Entity, Model};
use chrono::Duration;
use loco_rs::hash::random_string;
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
pub type OtpCodes = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl Model {
    pub async fn create_otp(db: &DatabaseConnection, user_id: i64) -> ModelResult<String> {
        let code = random_string(6).to_uppercase();
        let expires_at = chrono::Utc::now() + Duration::minutes(5);

        let active_model = ActiveModel {
            user_id: ActiveValue::Set(user_id),
            code: ActiveValue::Set(code.clone()),
            expires_at: ActiveValue::Set(expires_at.into()),
            used: ActiveValue::Set(false),
            ..Default::default()
        };

        active_model.insert(db).await?;
        Ok(code)
    }

    pub async fn verify_otp(
        db: &DatabaseConnection,
        user_id: i64,
        code: &str,
    ) -> ModelResult<bool> {
        let otp = Entity::find()
            .filter(
                model::query::condition()
                    .eq(super::_entities::otp_codes::Column::UserId, user_id)
                    .eq(super::_entities::otp_codes::Column::Code, code)
                    .eq(super::_entities::otp_codes::Column::Used, false)
                    .build(),
            )
            .one(db)
            .await?;

        if let Some(otp) = otp {
            let now: chrono::DateTime<chrono::FixedOffset> = chrono::Utc::now().into();
            if otp.expires_at >= now {
                let mut active_model = otp.into_active_model();
                active_model.used = ActiveValue::Set(true);
                active_model.update(db).await?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl ActiveModel {}
impl Entity {}
