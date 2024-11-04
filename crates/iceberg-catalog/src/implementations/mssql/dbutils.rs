use crate::api::ErrorModel;
use deadpool;
use tiberius;

pub(crate) trait DBErrorHandler
where
    Self: ToString + Sized + Send + Sync + std::error::Error + 'static,
{
    fn into_error_model(self, message: String) -> ErrorModel {
        ErrorModel::internal(message, "DatabaseError", Some(Box::new(self)))
    }
}

impl DBErrorHandler for deadpool::managed::PoolError<tiberius::error::Error> {
    fn into_error_model(self, message: String) -> ErrorModel {
        todo!()
    }
}

impl DBErrorHandler for tiberius::error::Error {
    fn into_error_model(self, message: String) -> ErrorModel {
        todo!()
    }
}
