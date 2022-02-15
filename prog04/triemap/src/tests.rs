pub use super::TrieMap;

// ************************************************************************* //
// check
// ************************************************************************* //

impl<V> TrieMap<V> {
    /// Check the invariants of a `TrieMap`.
    pub fn check(&self) -> Result<(), &'static str> {
        fn aux<V>(tm: &TrieMap<V>) -> Result<usize, &'static str> {
            let mut last_char = None;
            let mut len = 0;
            for (c, tm) in &tm.children {
                match last_char {
                    None => (),
                    Some(lc) => {
                        if lc >= c {
                            return Err("children not sorted");
                        }
                    }
                };
                last_char = Some(c);
                let l = aux(tm)?;
                if l == 0 {
                    // child trie is empty
                    return Err("child trie is empty");
                } else {
                    len += l
                };
            }
            if tm.val.is_some() {
                len += 1
            };
            if tm.len != len {
                // cached len is incorrect
                return Err("cached `len` is incorrect");
            }
            Ok(len)
        }
        aux(self).map(|_| ())
    }
}

// ************************************************************************* //
//
// ************************************************************************* //

pub const NUM_SEED_WORDS: usize = 24;

pub const SEED_WORDS: [&str; NUM_SEED_WORDS] = [
    "",
    "amp",
    "amperage",
    "ampere",
    "ampersand",
    "amphetamine",
    "amphibian",
    "crab",
    "crabby",
    "crust",
    "crustacean",
    "lazy",
    "leaf",
    "pizza",
    "pizzazz",
    "rust",
    "rusted",
    "rustic",
    "rusting",
    "rustle",
    "rustlers",
    "super",
    "whale",
    "zoo",
];

pub const NUM_ABS_WORDS: usize = 20;
pub const ABS_WORDS: [&str; NUM_ABS_WORDS] = [
    "ampheta",
    "amphibians",
    "amps",
    "boat",
    "bumblebee",
    "crunchy",
    "crusty",
    "dinosaur",
    "dragon",
    "kingdom",
    "lea",
    "leafy",
    "lean",
    "mathematics",
    "rusty",
    "sesquipedalianism",
    "superduper",
    "whales",
    "what",
    "zookeeper",
];

fn shuffled_seed_words(seed: u64) -> Vec<&'static str> {
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut ws: Vec<&'static str> = SEED_WORDS.into_iter().collect();
    ws.shuffle(&mut rng);
    ws
}

fn shuffled_all_words(seed: u64) -> Vec<&'static str> {
    use rand::seq::SliceRandom;
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut ws: Vec<_> = SEED_WORDS
        .into_iter()
        .chain(ABS_WORDS.into_iter())
        .collect();
    ws.shuffle(&mut rng);
    ws
}

pub fn mtm_from_words<V, F>(seed: u64, f: F) -> model::TrieMap<V>
where
    F: FnMut((usize, &str)) -> Option<(String, V)>,
{
    shuffled_seed_words(seed)
        .into_iter()
        .enumerate()
        .filter_map(f)
        .collect()
}

seq_macro::seq!(N in 00..24 {
#[allow(unused_comparisons)]
#[allow(clippy::absurd_extreme_comparisons,clippy::identity_op)]
pub fn mk_mtm_~N() -> (usize,model::TrieMap<(usize, bool)>) {
    (N,
     mtm_from_words(23, |(i, s)| {
         if N <= i {
             None
         } else {
             Some((String::from(s), (s.len(), s.len() % 2 == 0)))
         }
     }))
}
#[allow(dead_code)]
pub fn mk_tm_~N() -> TrieMap<(usize, bool)> {
    TrieMap::from(mk_mtm_~N().1)
}
pub fn mk_mtm_and_tm_~N() -> (usize, model::TrieMap<(usize, bool)>, TrieMap<(usize, bool)>) {
    let (n, mtm) = mk_mtm_~N();
    let tm = TrieMap::from(&mtm);
    (n, mtm, tm)
}
});

pub fn mutate_val(r: &mut usize) -> impl FnMut(&mut (usize, bool)) + '_ {
    |v| {
        *r += 1;
        v.0 += *r;
        v.1 = !v.1;
    }
}

pub fn mutate_key_val(r: &mut usize) -> impl FnMut((String, &mut (usize, bool))) + '_ {
    let mut f = mutate_val(r);
    move |k_v| f(k_v.1)
}

// ************************************************************************* //
// new
// ************************************************************************* //

mod new {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    #[test]
    fn test() {
        let mtm = ModelTrieMap::<(usize, bool)>::new();
        let tm = TrieMap::<(usize, bool)>::new();
        assert_eq!(
            tm.check(),
            Ok(()),
            "({}).check()",
            "TrieMap::<(usize,bool)>::new()"
        );
        assert_eq!(tm, TrieMap::from(mtm))
    }
}

