use std::mem::ManuallyDrop;

transparent_wrapper::transparent!(
    #[proxy_impl(
        Debug,
        Display,
        Default,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        From,
        Into,
        DerefInner,
        DerefMutInner,
        Index,
        IndexMut
    )]
    struct AllImpls<T: ?Sized> {
        inner: ManuallyDrop<T>,
    }
);

#[test]
fn all_proxy_impls() {

    let _ = AllImpls { inner: ManuallyDrop::new(10) };
}
