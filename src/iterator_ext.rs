use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;


pub trait IteratorExt : Iterator {
    fn unique(self) -> Unique<Self>;
}


pub struct Unique<I: ?Sized + Iterator> {
    seen: HashSet<I::Item>,
    iterator: I
}


impl<I: Iterator> IteratorExt for I where I::Item: Eq + Hash + Clone {
    fn unique(self) -> Unique<Self> {
        Unique { iterator: self, seen: HashSet::<I::Item>::new() }
    }
}


impl<I> Iterator for Unique<I> where I: Iterator, I::Item: Eq + Hash + Clone {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iterator.next() {
            if !self.seen.contains(&item) {
                self.seen.insert(item.clone());
                return Some(item)
            }
        }
        None
    }
}
