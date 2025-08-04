use opherast_core::command::Command;
use std::fs;
use std::path::PathBuf;

pub struct MakeFeatureCommand;

impl Command for MakeFeatureCommand {
    fn name(&self) -> &'static str {
        "make:feature"
    }

    fn run(&self, args: &[String]) -> eyre::Result<()> {
        let name = args.get(0).ok_or_else(|| eyre::eyre!("❌ Missing feature name"))?;
        let base = PathBuf::from("src/features").join(name);

        fs::create_dir_all(&base)?;

        // Genera ogni file
        write_file(&base, "mod.rs", templates::mod_rs())?;
        write_file(&base, "handler.rs", templates::handler_rs(name))?;
        write_file(&base, "service.rs", templates::service_rs(name))?;
        write_file(&base, "repository.rs", templates::repository_rs(name))?;
        write_file(&base, "model.rs", templates::model_rs(name))?;
        write_file(&base, "dto.rs", templates::dto_rs(name))?;

        println!("✅ Feature '{}' created at {:?}", name, base);
        Ok(())
    }
}

fn write_file(dir: &PathBuf, file: &str, content: String) -> eyre::Result<()> {
    let path = dir.join(file);
    std::fs::write(&path, content)?;
    Ok(())
}

mod templates {
    pub fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

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
use crate::features::{0}::dto::Create{1}Request;
use crate::features::{0}::repository::Default{1}Repository;
use crate::features::{0}::service::{1}Service;
use crate::shared::result::AppResult;
use crate::db::adapter::DbAdapter;
use std::sync::Arc;

pub async fn create_{0}(
    Extension(db): Extension<Arc<dyn DbAdapter>>,
    Json(payload): Json<Create{1}Request>,
) -> AppResult<Json<String>> {{
    let repo = Default{1}Repository {{ db: db.as_ref() }};
    let service = {1}Service::new(&repo);
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
            r#"use crate::features::{0}::dto::Create{1}Request;
use crate::features::{0}::repository::{{{1}Repository, Default{1}Repository}};
use crate::shared::result::AppResult;

pub struct {1}Service<'a> {{
    pub repo: &'a dyn {1}Repository,
}}

impl<'a> {1}Service<'a> {{
    pub fn new(repo: &'a dyn {1}Repository) -> Self {{
        Self {{ repo }}
    }}

    pub async fn create(&self, req: Create{1}Request) -> AppResult<()> {{
        let _ = self.repo.insert(req).await?;
        Ok(())
    }}
}}
"#,
            name,
            capitalize(name)
        )
    }

    pub fn repository_rs(name: &str) -> String {
        let cap = capitalize(name);
        format!(
            r#"use crate::db::adapter::DbAdapter;
use crate::features::{0}::dto::Create{1}Request;
use crate::features::{0}::model::{1};
use crate::shared::result::AppResult;
use chrono::Utc;
use async_trait::async_trait;

#[async_trait]
pub trait {1}Repository: Send + Sync {{
    async fn insert(&self, req: Create{1}Request) -> AppResult<{1}>;
}}

pub struct Default{1}Repository<'a> {{
    pub db: &'a dyn DbAdapter,
}}

#[async_trait]
impl<'a> {1}Repository for Default{1}Repository<'a> {{
    async fn insert(&self, req: Create{1}Request) -> AppResult<{1}> {{
        let entity = {1} {{
            id: Default::default(),
            created_at: Utc::now(),
            // ... altri campi
        }};

        let saved = self.db.insert(&entity).await?;
        Ok(saved)
    }}
}}
"#,
            name,
            cap
        )
    }

    pub fn model_rs(name: &str) -> String {
        format!(
            r#"use ulid::Ulid;
use chrono::{{DateTime, Utc}};
use serde::{{Serialize, Deserialize}};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
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
}
