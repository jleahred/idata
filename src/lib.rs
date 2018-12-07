#![warn(missing_docs)]
//! Small tools to work with rust reducing
//! mutability and programming more functional
//! way.
//!
//! Mutability and sharing is bad.
//!
//! Viric mutability is bad.
//!
//! Mutability on large pieces of code is bad.
//!
//!
//! In Rust, we can have no viric, not shared
//! mutability. Let's reduce to small scopes...
//!
//!
//! For an introduction and context view, read...
//!
//! [README.md](https://github.com/jleahred/idata)
//!
//! A very basic example...
//!
//! ```rust
//!    extern crate idata;
//!    use idata::cont::IVec;
//!
//!    fn main() {
//!         let v = vec![1, 2];
//!         let v = v.ipush(3)
//!                  .ipush(4);
//!
//!        assert!(v == vec![1,2,3,4]);
//!    }
//!```
//!

/// Try getting a char from top of a Chars
/// returning the (char, remaining_chars) if possible
///
///  ```rust
///    extern crate idata;
///
///    fn main() {
///         let chars = "Hello world".chars();
///         let (ch, chars) = idata::consume_char(chars).unwrap();
///
///         assert!(ch == 'H');
///         assert!(chars.as_str() == "ello world");
///    }
///```
pub fn consume_char(mut chars: std::str::Chars) -> Option<(char, std::str::Chars)> {
    match chars.next() {
        Some(ch) => Some((ch, chars)),
        None => None,
    }
}

/// &mut propagates mutability to top
/// This is bad
///
/// You could have also a type working by owner shipt (to avoid
/// mutability propagation)
///
/// In several cases you will have problems due to references
///
/// To avoid it, steal_borrow will let you execute as owner
/// a function
///
///  ```rust
///     extern crate idata;
///
///     struct Ex {
///         val1: u16,
///         val2: u32,
///     }
///
///     impl Ex {
///         fn inc(mut self) -> Self {
///             self.val1 += 1;
///             self.val2 += 2;
///             self
///         }
///     }
///
///     fn test(rm_ex: &mut Ex) {
///         idata::steal_borrow(rm_ex, &|o : Ex| o.inc() );
///     }
///
///     fn main() {
///         let mut ex = Ex{ val1: 0, val2: 0};
///         test(&mut ex);
///
///         assert!(ex.val1 == 1);
///         assert!(ex.val2 == 2);
///     }
///```
pub fn steal_borrow<T>(target: &mut T, f: &Fn(T) -> T) {
    let mut fake = unsafe { std::mem::zeroed() };
    std::mem::swap(&mut fake, target);
    let mut fake = f(fake);
    std::mem::swap(&mut fake, target);
    std::mem::forget(fake);
}

/// Operations on inmutable vars over an string
pub trait IString {
    /// Add a char to a String
    ///
    ///  ```rust
    ///    extern crate idata;
    ///    use idata::IString;
    ///
    ///    fn main() {
    ///         let s = "Hello world".to_string();
    ///         let s = s.ipush('!');
    ///
    ///         assert!(s == "Hello world!");
    ///    }
    ///```
    fn ipush(self, ch: char) -> String;

    /// Remove a char from a String
    ///
    ///  ```rust
    ///    extern crate idata;
    ///    use idata::IString;
    ///
    ///    fn main() {
    ///         let s = "Hello world!".to_string();
    ///         let s = s.ipop().unwrap();
    ///
    ///         assert!(s == "Hello world");
    ///    }
    ///```
    fn ipop(self) -> Option<String>;
}

impl IString for String {
    fn ipush(mut self, ch: char) -> String {
        self.push(ch);
        self
    }
    fn ipop(mut self) -> Option<String> {
        self.pop()?;
        Some(self)
    }
}

pub mod cont {
    //! Module to work with containers

