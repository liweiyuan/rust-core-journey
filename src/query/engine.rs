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
pub struct QueryConfig {
    pub cache_type: EngineType,
    pub db_type: EngineType,
}

#[derive(PartialEq, Eq)]
pub enum QueryType {
    Default(QueryConfig),
    RedisMySql(QueryConfig),
    RedisOracle(QueryConfig),
}

pub fn cache_query(query_type: &QueryType) -> Result<String> {
    //定义一个处理缓存查询的函数
    fn handle_cache_query<T: CacheQuery<K, String>, K>(cache: T, id: K) -> Result<String> {
        cache.cache_query(&id)
    }

    let cache_type = match query_type {
        QueryType::Default(config) => &config.cache_type,
        QueryType::RedisMySql(config) => &config.cache_type,
        QueryType::RedisOracle(config) => &config.cache_type,
    };

    match cache_type {
        EngineType::Local => {
            let cache = LocalCacheQuery {};
            handle_cache_query(cache, 1)
        }
        EngineType::Redis => {
            let cache = RedisCacheQuery {};
            handle_cache_query(cache, "1".to_string())
        }
        _ => Err(anyhow::anyhow!("not supported cache type")),
    }
}

pub fn db_query(query_type: &QueryType) -> Result<String> {
    //定义一个处理数据库查询的函数
    fn handle_db_query<T: DbQuery<K, String>, K>(db: T, id: K) -> Result<String> {
        db.db_query(&id)
    }

    let db_type = match query_type {
        QueryType::Default(config) => &config.db_type,
        QueryType::RedisMySql(config) => &config.db_type,
        QueryType::RedisOracle(config) => &config.db_type,
    };

    match db_type {
        EngineType::Local => {
            let db = LocalDbQuery {};
            handle_db_query(db, 1)
        }
        EngineType::MySql => {
            let db = MySqlDbQuery {};
            handle_db_query(db, "1".to_string())
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

    let config = QueryConfig {
        cache_type: EngineType::Local,
        db_type: EngineType::Local,
    };
    let result = query_order(&query_engine, &QueryType::Default(config))?;
    assert_eq!(result, String::from("cache from local"));

    Ok(())
}

#[test]
fn test_query_redis_mysql() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let config = QueryConfig {
        cache_type: EngineType::Redis,
        db_type: EngineType::MySql,
    };
    let result = query_order(&query_engine, &QueryType::RedisMySql(config))?;
    assert_eq!(result, String::from("cache from redis"));

    Ok(())
}

#[test]
fn test_query_other_mysql() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let config = QueryConfig {
        cache_type: EngineType::Other,
        db_type: EngineType::MySql,
    };
    let result = query_order(&query_engine, &QueryType::Default(config))?;
    assert_eq!(result, String::from("db from mysql"));

    Ok(())
}

#[test]
fn test_query_other_oracle() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let config = QueryConfig {
        cache_type: EngineType::Other,
        db_type: EngineType::Oracle,
    };
    match query_order(&query_engine, &QueryType::Default(config)) {
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

    let config = QueryConfig {
        cache_type: EngineType::Other,
        db_type: EngineType::Other,
    };
    match query_order(&query_engine, &QueryType::Default(config)) {
        Err(e) => assert_eq!(e.to_string(), "Database query failed"),
        Ok(_) => {}
    }
    Ok(())
}
