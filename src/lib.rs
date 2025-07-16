#[derive(Clone, Default, Debug)]
pub struct Tracer {
    pub log: Vec<String>,
}

#[allow(missing_docs)]
pub mod wrapper {
    use super::*;
    zucchero::init!(Tracer, trace);
}

pub use wrapper::trace;

pub fn dump() -> Vec<String> {
    trace(|state| {
        let log = state.log.clone();
        state.log.clear();
        log
    })
}
