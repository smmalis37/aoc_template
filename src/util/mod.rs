pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type HashSet<V> = rustc_hash::FxHashSet<V>;

pub trait BStrParse {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error>;
}

impl BStrParse for [u8] {
    #[inline]
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error> {
        lexical::parse(self)
    }
}

#[inline]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub const fn bytelines(&x: &u8) -> bool {
    x == b'\n'
}

pub trait WithCapacity {
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V> WithCapacity for HashMap<K, V> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, Default::default())
    }
}

impl<V> WithCapacity for HashSet<V> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, Default::default())
    }
}
