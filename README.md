# idata

Small tools to work with rust reducing mutability and programming more functional way.

* Mutability and sharing is bad.
* Viric mutability is bad.
* Mutability on large pieces of code is bad.

In Rust, we can have no viric, not shared
mutability. Lets use this feature.

This small tools aims to help with this points

[repository](https://github.com/jleahred/idata)

## Some examples

A very basic example...

```rust
   extern crate idata;
   use idata::IVec;
   fn main() {
        let v = vec![1, 2];
        let v = v.ipush(3)
                 .ipush(4);
       assert!(v == vec![1,2,3,4]);
   }
```

Push an element to a vector, and return the same vector

```rust
    extern crate idata;
    use idata::IVec;

    fn main() {
         let v = vec![1, 2];
         let v = v.ipush(3)
                  .ipush(4);

        assert!(v == vec![1,2,3,4]);
    }
```

Append a vector to another

```rust
    extern crate idata;
    use idata::IVec;

    fn main() {
         let v1 = vec![1, 2];
         let v2 = vec![3, 4, 5];
         let v1 = v1.iappend(v2);

         assert!(v1 == vec![1,2,3,4, 5]);
    }
```

Remove an element from back of a vector

```rust
    extern crate idata;
    use idata::IVec;

    fn main() {
         let v1 = vec![1, 2, 3, 4, 5, 6];
         let (o, v1) = v1.ipop();

         assert!(v1 == vec![1,2,3,4, 5]);
         assert!(o.unwrap() == 6);
    }
```

Try getting a char from top of a Chars
returning the (char, remaining_chars) if possible

```rust
    extern crate idata;

    fn main() {
         let chars = "Hello world".chars();
         let (ch, chars) = idata::consume_char(chars).unwrap();

         assert!(ch == 'H');
         assert!(chars.as_str() == "ello world");
    }
```

Recursive simulation with TCO

We cannot use `SSA` in `rust` combined with a for loop

It fits fine with recursion, but...alloc

Rust doesn't have `TCO` (tail call optimization) in recursion.

In some cases it could be expensive and even dangerous

One option, could be to use next "trampolin"

```rust
    extern crate idata;
    use idata::tc::*;

    fn main() {
            let (sum, _) = tail_call((0, 0), |(acc, counter)| {
                if counter < 101 {
                    TailCall::Call((acc + counter, counter + 1))
                } else {
                    TailCall::Return((acc, counter))
                }
            });
            assert!(sum == 5050);
    }
```
