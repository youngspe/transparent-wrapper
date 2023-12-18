use std::marker::PhantomData;

#[test]
fn with_phantom_data() {
    transparent_wrapper::transparent!(
        struct _MyStruct {
            _phantom: PhantomData<i32>,
            _inner: i32,
        }
    );
}
