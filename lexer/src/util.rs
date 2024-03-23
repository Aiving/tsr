use std::str;
use std::str::FromStr;
use std::str::Utf8Error;

use std::collections::HashMap;

use crate::globals::BytesSpan;

pub fn concat_slice_vec(c: &[u8], done: Vec<u8>) -> Vec<u8> {
    let mut new_vec = c.to_vec();

    new_vec.extend(&done);

    new_vec
}

pub fn convert_vec_utf8(v: Vec<u8>) -> Result<String, Utf8Error> {
    let slice = v.as_slice();

    str::from_utf8(slice).map(|s| s.to_owned())
}

pub fn complete_byte_slice_str_from_utf8(c: BytesSpan) -> Result<&str, Utf8Error> {
    str::from_utf8(c.fragment())
}

pub fn complete_str_from_str<F: FromStr>(c: &str) -> Result<F, F::Err> {
    FromStr::from_str(c)
}

pub fn get_by_index<T>(elements: Vec<T>, index: usize, default_value: T) -> T {
    elements.into_iter().nth(index).unwrap_or(default_value)
}

pub trait VecExt<T> {
    fn includes<P>(&self, predicate: P) -> bool
    where
        P: Fn(&T) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn includes<P>(&self, predicate: P) -> bool
    where
        P: Fn(&T) -> bool,
    {
        for element in self {
            if predicate(element) {
                return true;
            }
        }

        false
    }
}

pub trait MapExt<K, V> {
    fn find<P>(&self, predicate: P) -> Option<(&K, &V)>
    where
        P: Fn((&K, &V)) -> bool;

    fn find_mut<P>(&mut self, predicate: P) -> Option<(&K, &mut V)>
    where
        P: FnMut((&K, &V)) -> bool;
}

pub trait BoolExt {
    fn map<T>(&self, trueness: T, falseness: T) -> T;
}

impl BoolExt for bool {
    fn map<T>(&self, trueness: T, falseness: T) -> T {
        if self.eq(&true) {
            trueness
        } else {
            falseness
        }
    }
}

impl<K: Clone, V: Clone> MapExt<K, V> for HashMap<K, V> {
    fn find<P>(&self, predicate: P) -> Option<(&K, &V)>
    where
        P: Fn((&K, &V)) -> bool,
    {
        let mut result = None;

        for element in self {
            if predicate(element) {
                result = Some(element);

                break;
            }
        }

        result
    }

    fn find_mut<P>(&mut self, mut predicate: P) -> Option<(&K, &mut V)>
    where
        P: FnMut((&K, &V)) -> bool,
    {
        let mut result = None;

        for element in self.iter_mut() {
            if predicate((element.0, element.1)) {
                result = Some(element);

                break;
            }
        }

        result
    }
}
