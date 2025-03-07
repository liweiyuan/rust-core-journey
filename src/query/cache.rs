use anyhow::Result;
pub trait CacheQuery<T, R> {
    fn cache_query(&self, id: &T) -> Result<R>;
}

pub struct LocalCacheQuery;

impl CacheQuery<i32, String> for LocalCacheQuery {
    fn cache_query(&self, id: &i32) -> Result<String> {
        if *id == 1 {
            Ok(String::from("cache from local"))
        } else {
            anyhow::bail!("cache not found")
        }
    }
}

pub struct RedisCacheQuery;

impl CacheQuery<String, String> for RedisCacheQuery {
    fn cache_query(&self, str: &String) -> Result<String> {
        if str == "1" {
            Ok(String::from("cache from redis"))
        } else {
            anyhow::bail!("cache not found")
        }
    }
}