// ************************************************************************* //
// next
// ************************************************************************* //

mod next {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    const NUM_NEXT_CHARS: usize = 17;
    const NEXT_CHARS: [char; NUM_NEXT_CHARS] = [
        'a', 'b', 'c', 'd', 'k', 'l', 'm', 'o', 'p', 'q', 'r', 's', 'v', 'w', 'x', 'y', 'z',
    ];

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        c: char,
    ) {
        let (mk_num, mtm, tm) = mk_mtm_and_tm();
        let mtm_next_res = mtm.next(c);
        let tm_next_res = tm.next(c);
        assert_eq!(
            tm_next_res.map(TrieMap::check),
            (&tm_next_res).map(|_| Ok(())),
            "mk_tm_{:02}().next({}).map(TrieMap::check)",
            mk_num,
            c
        );
        assert_eq!(
            tm_next_res,
            (&(mtm_next_res.map(TrieMap::from))).as_ref(),
            "mk_tm_{:02}().next({})",
            mk_num,
            c
        );
    }

    seq_macro::seq!(N in 00..24 {
    #[test]
    fn test~N() {
        for c in NEXT_CHARS.into_iter() {
            mk_test(super::mk_mtm_and_tm_~N, c)
        }
    }
    });
}

// ************************************************************************* //
// get
// ************************************************************************* //

mod get {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mtm, tm) = mk_mtm_and_tm();
        let mtm_get_res = mtm.get(w);
        let tm_get_res = tm.get(w);
        assert_eq!(
            tm_get_res, mtm_get_res,
            "mk_tm_{:02}().get({:?})",
            mk_num, w
        );
    }

    seq_macro::seq!(N in 00..24 {
    #[test]
    fn test~N() {
        for w in super::SEED_WORDS.iter().chain(super::ABS_WORDS.iter()) {
            mk_test(super::mk_mtm_and_tm_~N, w)
        }
    }
    });
}

// ************************************************************************* //
// get_mut
// ************************************************************************* //

mod get_mut {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let mtm_get_mut_res = mtm.get_mut(w);
        let tm_get_mut_res = tm.get_mut(w);
        assert_eq!(
            tm_get_mut_res, mtm_get_mut_res,
            "{{ let mut tm = mk_tm_{:02}(); tm.get_mut({:?}) }}",
            mk_num, w
        );
        let mut r = 0;
        mtm_get_mut_res.map(super::mutate_val(&mut r));
        let mut r = 0;
        tm_get_mut_res.map(super::mutate_val(&mut r));
        assert_eq!(
            tm.check(),
            Ok(()),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; tm.get_mut({:?}).map(mutate_val(&mut r)); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            tm,
            TrieMap::from(&mtm),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; tm.get_mut({:?}).map(mutate_val(&mut r)); tm }}",
            mk_num,
            w
        );
    }

    seq_macro::seq!(N in 00..24 {
    #[test]
    fn test~N() {
        for w in super::SEED_WORDS.iter().chain(super::ABS_WORDS.iter()) {
            mk_test(super::mk_mtm_and_tm_~N, w)
        }
    }
    });
}

// ************************************************************************* //
// insert
// ************************************************************************* //

mod insert {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    fn mk_one_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let mtm_insert_res = mtm.insert(w, (42, true));
        let tm_insert_res = tm.insert(w, (42, true));
        assert_eq!(
            tm_insert_res, mtm_insert_res,
            "{{ let mut tm = mk_tm_{:02}(); tm.insert({:?}, (42, true)) }}",
            mk_num, w
        );
        assert_eq!(
            tm.check(),
            Ok(()),
            "{{ let mut tm = mk_tm_{:02}(); tm.insert({:?}, (42, true)); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            tm,
            TrieMap::from(&mtm),
            "{{ let mut tm = mk_tm_{:02}(); tm.insert({:?}, (42, true)); tm }}",
            mk_num,
            w
        );
    }

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn one_test~N() {
            for w in super::SEED_WORDS.iter().chain(super::ABS_WORDS.iter()) {
                mk_one_test(super::mk_mtm_and_tm_~N, w)
            }
        }
    });

    fn mk_many_test(n: usize) {
        let mtm = super::mtm_from_words(541, |(i, s)| {
            if i < n {
                Some((String::from(s), s.len() % 3 == 0))
            } else {
                None
            }
        });

        let mut tm = TrieMap::new();
        for w in super::shuffled_seed_words(541).into_iter().take(n) {
            tm.insert(w, w.len() % 3 == 0);
        }
        assert_eq!(tm.check(), Ok(()),
                   "{{ let mut tm: TrieMap<bool> = TrieMap::new(); for w in shuffled_seed_words(541).into_iter().take({}) {{ tm.insert(w, w.len() % 3 == 0); }}; tm.check() }}",
                   n);
        assert_eq!(tm, TrieMap::from(mtm),
                   "{{ let mut tm: TrieMap< bool> = TrieMap::new(); for w in shuffled_seed_words(541).into_iter().take({}) {{ tm.insert(w, w.len() % 3 == 0); }}; tm }}",
                   n);
    }

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn many_test~N() {
            mk_many_test(N)
        }
    });
}

