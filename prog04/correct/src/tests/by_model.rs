use super::by_meth;

fn mk_tests<I>(i: I)
where
    I: std::slice::SliceIndex<
        [(&'static str, usize, Option<&'static str>)],
        Output = [(&'static str, usize, Option<&'static str>)],
    >,
{
    by_meth::mk_tests(crate::by_model, "by_model", i)
}

seq_macro::seq!(N in 00..30 {
#[test]
fn test~N() {
    mk_tests(50*N..50*(N+1))
}
});
