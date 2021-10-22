#[allow(dead_code)]
pub type HashMap<K, V> = ahash::AHashMap<K, V>;
#[allow(dead_code)]
pub type HashSet<K, V> = ahash::AHashSet<K, V>;

pub trait BStrParse {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error>;
}

impl BStrParse for [u8] {
    fn parse<F: lexical::FromLexical>(&self) -> Result<F, lexical::Error> {
        lexical::parse(self)
    }
}
