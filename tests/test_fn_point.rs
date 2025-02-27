/// 学习测试函数指针
/// 模拟一个查询的接口,先从缓存查询，如果不存在，查询数据库
///
///
use anyhow::Result;
use rust_core_journey::{
    cache_query, db_query, query_order, EngineType, QueryConfig, QueryEngine, QueryType,
};
#[test]
fn test_query() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
    };

    let cfg = QueryConfig {
        cache_type: EngineType::Local,
        db_type: EngineType::Local,
    };

    //传递的是引用
    assert_eq!(
        query_order(&query_engine, &QueryType::Default(cfg))?,
        String::from("cache from local")
    );

    let cfg = QueryConfig {
        cache_type: EngineType::Local,
        db_type: EngineType::MySql,
    };
    assert_eq!(
        query_order(&query_engine, &QueryType::Default(cfg))?,
        String::from("cache from local")
    );
    let cfg = QueryConfig {
        cache_type: EngineType::Local,
        db_type: EngineType::Local,
    };
    match query_order(&query_engine, &QueryType::Default(cfg)) {
        Err(e) => assert_eq!(e.to_string(), "Database query failed"),
        Ok(_) => {}
    }

    Ok(())
}
