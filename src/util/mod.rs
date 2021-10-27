#[allow(dead_code)]
pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
#[allow(dead_code)]
pub type HashSet<V> = rustc_hash::FxHashSet<V>;

pub trait BStrParse {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error>;
}

impl BStrParse for [u8] {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error> {
        lexical::parse(self)
    }
}
