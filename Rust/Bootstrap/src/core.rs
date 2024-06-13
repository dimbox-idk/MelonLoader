use ctor::ctor;

use crate::{console, errors::DynErr, hooks, internal_failure, base_assembly};

#[ctor]
fn startup() {
    init().unwrap_or_else(|e| {
        internal_failure!("Failed to initialize MelonLoader: {}", e.to_string());
    })
}

fn init() -> Result<(), DynErr> {
    console::init()?;

    base_assembly::init(crate::runtime!()?)?;

    hooks::init_hook::hook()?;

    console::null_handles()?;

    Ok(())
}

pub fn shutdown() {
    std::process::exit(0);
}
