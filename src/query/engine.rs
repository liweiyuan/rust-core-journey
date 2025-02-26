use crate::query::LocalCacheQuery;
use anyhow::{Context, Result};

//use super::EngineType;
use super::{CacheQuery, DbQuery, LocalDbQuery};

pub struct QueryEngine<T, R> {
    pub cache: fn(&T) -> Result<R>,
    pub db: fn(&T) -> Result<R>,
}

#[derive(PartialEq, Eq)]
pub enum EngineType {
    Local,
    Redis,
    MySql,
    Oracle,
}

#[derive(PartialEq, Eq)]
pub enum QueryType {
    Default(EngineType, EngineType),
    RedisMySql(EngineType, EngineType),
    RedisOracle(EngineType, EngineType),
}

pub fn cache_query(cache_type: &QueryType) -> Result<String> {
    match cache_type {
        QueryType::Default(_, _) => {
            let cache = LocalCacheQuery {};
            cache.cache_query(&1)
        }
        _ => Err(anyhow::anyhow!("not supported cache type")),
    }
}

pub fn db_query(db_type: &QueryType) -> Result<String> {
    match db_type {
        QueryType::Default(_, _) => {
            let db = LocalDbQuery {};
            db.db_query(&1)
        }
        _ => anyhow::bail!("db not found"),
    }
}

pub fn query_order<T, R>(engine: &QueryEngine<T, R>, id: &T) -> Result<R> {
    // 查询缓存
    match (engine.cache)(id) {
        Ok(cache) => return Ok(cache),
        Err(_) => (), // 忽略缓存错误，继续查询数据库
    }

    // 查询数据库
    (engine.db)(id).context("Database query failed")
}

#[test]
fn test_query() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let result = query_order(
        &query_engine,
        &QueryType::Default(EngineType::Local, EngineType::Local),
    )?;
    assert_eq!(result, String::from("cache"));

    Ok(())
}
