use extism_pdk::*;
use bip39::{Mnemonic, Language};

const LANG: Language = Language::English;

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[plugin_fn]
pub fn createMnemonic() -> FnResult<String> {
    let entropy = "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f".as_bytes();
    let mnemonic = Mnemonic::from_entropy_in(LANG, &entropy).unwrap();

    Ok(mnemonic.to_string())
}
