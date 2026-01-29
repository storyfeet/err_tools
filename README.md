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
use err_tools::{*,traceable::*};
fn do_thing()->Result<i32,AtError>{
    err_at_res!("I had {} issues", 20)
}
// The line and file where the macro is called.
let e_line = line!() - 3;
let e_file = file!();

let x = do_thing();
let err = x.err().unwrap();

assert_eq!(e_file,err.loc.file);
assert_eq!(e_line,err.loc.line);
assert_eq!(
    format!("{}:{} - I had 20 issues",e_file,e_line),
    err.to_string()
);

```




To produce an error compatible with anyhow::Result

Moreover it allows you to wrap the errors on top of each other, so that you can build a stack trace in terms of the 
errors as they build up




Dependencies
------------

It depends on the 'anyhow' crate. Which provides the base ```anyhow::Result<T>```.
where the actual error depends type is a 
type is defined a









