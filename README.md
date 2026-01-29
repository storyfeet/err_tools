Err Tools
=========

A library for simple but precise error handling.

By Matthew Stoodley

Purpose
------

'err tools' provides traits to make it easy to convert errors in anyhow::Err values,
in order to keep the error handling 'explicit', but out of the way.

It enables you to replace create errors detailed traceable errors.

Starting with the macro ```err_at``` you can make an error from a static ```&str``` or a 'format string' or any thing that implements ```[

```rust

#[macro_use]
use err_tools::*;
use err_tools::stackable::*;
fn do_thing()->Result<i32,AtError>{
    err_at!("I had an issue {}", 20)
}
let e_line = line!() - 2;

let x = do_thing();
let err = x.err().unwrap();

// Note this is readme is included in src/lib.rs for doc_generation
assert_eq!("src/lib.rs",err.loc.file);
assert_eq!(e_line,err.loc.line);


```




To produce an error compatible with anyhow::Result

Moreover it allows you to wrap the errors on top of each other, so that you can build a stack trace in terms of the 
errors as they build up




Dependencies
------------

It depends on the 'anyhow' crate. Which provides the base ```anyhow::Result<T>```.
where the actual error depends type is a 
type is defined a









