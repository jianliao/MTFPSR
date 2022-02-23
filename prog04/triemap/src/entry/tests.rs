use super::*;
use crate::tests::*;

// ************************************************************************* //
// Entry API utils
// ************************************************************************* //

#[derive(Debug, PartialEq, Eq)]
enum EntryKind {
    Vacant,
    Occupied,
}
trait EntryKindTrait {
    fn kind(&self) -> EntryKind;
}

trait EntryTrait
where
    Self: Sized,
{
    type VacantEntry;
    fn vacant(self) -> Option<Self::VacantEntry>;
    fn unwrap_vacant(self) -> Self::VacantEntry {
        self.vacant().unwrap()
    }
    type OccupiedEntry;
    fn occupied(self) -> Option<Self::OccupiedEntry>;
    fn unwrap_occupied(self) -> Self::OccupiedEntry {
        self.occupied().unwrap()
    }
}

impl<'a, V> EntryKindTrait for Entry<'a, V> {
    fn kind(&self) -> EntryKind {
        match self {
            Entry::Vacant(_) => EntryKind::Vacant,
            Entry::Occupied(_) => EntryKind::Occupied,
        }
    }
}
impl<'a, V> EntryTrait for Entry<'a, V> {
    type VacantEntry = VacantEntry<'a, V>;
    fn vacant(self) -> Option<VacantEntry<'a, V>> {
        match self {
            Entry::Vacant(ve) => Some(ve),
            Entry::Occupied(_) => None,
        }
    }
    type OccupiedEntry = OccupiedEntry<'a, V>;
    fn occupied(self) -> Option<OccupiedEntry<'a, V>> {
        match self {
            Entry::Vacant(_) => None,
            Entry::Occupied(oe) => Some(oe),
        }
    }
}

trait EntryTraitForPair
where
    Self: Sized,
{
    type VacantEntry;
    fn vacant(self, msg: &str) -> Option<Self::VacantEntry>;
    type OccupiedEntry;
    fn occupied(self, msg: &str) -> Option<Self::OccupiedEntry>;
}

impl<T1, T2> EntryTraitForPair for (T1, T2)
where
    T1: EntryKindTrait + EntryTrait,
    T2: EntryKindTrait + EntryTrait,
{
    type VacantEntry = (T1::VacantEntry, T2::VacantEntry);
    fn vacant(self, msg: &str) -> Option<Self::VacantEntry> {
        let (k1, k2) = (self.0.kind(), self.1.kind());
        match (self.0.vacant(), self.1.vacant()) {
            (Some(ve1), Some(ve2)) => Some((ve1, ve2)),
            (None, None) => None,
            _ => {
                assert_eq!(k1, k2, "{}", msg);
                None
            }
        }
    }
    type OccupiedEntry = (T1::OccupiedEntry, T2::OccupiedEntry);
    fn occupied(self, msg: &str) -> Option<Self::OccupiedEntry> {
        let (k1, k2) = (self.0.kind(), self.1.kind());
        match (self.0.occupied(), self.1.occupied()) {
            (Some(oe1), Some(oe2)) => Some((oe1, oe2)),
            (None, None) => None,
            _ => {
                assert_eq!(k1, k2, "{}", msg);
                None
            }
        }
    }
}

// ************************************************************************* //
// entry
// ************************************************************************* //

mod entry {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryKindTrait;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let mtm_res = mtm.entry(w).kind();
        let tm_res = tm.entry(w).kind();
        assert_eq!(mtm_res, tm_res, "mk_tm_{:02}().entry({:?})", mk_num, w);
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
// OccupiedEntry::key
// ************************************************************************* //

mod occupied_key {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.key();
            let tm_res = oe.key();
            assert_eq!(
                mtm_res, tm_res,
                "mk_tm_{:02}().entry({:?}).unwrap_occupied().key()",
                mk_num, w
            )
        }
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
// OccupiedEntry::remove_entry
// ************************************************************************* //

mod occupied_remove_entry {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.remove_entry();
            let tm_res = oe.remove_entry();
            assert_eq!(
                mtm_res, tm_res,
                "mk_tm_{:02}().entry({:?}).unwrap_occupied().remove_entry()",
                mk_num, w
            )
        }
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().remove_entry(); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().remove_entry(); tm }}",
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
// OccupiedEntry::get
// ************************************************************************* //

mod occupied_get {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.get();
            let tm_res = oe.get();
            assert_eq!(
                mtm_res, tm_res,
                "mk_tm_{:02}().entry({:?}).unwrap_occupied().get()",
                mk_num, w
            )
        }
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
// OccupiedEntry::get_mut
// ************************************************************************* //

mod occupied_get_mut {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((mut moe, mut oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.get_mut();
            let tm_res = oe.get_mut();
            assert_eq!(
                mtm_res, tm_res,
                "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().get_mut() }}",
                mk_num, w
            );
            let mut r = 0;
            (super::mutate_val(&mut r))(mtm_res);
            let mut r = 0;
            (super::mutate_val(&mut r))(tm_res);
        };
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().get_mut()); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().get_mut()); tm }}",
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
// OccupiedEntry::into_mut
// ************************************************************************* //

