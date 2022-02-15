use rand::{Rng, SeedableRng};

pub use super::super::TrieMap;

use super::super::tests::*;

// ************************************************************************* //
// DoubleEndedIterator utils
// ************************************************************************* //

pub struct Bias<I>
where
    I: DoubleEndedIterator,
{
    bias: f64,
    iter: I,
    rng: rand::rngs::StdRng,
}
impl<I> Iterator for Bias<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let r: f64 = self.rng.sample(rand::distributions::Standard);
        if r < self.bias {
            self.iter.next()
        } else {
            self.iter.next_back()
        }
    }
}
pub trait BiasIterator: DoubleEndedIterator {
    fn bias(self, bias: f64, seed: u64) -> Bias<Self>
    where
        Self: Sized,
    {
        Bias {
            bias,
            iter: self,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }
}
impl<I> BiasIterator for I where I: DoubleEndedIterator {}

const BIASES: [f64; 2] = [0.25, 0.5];
const SEEDS: std::ops::Range<u64> = 0..32;

// ************************************************************************* //
// into_iter
// ************************************************************************* //

mod de_into_iter {
    use super::BiasIterator;

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_count_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.into_iter().rev().count();
            let r2 = mtm.into_iter().rev().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().rev().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.into_iter().rev().nth(n);
                let r2 = mtm.into_iter().rev().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().rev().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_collect_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.into_iter().rev().collect::<Vec<_>>();
            let r2 = mtm.into_iter().rev().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().rev().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.into_iter().rev().skip(n).size_hint();
                let r2 = mtm.into_iter().rev().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().rev().skip({}).size_hint()", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_count_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.into_iter().bias(bias, seed).count();
                    let r2 = mtm.into_iter().bias(bias, seed).count();
                    assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().bias({}, {}).count()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_nth_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.into_iter().bias(bias, seed).nth(n);
                        let r2 = mtm.into_iter().bias(bias, seed).nth(n);
                        assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().bias({}, {}).nth({})", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_collect_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.into_iter().bias(bias, seed).collect::<Vec<_>>();
                    let r2 = mtm.into_iter().bias(bias, seed).collect::<Vec<_>>();
                    assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().bias({}, {}).collect::<Vec<_>>()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_size_hint_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.into_iter().bias(bias, seed).skip(n).size_hint();
                        let r2 = mtm.into_iter().bias(bias, seed).skip(n).size_hint();
                        assert_eq!(r1, r2, "mk_tm_{:02}().into_iter().bias({}, {}).skip({}).size_hint()", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });
}

// ************************************************************************* //
// iter_mut
// ************************************************************************* //

mod de_iter_mut {
    use super::BiasIterator;

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_count_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter_mut().rev().count();
            let r2 = mtm.iter_mut().rev().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().rev().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter_mut().rev().nth(n);
                let r2 = mtm.iter_mut().rev().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().rev().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_collect_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter_mut().rev().collect::<Vec<_>>();
            let r2 = mtm.iter_mut().rev().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().rev().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_for_each_test~N() {
            let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
            let mut r = 0;
            tm.iter_mut().rev().for_each(super::mutate_key_val(&mut r));
            let mut r = 0;
            mtm.iter_mut().rev().for_each(super::mutate_key_val(&mut r));
            assert_eq!(
                tm.check(),
                Ok(()),
                "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().rev().for_each(mutate_key_val(&mut r)); tm.check() }}",
                mk_num
            );
            assert_eq!(
                tm,
                super::TrieMap::from(&mtm),
                "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().rev().for_each(mutate_key_val(&mut r)); tm }}",
                mk_num
            );
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter_mut().rev().skip(n).size_hint();
                let r2 = mtm.iter_mut().rev().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().rev().skip({}).size_hint()", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_count_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.iter_mut().bias(bias, seed).count();
                    let r2 = mtm.iter_mut().bias(bias, seed).count();
                    assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().bias({}, {}).count()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_nth_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.iter_mut().bias(bias, seed).nth(n);
                        let r2 = mtm.iter_mut().bias(bias, seed).nth(n);
                        assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().bias({}, {}).nth({})", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_collect_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.iter_mut().bias(bias, seed).collect::<Vec<_>>();
                    let r2 = mtm.iter_mut().bias(bias, seed).collect::<Vec<_>>();
                    assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().bias({}, {}).collect::<Vec<_>>()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_size_hint_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.iter_mut().bias(bias, seed).skip(n).size_hint();
                        let r2 = mtm.iter_mut().bias(bias, seed).skip(n).size_hint();
                        assert_eq!(r1, r2, "mk_tm_{:02}().iter_mut().bias({}, {}).skip({}).size_hint()", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_for_each_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mut mtm, mut tm) = super::mk_mtm_and_tm_~N();
                    let mut r = 0;
                    tm.iter_mut().bias(bias, seed).for_each(super::mutate_key_val(&mut r));
                    let mut r = 0;
                    mtm.iter_mut().bias(bias, seed).for_each(super::mutate_key_val(&mut r));
                    assert_eq!(
                        tm.check(),
                        Ok(()),
                        "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().bias({}, {}).for_each(mutate_key_val(&mut r)); tm.check() }}",
                        mk_num,
                        bias,
                        seed,
                    );
                    assert_eq!(
                        tm,
                        super::TrieMap::from(&mtm),
                        "{{ let mut tm = mk_tm_{:02}(); let r = 0; tm.iter_mut().bias({}, {}).for_each(mutate_key_val(&mut r)); tm }}",
                        mk_num,
                        bias,
                        seed,
                    );
                }
            }
        }
    });
}

