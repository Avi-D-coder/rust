// aux-build:stability_attribute_generic.rs

extern crate stability_attribute_generic;
use stability_attribute_generic::*;

fn new_foo(f: FooBarBazz) {}

fn new_foo_t<T>(f: FooBarBazz<T>) {}

fn new_foo_str(f: FooBarBazz<&str>) {}

fn main() {}
