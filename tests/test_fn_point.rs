/// 学习测试函数指针
/// 模拟一个查询的接口,先从缓存查询，如果不存在，查询数据库
///
///
use anyhow::Result;
use rust_core_journey::{cache_query, db_query, query_order, QueryEngine};
#[test]
fn test_query() -> Result<()> {
    let query_engine = QueryEngine {
        cache: cache_query,
        db: db_query,
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
