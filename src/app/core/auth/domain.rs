use super::{
    contract::{AuthCacheContract, AuthRepositoryContract},
    data::{Login, Logout, Register, SessionResponse},
};
use crate::error::Error;
use hextacy::web::http::response::MessageResponse;

pub struct Authentication<R, C> {
    pub repository: R,
    pub cache: C,
}

#[hextacy::contract]
impl<R, C> Authentication<R, C>
where
    R: AuthRepositoryContract + Send + Sync,
    C: AuthCacheContract + Send + Sync,
{
    async fn register<'a>(&self, register: Register) -> Result<SessionResponse<'a>, Error> {
        todo!()
    }

    async fn login<'a>(&self, info: Login) -> Result<SessionResponse<'a>, Error> {
        todo!()
    }

    async fn logout<'a>(&self, logout: Logout) -> Result<MessageResponse<'a>, Error> {
        todo!()
    }
}
