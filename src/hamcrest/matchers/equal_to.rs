use std::fmt::{self, Show};
use {success,Matcher,MatchResult};

pub struct EqualTo<T> {
  expected: T
}

impl<T: Show> fmt::Show for EqualTo<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.expected.fmt(f)
  }
}

impl<T : PartialEq + Show> Matcher<T> for EqualTo<T> {
  fn matches(&self, actual: T) -> MatchResult {
    if self.expected.eq(&actual) {
      success()
    }
    else {
      Err(format!("was {:?}", actual))
    }
  }
}

pub fn equal_to<T : PartialEq + Show>(expected: T) -> EqualTo<T> {
  EqualTo { expected: expected }
}

#[cfg(test)]
mod test {
    use std::thread::Thread;
    use {assert_that,is,equal_to};

    #[test]
    fn test_equality_of_ints() {
        // Successful match
        assert_that(&1, is(equal_to(&1i)));

        // Unsuccessful match
        let res = Thread::scoped(|| {
          assert_that(&2, is(equal_to(&1i)));
        }).join();

        assert!(res.is_err());
    }
}
