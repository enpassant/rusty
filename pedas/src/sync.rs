#![macro_use]
pub mod list;

#[macro_export]
macro_rules! list {
    () => (
        List::empty()
    );
    ($elem:expr) => (
        List::new(&[$elem])
    );
    ($($x:expr),+ $(,)?) => ( {
        List::new(&[$($x),+])
    });
}


