pub mod dag;
pub mod hmm;

#[macro_export]
macro_rules! embed_data {
    ($name:ident,$t:ty,$byte:ident,$path:literal) => {
        pub const $byte: &'static [u8] = include_bytes!($path);
        pub static $name: ::std::sync::LazyLock<$t> = ::std::sync::LazyLock::new(|| {
            let mut aligned = rkyv::util::AlignedVec::<16>::new();
            aligned.extend_from_slice($byte);
            rkyv::from_bytes::<$t, rkyv::rancor::Error>(&aligned).expect(concat!(
                "Failed to crate ",
                stringify!($name),
                "stringify!($name)",
                $path
            ))
        });
    };
}
