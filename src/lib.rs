#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "net")]
#[cfg_attr(docsrs, doc(cfg(feature = "net")))]
pub mod net;

#[cfg(feature = "window")]
#[cfg_attr(docsrs, doc(cfg(feature = "window")))]
pub mod window;

#[cfg(feature = "instance")]
#[cfg_attr(docsrs, doc(cfg(feature = "instance")))]
pub mod instance;

pub mod error;
pub mod str;

#[allow(non_snake_case)]
pub mod macros;
