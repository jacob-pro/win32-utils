#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "net")]
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub mod net;

pub mod error;
pub mod str;
