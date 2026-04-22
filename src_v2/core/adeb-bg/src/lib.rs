//! ADead-BIB Background Runtime
//!
//! Runtime para tareas en background: I/O asíncrono, timers, event loop.

pub mod runtime;
pub mod task;
pub mod timer;
pub mod io;

pub use runtime::*;
pub use task::*;
pub use timer::*;
pub use io::*;
