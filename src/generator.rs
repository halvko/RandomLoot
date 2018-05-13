use std::collections::BTreeMap;
use std::iter::FromIterator;
use item::Item;
use item::ItemStack;
use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Generator<E> {
    tree: BTreeMap<u32, E>,
    total: u32,
}

pub struct Iter<'a, E: 'a> {
    generator: &'a Generator<E>
}

impl<E> FromIterator<(u32, E)> for Generator<E> {
    fn from_iter<I: IntoIterator<Item = (u32, E)>>(iter: I) -> Generator<E> {
        let mut tree = BTreeMap::new();
        let mut total = 0;

        for (rand, item) in iter {
            total += rand;
            tree.insert(total, item);
        }

        Generator { tree: tree, total: total }
    }
}

impl<'a, E> Iterator for Iter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<&'a E> {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, self.generator.total);
        self.generator.tree.range(i..).next().map(|e| e.1)
    }
}

impl<E> Generator<E> {
    pub fn iter<'a>(&'a self) -> Iter<'a, E> {
        Iter { generator: self }
    }
}