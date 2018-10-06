// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(tool_lints)]


#[allow(unused_assignments)]
#[warn(clippy::misrefactored_assign_op, clippy::assign_op_pattern)]
fn main() {
    let mut a = 5;
    a += a + 1;
    a += 1 + a;
    a -= a - 1;
    a *= a * 99;
    a *= 42 * a;
    a /= a / 2;
    a %= a % 5;
    a &= a & 1;
    a *= a * a;
    a = a * a * a;
    a = a * 42 * a;
    a = a * 2 + a;
    a -= 1 - a;
    a /= 5 / a;
    a %= 42 % a;
    a <<= 6 << a;
}

// check that we don't lint on op assign impls, because that's just the way to impl them

use std::ops::{Mul, MulAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Wrap(i64);

impl Mul<i64> for Wrap {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Wrap(self.0 * rhs)
    }
}

impl MulAssign<i64> for Wrap {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs
    }
}