// ************************************************************************* //
// remove
// ************************************************************************* //

mod remove {
    use super::model::TrieMap as ModelTrieMap;
    use super::TrieMap;

    fn mk_one_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let mtm_remove_res = mtm.remove(w);
        let tm_remove_res = tm.remove(w);
        assert_eq!(
            tm_remove_res, mtm_remove_res,
            "{{ let mut tm = mk_tm_{:02}(); tm.remove({:?}) }}",
            mk_num, w
        );
        assert_eq!(
            tm.check(),
            Ok(()),
            "{{ let mut tm = mk_tm_{:02}(); tm.remove({:?}); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            tm,
            TrieMap::from(&mtm),
            "{{ let mut tm = mk_tm_{:02}(); tm.remove({:?}); tm }}",
            mk_num,
            w
        );
    }

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn one_test~N() {
            for w in super::SEED_WORDS.iter().chain(super::ABS_WORDS.iter()) {
                mk_one_test(super::mk_mtm_and_tm_~N, w)
            }
        }
    });

    fn mk_many_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
    ) {
        let (mk_num, _, mut tm) = mk_mtm_and_tm();
        for w in super::shuffled_all_words(641) {
            tm.remove(w);
        }
        assert_eq!(tm.check(), Ok(()),
                   "{{ let mut tm = mk_tm_{:02}(); for w in shuffled_all_words(641) {{ tm.iremove(w); }}; tm.check() }}",
                   mk_num);
        assert_eq!(tm, TrieMap::new(),
                   "{{ let mut tm = mk_tm_{:02}(); for w in shuffled_all_words(641) {{ tm.iremove(w); }}; tm }}",
                   mk_num);
    }

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn many_test~N() {
            mk_many_test(super::mk_mtm_and_tm_~N)
        }
    });
}

// ************************************************************************* //
// into_iter
// ************************************************************************* //

mod into_iter {
    seq_macro::seq!(N in 00..24 {
        #[test]
        fn count_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.into_iter().count();
            let r2 = mtm.into_iter().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.into_iter().nth(n);
                let r2 = mtm.into_iter().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn collect_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.into_iter().collect::<Vec<_>>();
            let r2 = mtm.into_iter().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.into_iter().skip(n).size_hint();
                let r2 = mtm.into_iter().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().skip({}).size_hint()", mk_num, n)
            }
        }
    });
}

// ************************************************************************* //
// iter_mut
// ************************************************************************* //

mod iter_mut {
    seq_macro::seq!(N in 00..24 {
        #[test]
        fn count_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter_mut().count();
            let r2 = mtm.iter_mut().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter_mut().nth(n);
                let r2 = mtm.iter_mut().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn collect_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter_mut().collect::<Vec<_>>();
            let r2 = mtm.iter_mut().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter_mut().skip(n).size_hint();
                let r2 = mtm.iter_mut().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().skip({}).size_hint()", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn for_each_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let mut r = 0;
            tm.iter_mut().for_each(super::mutate_key_val(&mut r));
            let mut r = 0;
            mtm.iter_mut().for_each(super::mutate_key_val(&mut r));
            assert_eq!(
                tm.check(),
                Ok(()),
                "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().for_each(mutate_key_val(&mut r)); tm.check() }}",
                mk_num
            );
            assert_eq!(
                tm,
                super::TrieMap::from(&mtm),
                "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().for_each(mutate_key_val(&mut r)); tm }}",
                mk_num
            );
        }
    });
}

// ************************************************************************* //
// iter
// ************************************************************************* //

mod iter {
    seq_macro::seq!(N in 00..24 {
        #[test]
        fn count_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter().count();
            let r2 = mtm.iter().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter().nth(n);
                let r2 = mtm.iter().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().iter().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn collect_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter().collect::<Vec<_>>();
            let r2 = mtm.iter().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter().skip(n).size_hint();
                let r2 = mtm.iter().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().iter().skip({}).size_hint()", mk_num, n)
            }
        }
    });
}

// ************************************************************************* //
// Model a TrieMap with a BTreeMap
// ************************************************************************* //

