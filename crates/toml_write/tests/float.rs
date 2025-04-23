#![cfg(feature = "alloc")]
#![allow(clippy::dbg_macro)] // unsure why config isn't working

use snapbox::prelude::*;
use snapbox::str;

use toml_write::ToTomlValue;

#[track_caller]
fn t(decoded: impl ToTomlValue, expected: impl IntoData) {
    let value = decoded.to_toml_value();
    snapbox::assert_data_eq!(value, expected.raw());
}

#[test]
fn zero() {
    t(0.0f64, str!["0.0"]);
}

#[test]
fn neg_zero() {
    t(-0.0f64, str!["-0.0"]);
}

#[test]
fn inf() {
    t(f64::INFINITY, str!["inf"]);
}

#[test]
fn neg_inf() {
    t(f64::NEG_INFINITY, str!["-inf"]);
}

#[test]
fn nan() {
    t(f64::NAN.copysign(1.0), str!["nan"]);
}

#[test]
fn neg_nan() {
    t(f64::NAN.copysign(-1.0), str!["-nan"]);
}
