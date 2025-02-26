use anyhow::{Context, Result};

pub struct QueryEngine<T, R> {
    pub cache_query: fn(&T) -> Result<R>,
    pub db_query: fn(&T) -> Result<R>,
}

//模拟缓存查询
pub fn cache_query(id: &i32) -> Result<String> {
    if *id == 1 {
        Ok(String::from("cache"))
    } else {
        anyhow::bail!("cache not found")
    }
}

pub fn db_query(id: &i32) -> Result<String> {
    match id {
        1 => Ok(String::from("db")),
        2 => Ok(String::from("db2")),
        _ => anyhow::bail!("db not found"),
    }
}

pub fn query_order<T, R>(engine: &QueryEngine<T, R>, id: &T) -> Result<R> {
    // 查询缓存
    match (engine.cache_query)(id) {
        Ok(cache) => return Ok(cache),
        Err(_) => (), // 忽略缓存错误，继续查询数据库
    }

    // 查询数据库
    (engine.db_query)(id).context("Database query failed")
}
