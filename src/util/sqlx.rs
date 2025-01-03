pub type QueryResult<T> = sqlx::Result<Option<T>>;

macro_rules! map_query_result {
    ($res:expr) => {
        match $res {
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
            Ok(row) => Ok(Some(row)),
        }
    };
}

pub(crate) use map_query_result;

macro_rules! try_unwrap_query_result {
    ($res:ident) => {
        if let Err(sqlx::Error::RowNotFound) = $res {
            return Ok(None);
        } else if let Err(e) = $res {
            return Err(e);
        }
        let $res = $res.unwrap();
    };
}

pub(crate) use try_unwrap_query_result;