mod occupied_into_mut {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let res = if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.into_mut();
            let tm_res = oe.into_mut();
            assert_eq!(
                mtm_res, tm_res,
                "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().into_mut() }}",
                mk_num, w
            );
            Some((mtm_res, tm_res))
        } else {
            None
        };
        if let Some((mtm_res, tm_res)) = res {
            let mut r = 0;
            (super::mutate_val(&mut r))(mtm_res);
            let mut r = 0;
            (super::mutate_val(&mut r))(tm_res);
        };
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().into_mut()); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().into_mut()); tm }}",
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
// OccupiedEntry::insert
// ************************************************************************* //

mod occupied_insert {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((mut moe, mut oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.insert((42, true));
            let tm_res = oe.insert((42, true));
            assert_eq!(
                mtm_res, tm_res,
                "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().insert((42, true)) }}",
                mk_num, w
            )
        };
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().insert((42, true))); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_occupied().insert((42, true))) }}",
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
// OccupiedEntry::remove
// ************************************************************************* //

mod occupied_remove {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).occupied(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.remove();
            let tm_res = oe.remove();
            assert_eq!(
                mtm_res, tm_res,
                "mk_tm_{:02}().entry({:?}).unwrap_occupied().remove()",
                mk_num, w
            )
        }
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().remove(); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_occupied().entry(); tm }}",
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
// VacantEntry::key
// ************************************************************************* //

mod vacant_key {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        if let Some((mve, ve)) =
            (mtm.entry(w), tm.entry(w)).vacant(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = mve.key();
            let tm_res = ve.key();
            assert_eq!(
                mtm_res, tm_res,
                "mk_tm_{:02}().entry({:?}).unwrap_vacant().key()",
                mk_num, w
            )
        }
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
// VacantEntry::insert
// ************************************************************************* //

mod vacant_insert {
    use super::model::TrieMap as ModelTrieMap;
    use crate::TrieMap;

    use super::EntryTraitForPair;

    fn mk_test(
        mk_mtm_and_tm: impl FnOnce() -> (usize, ModelTrieMap<(usize, bool)>, TrieMap<(usize, bool)>),
        w: &str,
    ) {
        let (mk_num, mut mtm, mut tm) = mk_mtm_and_tm();
        let res = if let Some((moe, oe)) =
            (mtm.entry(w), tm.entry(w)).vacant(&format!("mk_tm_{:02}().entry({:?})", mk_num, w))
        {
            let mtm_res = moe.insert((42, true));
            let tm_res = oe.insert((42, true));
            assert_eq!(
                mtm_res, tm_res,
                "{{ let mut tm = mk_tm_{:02}(); tm.entry({:?}).unwrap_vacant().insert((42,true)) }}",
                mk_num, w
            );
            Some((mtm_res, tm_res))
        } else {
            None
        };
        if let Some((mtm_res, tm_res)) = res {
            let mut r = 0;
            (super::mutate_val(&mut r))(mtm_res);
            let mut r = 0;
            (super::mutate_val(&mut r))(tm_res);
        };
        assert_eq!(
            Ok(()),
            tm.check(),
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_vacant().insert((42, true))); tm.check() }}",
            mk_num,
            w
        );
        assert_eq!(
            TrieMap::from(&mtm),
            tm,
            "{{ let mut tm = mk_tm_{:02}(); let mut r = 0; mutate_val(&mut r)(tm.entry({:?}).unwrap_vacant().insert((42, true))); tm.check() }}",
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
// Model a TrieMap with a BTreeMap
// ************************************************************************* //

pub mod model {
    use std::collections::btree_map;

    pub use super::super::super::tests::model::*;
    use super::{EntryKind, EntryKindTrait, EntryTrait};

    type Entry<'a, V> = btree_map::Entry<'a, String, V>;
    type VacantEntry<'a, V> = btree_map::VacantEntry<'a, String, V>;
    type OccupiedEntry<'a, V> = btree_map::OccupiedEntry<'a, String, V>;

    impl<'a, V> EntryKindTrait for Entry<'a, V> {
        fn kind(&self) -> EntryKind {
            match self {
                Entry::Vacant(_) => EntryKind::Vacant,
                Entry::Occupied(_) => EntryKind::Occupied,
            }
        }
    }
    impl<'a, V> EntryTrait for Entry<'a, V> {
        type VacantEntry = VacantEntry<'a, V>;
        fn vacant(self) -> Option<VacantEntry<'a, V>> {
            match self {
                Entry::Vacant(ve) => Some(ve),
                Entry::Occupied(_) => None,
            }
        }
        type OccupiedEntry = OccupiedEntry<'a, V>;
        fn occupied(self) -> Option<OccupiedEntry<'a, V>> {
            match self {
                Entry::Vacant(_) => None,
                Entry::Occupied(oe) => Some(oe),
            }
        }
    }

    impl<V> TrieMap<V> {
        pub fn entry(&mut self, key: &str) -> Entry<'_, V> {
            self.0.entry(String::from(key))
        }
    }
}