    /// Some operations to work with vectors
    ///
    pub trait IVec<T> {
        /// Push an element to a vector, and return the same vector
        ///
        ///  ```rust
        ///    extern crate idata;
        ///    use idata::cont::IVec;
        ///
        ///    fn main() {
        ///         let v = vec![1, 2];
        ///         let v = v.ipush(3)
        ///                  .ipush(4);
        ///
        ///        assert!(v == vec![1,2,3,4]);
        ///    }
        ///```
        fn ipush(self, T) -> Self;

        /// Append a vector to another
        ///
        ///  ```rust
        ///    extern crate idata;
        ///    use idata::cont::IVec;
        ///
        ///    fn main() {
        ///         let v1 = vec![1, 2];
        ///         let v2 = vec![3, 4, 5];
        ///         let v1 = v1.iappend(v2);
        ///
        ///         assert!(v1 == vec![1,2,3,4, 5]);
        ///    }
        ///```
        fn iappend(self, Vec<T>) -> Self;

        /// Remove an element from back of a vector
        ///
        ///  ```rust
        ///    extern crate idata;
        ///    use idata::cont::IVec;
        ///
        ///    fn main() {
        ///         let v1 = vec![1, 2, 3, 4, 5, 6];
        ///         let (o, v1) = v1.ipop();
        ///
        ///         assert!(v1 == vec![1,2,3,4, 5]);
        ///         assert!(o.unwrap() == 6);
        ///    }
        ///```
        fn ipop(self) -> (Option<T>, Self);
    }

    impl<T> IVec<T> for Vec<T> {
        fn ipush(mut self, v: T) -> Self {
            self.push(v);
            self
        }

        fn iappend(mut self, mut v: Vec<T>) -> Self {
            self.append(&mut v);
            self
        }

        fn ipop(mut self) -> (Option<T>, Self) {
            (self.pop(), self)
        }
    }
}

//-----------------------------------------------------------------------
//  TailCall
//-----------------------------------------------------------------------

pub mod tc {
    //! Recursive simulation with TCO
    //!
    //! We cannot use SSA in rust combined with a for loop
    //!
    //! It fits fine with recursion, but...alloc
    //!
    //! Rust doesn't have TCO (tail call optimization) in recursion.
    //!
    //! In some cases it could be expensive and even dangerous
    //!
    //! One option, could be to use next "trampolin"
    //!
    //!  ```rust
    //!    extern crate idata;
    //!    use idata::tc::*;
    //!
    //!    fn main() {
    //!         
    //!         let (sum, _) = tail_call((0, 0), |(acc, counter)| {
    //!             if counter < 101 {
    //!                 TailCall::Call((acc + counter, counter + 1))
    //!             } else {
    //!                 TailCall::Return((acc, counter))
    //!             }
    //!         });
    //!         assert!(sum == 5050);
    //!    }
    //!```

    /// Support to call or return from a recursive function
    pub enum TailCall<T, R> {
        /// Support to simulate a recursive call
        Call(T),
        /// Simultate a recursive return
        Return(R),
    }

    /// Function to simulate TCO. See example
    ///
    ///  ```rust
    ///    extern crate idata;
    ///    use idata::tc::*;
    ///
    ///    fn main() {
    ///         
    ///         let (sum, _) = tail_call((0, 0), |(acc, counter)| {
    ///             if counter < 101 {
    ///                 TailCall::Call((acc + counter, counter + 1))
    ///             } else {
    ///                 TailCall::Return((acc, counter))
    ///             }
    ///         });
    ///
    ///         assert!(sum == 5050);
    ///    }
    ///```
    pub fn tail_call<T, R, F>(seed: T, recursive_function: F) -> R
    where
        F: Fn(T) -> TailCall<T, R>,
    {
        let mut state = TailCall::Call(seed);
        loop {
            match state {
                TailCall::Call(arg) => {
                    state = recursive_function(arg);
                }
                TailCall::Return(result) => {
                    return result;
                }
            }
        }
    }
}
