#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! assert_initialized_main_thread {
    () => {};
}
macro_rules! skip_assert_initialized {
    () => {};
}

pub use auto::*;
mod auto;
