mod results;

use results::CrudResult;

pub(crate) trait Insert<T, R> {
    fn insert(entity: &T) -> CrudResult<R>;
}

pub(crate) trait InsertMany<T, R> {
    fn insert(entity: &[&T]) -> CrudResult<R>;
}