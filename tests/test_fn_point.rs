/// 学习测试函数指针
/// 模拟一个查询的接口,先从缓存查询，如果不存在，查询数据库
///

struct QueryEngine<T, R> {
    //cache
    cache_query: fn(&T) -> Option<R>,

    //db
    db_query: fn(&T) -> Option<R>,
}

//模拟缓存查询
fn cache_query(id: &i32) -> Option<String> {
    if *id == 1 {
        Some(String::from("cache"))
    } else {
        None
    }
}

//模拟数据库查询
fn db_query(id: &i32) -> Option<String> {
    match id {
        1 => Some(String::from("db")),
        2 => Some(String::from("db2")),
        _ => None,
    }
}

fn query_order<T, R>(engine: &QueryEngine<T, R>, id: &T) -> Option<R> {
    //查询缓存
    if let Some(cache) = (engine.cache_query)(id) {
        return Some(cache);
    }

    //查询数据库
    if let Some(db) = (engine.db_query)(id) {
        return Some(db);
    }
    None
}

#[test]
fn test_query() {
    let query_engine = QueryEngine {
        cache_query,
        db_query,
    };

    assert_eq!(query_order(&query_engine, &1), Some(String::from("cache")));
    assert_eq!(query_order(&query_engine, &2), Some(String::from("db2")));
    assert_eq!(query_order(&query_engine, &3), None);
}
