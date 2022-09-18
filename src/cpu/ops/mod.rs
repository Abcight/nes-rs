pub use super::CPU;

#[macro_export]
macro_rules! op {
	( $mod:ident ) => {
		pub mod $mod;
		pub use $mod::*;
	};
}

op!(lda);
op!(tax);
op!(inx);
op!(dex);
op!(brk);
