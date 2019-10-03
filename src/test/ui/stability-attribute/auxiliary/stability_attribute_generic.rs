#![feature(staged_api)]
#![feature(dropck_eyepatch)]
#![stable(feature = "stable_generic_feature", since = "1.40.0")]


#[stable(feature = "stable_generic_feature", since = "1.40.0")]
pub struct FooBarBazz<#[stable(feature = "stable_generic_feature", since = "1.40.0")] T = usize> {
    _f: T,
}
