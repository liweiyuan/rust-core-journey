use anyhow::{Context, Result};

use crate::{
    cache::LocalCacheQuery, cache::RedisCacheQuery, db::LocalDbQuery, db::MySqlDbQuery, CacheQuery,
    DbQuery,
};

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
    Other,
}

#[derive(PartialEq, Eq)]
pub enum QueryType {
    Default(EngineType, EngineType),
    RedisMySql(EngineType, EngineType),
    RedisOracle(EngineType, EngineType),
}

pub fn cache_query(cache_type: &QueryType) -> Result<String> {
    match cache_type {
        QueryType::Default(a, _) if a == &EngineType::Local => {
            let cache = LocalCacheQuery {};
            cache.cache_query(&1)
        }
        QueryType::Default(a, _) if a == &EngineType::Redis => {
            let cache = RedisCacheQuery {};
            cache.cache_query(&"1".to_string())
        }
        _ => Err(anyhow::anyhow!("not supported cache type")),
    }
}

pub fn db_query(db_type: &QueryType) -> Result<String> {
    match db_type {
        QueryType::Default(_, b) if b == &EngineType::Local => {
            let db = LocalDbQuery {};
            db.db_query(&1)
        }

        QueryType::Default(_, b) if b == &EngineType::MySql => {
            let db = MySqlDbQuery {};
            db.db_query(&"1".to_string())
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
fn test_query_local_local() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let result = query_order(
        &query_engine,
        &QueryType::Default(EngineType::Local, EngineType::Local),
    )?;
    assert_eq!(result, String::from("cache from local"));

    Ok(())
}

#[test]
fn test_query_redis_mysql() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let result = query_order(
        &query_engine,
        &QueryType::Default(EngineType::Redis, EngineType::MySql),
    )?;
    assert_eq!(result, String::from("cache from redis"));

    Ok(())
}

#[test]
fn test_query_other_mysql() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let result = query_order(
        &query_engine,
        &QueryType::Default(EngineType::Other, EngineType::MySql),
    )?;
    assert_eq!(result, String::from("db from mysql"));

    Ok(())
}

#[test]
fn test_query_other_oracle() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    match query_order(
        &query_engine,
        &QueryType::Default(EngineType::Other, EngineType::Oracle),
    ) {
        Err(e) => assert_eq!(e.to_string(), "Database query failed"),
        Ok(_) => {}
    }
    Ok(())
}

#[test]
fn test_query_other_other() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    match query_order(
        &query_engine,
        &QueryType::Default(EngineType::Other, EngineType::Other),
    ) {
        Err(e) => assert_eq!(e.to_string(), "Database query failed"),
        Ok(_) => {}
    }
    Ok(())
}