// ************************************************************************* //
// iter
// ************************************************************************* //

mod de_iter {
    use super::BiasIterator;

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_count_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter().rev().count();
            let r2 = mtm.iter().rev().count();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter().rev().count()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_nth_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter().rev().nth(n);
                let r2 = mtm.iter().rev().nth(n);
                assert_eq!(r1, r2, "mk_tm_{:02}().iter().rev().nth({})", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn rev_collect_test~N() {
            let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
            let r1 = tm.iter().rev().collect::<Vec<_>>();
            let r2 = mtm.iter().rev().collect::<Vec<_>>();
            assert_eq!(r1, r2, "mk_tm_{:02}().iter().rev().collect::<Vec<_>>()", mk_num)
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn rev_size_hint_test~N() {
            for n in 0..N+1 {
                let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                let r1 = tm.iter().rev().skip(n).size_hint();
                let r2 = mtm.iter().rev().skip(n).size_hint();
                assert_eq!(r1, r2, "mk_tm_{:02}().iter().rev().skip({}).size_hint()", mk_num, n)
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_count_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.iter().bias(bias, seed).count();
                    let r2 = mtm.iter().bias(bias, seed).count();
                    assert_eq!(r1, r2, "mk_tm_{:02}().iter().bias({}, {}).count()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_nth_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.iter().bias(bias, seed).nth(n);
                        let r2 = mtm.iter().bias(bias, seed).nth(n);
                        assert_eq!(r1, r2, "mk_tm_{:02}().iter().bias({}, {}).nth({})", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        fn bias_collect_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                    let r1 = tm.iter().bias(bias, seed).collect::<Vec<_>>();
                    let r2 = mtm.iter().bias(bias, seed).collect::<Vec<_>>();
                    assert_eq!(r1, r2, "mk_tm_{:02}().iter().bias({}, {}).collect::<Vec<_>>()", mk_num, bias, seed)
                }
            }
        }
    });

    seq_macro::seq!(N in 00..24 {
        #[test]
        #[allow(clippy::identity_op)]
        fn bias_size_hint_test~N() {
            for bias in super::BIASES {
                for seed in super::SEEDS {
                    for n in 0..N+1 {
                        let (mk_num, mtm, tm) = super::mk_mtm_and_tm_~N();
                        let r1 = tm.iter().bias(bias, seed).skip(n).size_hint();
                        let r2 = mtm.iter().bias(bias, seed).skip(n).size_hint();
                        assert_eq!(r1, r2, "mk_tm_{:02}().iter().bias({}, {}).skip({}).size_hint()", mk_num, bias, seed, n)
                    }
                }
            }
        }
    });
}
