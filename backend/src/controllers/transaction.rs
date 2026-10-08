#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::Condition;

use crate::models::_entities::transactions::{ActiveModel, Entity, Model, Column};
use crate::models::_entities::users;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub transaction_type: String,
    pub amount: i64,
    pub description: Option<String>,
    pub date: chrono::NaiveDate,
    pub category_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListQuery {
    pub month: Option<i32>,
    pub year: Option<i32>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.transaction_type = Set(self.transaction_type.clone());
        item.amount = Set(self.amount);
        item.description = Set(self.description.clone());
        item.date = Set(self.date);
        item.category_id = Set(self.category_id);
    }
}

async fn load_item(ctx: &AppContext, id: i64, user_id: i64) -> Result<Model> {
    let item = Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(&ctx.db)
        .await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(query): Query<ListQuery>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    let mut cond = Condition::all().add(Column::UserId.eq(user.id));
    
    if let (Some(m), Some(y)) = (query.month, query.year) {
        if let Some(start_date) = chrono::NaiveDate::from_ymd_opt(y, m as u32, 1) {
            let next_m = if m == 12 { 1 } else { m + 1 };
            let next_y = if m == 12 { y + 1 } else { y };
            if let Some(end_date) = chrono::NaiveDate::from_ymd_opt(next_y, next_m as u32, 1) {
                cond = cond.add(Column::Date.gte(start_date)).add(Column::Date.lt(end_date));
            }
        }
    }

    let items = Entity::find()
        .filter(cond)
        .order_by_desc(Column::Date)
        .all(&ctx.db)
        .await?;
    format::json(items)
}

#[debug_handler]
pub async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    item.user_id = Set(user.id);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i64>,
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(
    Path(id): Path<i64>,
    auth: auth::JWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let item = load_item(&ctx, id, user.id).await?;
    item.delete(&ctx.db).await?;
    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/transactions")
        .add("/", get(list))
        .add("/", post(add))
        .add("/{id}", put(update))
        .add("/{id}", delete(remove))
}
