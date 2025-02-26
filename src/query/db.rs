use anyhow::Result;
pub trait DbQuery<T, R> {
    fn db_query(&self, id: &T) -> Result<R>;
}

pub struct LocalDbQuery;

impl DbQuery<i32, String> for LocalDbQuery {
    fn db_query(&self, id: &i32) -> Result<String> {
        if *id == 1 {
            Ok(String::from("db from local"))
        } else {
            anyhow::bail!("mysql not found")
        }
    }
}

pub struct MySqlDbQuery;

impl DbQuery<String, String> for MySqlDbQuery {
    fn db_query(&self, str: &String) -> Result<String> {
        if str == "1" {
            Ok(String::from("db from mysql"))
        } else {
            anyhow::bail!("mysql not found")
        }
    }
}
