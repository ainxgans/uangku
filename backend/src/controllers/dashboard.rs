#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::budgets;
use crate::models::_entities::categories;
use crate::models::_entities::transactions;
use crate::models::_entities::users;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardQuery {
    pub month: i32,
    pub year: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    pub income: i64,
    pub expense: i64,
    pub balance: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategorySummary {
    pub category_id: i64,
    pub category_name: String,
    pub spent: i64,
    pub budget: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub summary: Summary,
    pub by_category: Vec<CategorySummary>,
}

#[debug_handler]
pub async fn summary(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Query(query): Query<DashboardQuery>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    
    let start_date = chrono::NaiveDate::from_ymd_opt(query.year, query.month as u32, 1)
        .ok_or_else(|| Error::BadRequest("Invalid date".into()))?;
    
    let next_m = if query.month == 12 { 1 } else { query.month + 1 };
    let next_y = if query.month == 12 { query.year + 1 } else { query.year };
    let end_date = chrono::NaiveDate::from_ymd_opt(next_y, next_m as u32, 1)
        .ok_or_else(|| Error::BadRequest("Invalid date".into()))?;

    let txs = transactions::Entity::find()
        .filter(transactions::Column::UserId.eq(user.id))
        .filter(transactions::Column::Date.gte(start_date))
        .filter(transactions::Column::Date.lt(end_date))
        .all(&ctx.db)
        .await?;

    let mut income = 0;
    let mut expense = 0;
    
    let mut spent_by_cat: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();

    for tx in &txs {
        if tx.transaction_type == "income" {
            income += tx.amount;
        } else {
            expense += tx.amount;
            *spent_by_cat.entry(tx.category_id).or_default() += tx.amount;
        }
    }

    let bgs = budgets::Entity::find()
        .filter(budgets::Column::UserId.eq(user.id))
        .filter(budgets::Column::Month.eq(start_date))
        .all(&ctx.db)
        .await?;

    let cats = categories::Entity::find()
        .filter(categories::Column::UserId.eq(user.id))
        .all(&ctx.db)
        .await?;

    let mut by_category = Vec::new();
    for cat in cats {
        if cat.r#type != "expense" {
            continue;
        }
        let spent = spent_by_cat.get(&cat.id).copied().unwrap_or(0);
        let budget = bgs
            .iter()
            .find(|b| b.category_id == cat.id)
            .map(|b| b.limit_amount)
            .unwrap_or(0);
        
        by_category.push(CategorySummary {
            category_id: cat.id,
            category_name: cat.name,
            spent,
            budget,
        });
    }

    let res = DashboardResponse {
        summary: Summary {
            income,
            expense,
            balance: income - expense,
        },
        by_category,
    };

    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("dashboard")
        .add("/summary", get(summary))
}
