/// 学习测试函数指针
/// 模拟一个查询的接口,先从缓存查询，如果不存在，查询数据库
///

struct QueryEngine<T, R> {
    //cache
    cache_query: fn(&T) -> Result<R, String>,

    //db
    db_query: fn(&T) -> Result<R, String>,
}

//模拟缓存查询
fn cache_query(id: &i32) -> Result<String, String> {
    if *id == 1 {
        Ok(String::from("cache"))
    } else {
        Err(String::from("cache not found"))
    }
}

fn db_query(id: &i32) -> Result<String, String> {
    match id {
        1 => Ok(String::from("db")),
        2 => Ok(String::from("db2")),
        _ => Err(String::from("db not found")),
    }
}

fn query_order<T, R>(engine: &QueryEngine<T, R>, id: &T) -> Result<R, String> {
    // 查询缓存
    match (engine.cache_query)(id) {
        Ok(cache) => return Ok(cache),
        Err(_) => (), // 忽略缓存错误，继续查询数据库
    }

    // 查询数据库
    match (engine.db_query)(id) {
        Ok(db) => return Ok(db),
        Err(e) => return Err(e), // 返回数据库查询错误
    }
}

#[test]
fn test_query() {
    let query_engine = QueryEngine {
        cache_query,
        db_query,
    };

    assert_eq!(query_order(&query_engine, &1), Ok(String::from("cache")));
    assert_eq!(query_order(&query_engine, &2), Ok(String::from("db2")));
    assert_eq!(
        query_order(&query_engine, &3),
        Err("db not found".to_string())
    );
}
