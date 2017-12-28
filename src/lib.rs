/*
 * Copyright © 2017 NixCheat
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

use std::borrow::Cow;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd)]
struct Version<'a> {
  raw: Cow<'a, str>
}

impl<'a> Version<'a> {
  pub fn new<S>(raw: S) -> Version<'a>
    where S: Into<Cow<'a, str>>
    {
      Version { raw: raw.into() }
    }
}

impl<'a> Ord for Version<'a> {
  fn cmp(&self, other: &Version<'a>) -> Ordering {
    Ordering::Equal
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_equal () {
    assert_eq!(Ordering::Equal, Version::new("1.10").cmp(&Version::new("1.10")))
  }

  #[test]
  fn is_less_than () {
    assert_eq!(Ordering::Less, Version::new("1.9").cmp(&Version::new("1.10")))
  }

  #[test]
  fn is_greater_than () {
    assert_eq!(Ordering::Greater, Version::new("1.10").cmp(&Version::new("1.9")))
  }
}
