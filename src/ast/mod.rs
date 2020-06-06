mod set;
mod snippet;

pub use set::*;
pub use snippet::*;

pub struct Scope {
    pub snippets: Vec<Snippet>,
    pub sets: Vec<Set>,
}
