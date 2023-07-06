pub mod session;
pub mod user;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("Entry does not exist")]
    DoesNotExist,
    #[error("Driver: {0}")]
    Driver(#[from] hextacy::driver::DriverError),
    #[error("SeaORM: {0}")]
    SeaORM(#[from] sea_orm::DbErr),
}
