pub trait DbQuery<T, R> {
    fn db_query(&self, id: &T) -> Result<R>;
}
