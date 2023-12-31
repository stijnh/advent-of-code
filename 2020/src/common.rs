pub use anyhow::{anyhow, bail, Context as _, Error};
pub use itertools::{all, any, enumerate, max, min, zip, Itertools};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Ord, Ordering};
use std::default::Default;
use std::iter::{Map, Sum};
pub use std::mem::swap;
use std::sync::Mutex;

pub type HashMap<K, V> = std::collections::HashMap<K, V, fnv::FnvBuildHasher>;
pub type HashSet<K> = std::collections::HashSet<K, fnv::FnvBuildHasher>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[allow(dead_code)]
pub fn default<T: Default>() -> T {
    T::default()
}

pub fn read_input(filename: &str) -> Result<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let path = format!("inputs/{}", filename);
    let f = File::open(&path).with_context(|| format!("failed to open {}", path))?;

    BufReader::new(f)
        .lines()
        .collect::<Result<_, _>>()
        .with_context(|| format!("error while reading {}", path))
}

pub fn cmp<T: Ord>(lhs: T, rhs: T) -> Ordering {
    Ord::cmp(&lhs, &rhs)
}

pub fn map<I, F, B>(iter: I, fun: F) -> Map<I::IntoIter, F>
where
    I: IntoIterator,
    F: FnMut(I::Item) -> B,
{
    iter.into_iter().map(fun)
}

pub fn sum<I>(iter: I) -> I::Item
where
    I: IntoIterator,
    I::Item: Sum,
{
    iter.into_iter().sum()
}

lazy_static! {
    static ref PATTERN_CACHE: Mutex<HashMap<String, &'static Regex>> = Mutex::default();
}

fn compile(pattern: &str) -> &'static Regex {
    let mut guard = PATTERN_CACHE.lock().unwrap();
    if let Some(p) = guard.get(pattern) {
        return p;
    }

    let result = Box::leak(Box::new(Regex::new(pattern).unwrap()));
    guard.insert(pattern.to_string(), result);

    result
}

#[allow(dead_code)]
pub fn is_match(pattern: &str, string: &str) -> bool {
    compile(pattern).is_match(string)
}

#[allow(dead_code)]
pub fn find<'t>(pattern: &str, string: &'t str) -> Option<regex::Captures<'t>> {
    compile(pattern).captures(string)
}

#[allow(dead_code)]
pub fn find_all<'t>(pattern: &str, string: &'t str) -> regex::CaptureMatches<'static, 't> {
    compile(pattern).captures_iter(string)
}
