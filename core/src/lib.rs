pub mod accounts;
pub mod minecraft;

#[cfg(feature = "native-binding")]
uniffi::setup_scaffolding!();
