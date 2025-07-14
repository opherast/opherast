pub fn mod_rs() -> String {
    r#"pub mod handler;
pub mod service;
pub mod repository;
pub mod model;
pub mod dto;
"#
    .to_string()
}

pub fn handler_rs(name: &str) -> String {
    format!(
        r#"use axum::{{Json, Extension}};
use crate::{0}::dto::Create{1}Request;
use crate::{0}::service::{1}Service;
use crate::shared::result::AppResult;
use crate::db::adapter::DbAdapter;
use std::sync::Arc;

pub async fn create_{0}(
    Extension(db): Extension<Arc<dyn DbAdapter>>,
    Json(payload): Json<Create{1}Request>,
) -> AppResult<Json<String>> {{
    let service = {1}Service::new(db.as_ref());
    service.create(payload).await?;
    Ok(Json("Created.".into()))
}}
"#,
        name,
        capitalize(name)
    )
}

pub fn service_rs(name: &str) -> String {
    format!(
        r#"use crate::{0}::dto::Create{1}Request;
use crate::shared::result::AppResult;
use crate::db::adapter::DbAdapter;

pub struct {1}Service<'a> {{
    pub db: &'a dyn DbAdapter,
}}

impl<'a> {1}Service<'a> {{
    pub fn new(db: &'a dyn DbAdapter) -> Self {{
        Self {{ db }}
    }}

    pub async fn create(&self, req: Create{1}Request) -> AppResult<()> {{
        // self.db.insert::<YourModel>(&req).await?;
        Ok(())
    }}
}}
"#,
        name,
        capitalize(name)
    )
}

pub fn model_rs(name: &str) -> String {
    format!(
        r#"use ulid::Ulid;
use chrono::{{DateTime, Utc}};
use serde::{{Serialize, Deserialize}};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct {0} {{
    pub id: Ulid,
    pub created_at: DateTime<Utc>,
    // Aggiungi i tuoi campi qui
}}
"#,
        capitalize(name)
    )
}

pub fn dto_rs(name: &str) -> String {
    format!(
        r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Deserialize)]
pub struct Create{0}Request {{
    // pub field: String,
}}
"#,
        capitalize(name)
    )
}

pub fn repository_rs(name: &str) -> String {
    let feature_snake = name.to_lowercase();
    let feature_pascal = capitalize(name);

    format!(
        r#"//! Repository override for feature: {feature_snake}
//! Use this to intercept DB operations with domain-specific logic.
//!
//! This file is optional and can override default behavior provided by the central DbAdapter.

use crate::db::adapter::DbAdapter;
use crate::{feature_snake}::dto::Create{feature_pascal}Request;
use crate::{feature_snake}::model::{feature_pascal};
use anyhow::Result;
use chrono::Utc;
use tracing::info;

/// Insert a new {feature_snake}.
/// Override this function to apply domain-specific transformations,
/// validation, enrichment or fallback logic.
pub async fn insert_{feature_snake}(
    db: &dyn DbAdapter,
    req: Create{feature_pascal}Request,
) -> Result<{feature_pascal}> {{
    info!("Inserting new {feature_snake}: {{:?}}", req);

    let entity = {feature_pascal} {{
        id: Default::default(),
        email: req.email,
        display_name: req.display_name,
        created_at: Utc::now(),
    }};

    let result = db.insert(&entity).await;

    match result {{
        Ok(v) => Ok(v),
        Err(e) => {{
            tracing::warn!("Insert failed for {feature_snake}: {{:?}}", e);
            // TODO: fallback logic, retry, or custom error
            Err(e)
        }}
    }}
}}
"#,
        feature_snake = feature_snake,
        feature_pascal = feature_pascal
    )
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
    }
}
