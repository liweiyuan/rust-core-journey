/// 学习测试函数指针
/// 模拟一个查询的接口,先从缓存查询，如果不存在，查询数据库
///
///
use anyhow::{Context, Result};

struct QueryEngine<T, R> {
    //cache
    cache_query: fn(&T) -> Result<R>,
    //db
    db_query: fn(&T) -> Result<R>,
}

//模拟缓存查询
fn cache_query(id: &i32) -> Result<String> {
    if *id == 1 {
        Ok(String::from("cache"))
    } else {
        anyhow::bail!("cache not found")
    }
}

fn db_query(id: &i32) -> Result<String> {
    match id {
        1 => Ok(String::from("db")),
        2 => Ok(String::from("db2")),
        _ => anyhow::bail!("db not found"),
    }
}

fn query_order<T, R>(engine: &QueryEngine<T, R>, id: &T) -> Result<R> {
    // 查询缓存
    match (engine.cache_query)(id) {
        Ok(cache) => return Ok(cache),
        Err(_) => (), // 忽略缓存错误，继续查询数据库
    }

    // 查询数据库
    (engine.db_query)(id).context("Database query failed")
}

#[test]
fn test_query() -> Result<()> {
    let query_engine = QueryEngine {
        cache_query,
        db_query,
    };

    //传递的是引用
    assert_eq!(query_order(&query_engine, &1)?, String::from("cache"));
    assert_eq!(query_order(&query_engine, &2)?, String::from("db2"));
    match query_order(&query_engine, &3) {
        Err(e) => assert_eq!(e.to_string(), "Database query failed"),
        Ok(_) => panic!("Expected an error"),
    }

    Ok(())
}
