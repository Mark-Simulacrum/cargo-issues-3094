#![feature(rustc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize)]
pub struct Foo {
    field: String
}