pub mod model {
    use std::collections::{btree_map, BTreeMap};
    use std::iter::FromIterator;
    use std::ops::Index;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TrieMap<V>(BTreeMap<String, V>);

    impl<V> TrieMap<V> {
        pub fn new() -> Self {
            TrieMap(BTreeMap::new())
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn next(&self, c: char) -> Option<Self>
        where
            V: Clone,
        {
            let btm = &self.0;
            let res: BTreeMap<String, V> = btm
                .iter()
                .filter_map(|(k, v)| k.strip_prefix(c).map(|s| (String::from(s), v.clone())))
                .collect();
            if res.is_empty() {
                None
            } else {
                Some(TrieMap(res))
            }
        }

        pub fn get(&self, key: &str) -> Option<&V> {
            self.0.get(key)
        }

        pub fn contains_key(&self, key: &str) -> bool {
            self.0.contains_key(key)
        }

        pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
            self.0.get_mut(key)
        }

        pub fn insert(&mut self, key: &str, v: V) -> Option<V> {
            self.0.insert(String::from(key), v)
        }

        pub fn remove(&mut self, key: &str) -> Option<V> {
            self.0.remove(key)
        }

        pub fn iter(
            &self,
        ) -> std::iter::Map<
            btree_map::Iter<'_, String, V>,
            for<'a> fn((&'a String, &'a V)) -> (String, &'a V), /*{iter_fn::<_>}*/
        > {
            self.0.iter().map(iter_fn)
        }

        pub fn iter_mut(
            &mut self,
        ) -> std::iter::Map<
            btree_map::IterMut<'_, String, V>,
            for<'a> fn((&'a String, &'a mut V)) -> (String, &'a mut V), /*{iter_mut_fn::<_>}*/
        > {
            self.0.iter_mut().map(iter_mut_fn)
        }
    }

    impl<V> Default for TrieMap<V> {
        fn default() -> TrieMap<V> {
            TrieMap::new()
        }
    }

    impl<V> Index<&str> for TrieMap<V> {
        type Output = V;
        fn index(&self, key: &str) -> &V {
            self.get(key).expect("no entry found for key")
        }
    }

    impl<V> IntoIterator for TrieMap<V> {
        type Item = (String, V);
        type IntoIter = btree_map::IntoIter<String, V>;
        fn into_iter(self: TrieMap<V>) -> Self::IntoIter {
            self.0.into_iter()
        }
    }

    impl<'a, V> FromIterator<(&'a str, V)> for TrieMap<V> {
        fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = (&'a str, V)>,
        {
            let mut tm = TrieMap::new();
            tm.extend(iter);
            tm
        }
    }

    impl<'a, V> Extend<(&'a str, V)> for TrieMap<V> {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = (&'a str, V)>,
        {
            for (k, v) in iter {
                self.insert(k, v);
            }
        }
    }

    impl<'a, V> FromIterator<(String, V)> for TrieMap<V> {
        fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item = (String, V)>,
        {
            let mut tm = TrieMap::new();
            tm.extend(iter);
            tm
        }
    }

    impl<'a, V> Extend<(String, V)> for TrieMap<V> {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = (String, V)>,
        {
            for (k, v) in iter {
                self.insert(&k, v);
            }
        }
    }

    impl<'a, V: 'a> Extend<(&'a str, &'a V)> for TrieMap<V>
    where
        V: Copy,
    {
        fn extend<T>(&mut self, iter: T)
        where
            T: IntoIterator<Item = (&'a str, &'a V)>,
        {
            for (k, v) in iter {
                self.insert(k, *v);
            }
        }
    }

    impl<'a, V: Clone> From<&'a TrieMap<V>> for super::TrieMap<V> {
        fn from(mbtm: &'a TrieMap<V>) -> Self {
            let len = mbtm.len();
            let val = mbtm.get("").cloned();
            let mut children_chars = (&mbtm.0)
                .keys()
                .filter_map(|k| k.chars().next())
                .collect::<Vec<_>>();
            children_chars.dedup();
            let children = children_chars
                .into_iter()
                .filter_map(|c| mbtm.next(c).map(|mbtm| (c, super::TrieMap::from(mbtm))))
                .collect();
            super::TrieMap { len, val, children }
        }
    }

    impl<V: Clone> From<TrieMap<V>> for super::TrieMap<V> {
        fn from(mbtm: TrieMap<V>) -> Self {
            super::TrieMap::from(&mbtm)
        }
    }

    fn iter_fn<'a, V>((w, v): (&'a String, &'a V)) -> (String, &'a V) {
        (String::from(w), v)
    }

    fn iter_mut_fn<'a, V>((w, v): (&'a String, &'a mut V)) -> (String, &'a mut V) {
        (String::from(w), v)
    }
}
