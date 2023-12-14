/*! guard相关配置 */
use actix_web::guard::{Guard, GuardContext};
pub struct AuthorityGuard {}
impl AuthorityGuard {
    pub fn new() -> AuthorityGuard {
        AuthorityGuard {}
    }
}
impl Guard for AuthorityGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        true
    }
}
