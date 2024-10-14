use enum_dispatch::enum_dispatch;

use self::rug_backend::RugBackend;
pub mod rug_backend;
pub mod traits;

// different integer backends
// some place to save integers from different backends
// later: will implement transfer between different backends.
//        e.g. from cpu to gpu
#[enum_dispatch(ZkperIntegerTrait)]
pub enum ZkperInteger {
    Rug(RugBackend),
}
