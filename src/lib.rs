#[derive(Clone, Default, Debug)]
pub struct Tracer {
    pub log: Vec<String>,
    pub enabled: bool,
}

#[allow(missing_docs)]
pub mod wrapper {
    use super::*;
    zucchero::init!(Tracer, trace);
}

pub fn enable() {
    wrapper::trace(|s| s.enabled = true);
}

pub fn disable() {
    wrapper::trace(|s| s.enabled = false);
}

pub fn trace<R: Default>(f: impl FnOnce(&mut Tracer) -> R) -> R {
    wrapper::trace(|s| if s.enabled { f(s) } else { R::default() })
}

pub fn dump() -> Vec<String> {
    trace(|state| {
        let log = state.log.clone();
        state.log.clear();
        log
    })
}
