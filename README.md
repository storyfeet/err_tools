Err Tools
=========

A library for simple but precise error handling.

By Matthew Stoodley

Purpose
------

'err tools' provides tools for maintaining an 'error stack' explicitly, and easily in error handling.

It provides Two Concrete Types:

* ErrorAt 
    * An Error with a file and line number
    * The error can be a simple ```&'static str```, or a 'String', or an anyhow::error.
* ErrorTrace
    * A list of 'ErrorAt's that itself implements Error

It also provides a set of macros that attach the *current line* and *current file* to the created error. These had to be macros, otherwise the line_number and file_name would be applied in the wrong place.

* ```err_at(err,?fmt_params...)!``` creates an ErrorAt with *Line* and *File* set correctly.    
    ie: ```option.ok_or(err_at!('No ingredients found'))?```
* ```err_at_res(err,?fmt_params...)``` creates an Err(ErrorAt) result with *Line* and *File* set correctly    
    ie: ```return err_at_res('A new error')```
* ```err_at_map!``` A closure to add location to the given error     
    ie: ```err.map_err(err_at_map!())```
* ```err_wrap!(err,?fmt_params...)``` Wrap the current error with another error, creating a stack trace
    ie : ```err.map_err(err_wrap!('Trying to bake a cake'))?```
    

These keep the error handling 'explicit', but out of the way.

It enables you to create errors detailed traceable errors.

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

However it is designed for building a trace too. 
To build a full call stack history of the error, more info can be easily added at each return.

```rust
#[macro_use]
use err_tools::{*,traceable::*};

fn inner()->Result<i32,AtError>{
    err_at_res!("inner error")
}

fn outer()->Result<i32,TraceError>{
    inner().map_err(err_wrap!("outer error"))
}

let res = outer();
let e = res.err().unwrap();
assert_eq!(2,e.trace().len());

```

AtError implements 'into' TraceError, and the err_wrap! macro takes advantage of this.

It also works from anyhow errors, but with other error types it is better to use the err_at!


Dependencies
------------

* 'anyhow' enables handling all kinds of Errors in the stack.
* 'thiserror' provides some derivations for the display method.



ChangeLog
---------

# v0.2.2

Adds a call to 'into' inside the err_at! macro, so that if you function needs a TraceError or Anyhow::Error, that is automatically covered.

fixes err_at_map macro so that it calls the err_at function not err_at_res,  
meaning it can be used effectively inside err_map.

# v0.2.1

Removes 'op_err' and 'res_err' from OpErr, and ResErr Traits respectively, and move the implementation from the trait to Option and Result, types.  op_err should not have been used directly, now it can't be.

creates a traceable error and macros to fit with it and include the file_name and line_number.






