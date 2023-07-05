//! Theme dependencies for in-browser search. Not included in mdbook when
//! the "search" cargo feature is disabled.

pub static JS: &[u8] = include_bytes!("searcher.js");
pub static MARK_JS: &[u8] = include_bytes!("mark.min.js");
pub static ELASTICLUNR_JS: &[u8] = include_bytes!("elasticlunr.min.js");

#[cfg(feature = "zh")]
pub static LUNR_ZH_JS: &[u8] = include_bytes!("lunr.zh.js");
#[cfg(feature = "zh")]
pub static LUNR_STEMMER_SUPPORT_JS: &[u8] = include_bytes!("lunr.stemmer.support.js");
#[cfg(feature = "zh")]
pub static JIEBA_WASM_JS: &[u8] = include_bytes!("jieba_rs_wasm.js");
#[cfg(feature = "zh")]
pub static JIEBA_WASM: &[u8] = include_bytes!("jieba_rs_wasm_bg.wasm");
