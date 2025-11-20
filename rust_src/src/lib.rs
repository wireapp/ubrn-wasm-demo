use std::sync::Arc;

uniffi::setup_scaffolding!("rust_src");

#[derive(uniffi::Object)]
pub struct MyObject {}

#[derive(uniffi::Record)]
pub struct MyRecord {
    member: Arc<MyObject>,
}

#[uniffi::export]
pub fn my_function(my_object: Arc<MyObject>) -> Arc<MyObject> {
    my_object
}
