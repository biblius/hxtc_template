use super::{
    components::{CacheComponentContract, RepoComponentContract},
    data::{Login, Logout, Register},
};
use crate::error::Error;
use actix_web::HttpResponse;

pub(super) struct Authentication<R, C> {
    pub repository: R,
    pub cache: C,
}

#[hextacy::contract(super)]
impl<R, C> Authentication<R, C>
where
    R: RepoComponentContract + Send + Sync,
    C: CacheComponentContract + Send + Sync,
{
    async fn register(&self, register: Register) -> Result<HttpResponse, Error> {
        todo!()
    }

    async fn login(&self, info: Login) -> Result<HttpResponse, Error> {
        todo!()
    }

    async fn logout(&self, logout: Logout) -> Result<HttpResponse, Error> {
        todo!()
    }
}
