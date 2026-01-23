Err Tools
=========

A library for simple but precise error handling.

By Matthew Stoodley

Purpose
------

'err tools' provides traits to make it easy to convert errors inline.

It enables you to replace

```rs

let mut x = match fn_that_returns_option() {
    Option(val)=>val,
    None => return Err(SomeErrorClass::new()),
};

```

With 

```rs

let mut x = fn_that_returns_option().op_err(SomeErrorClass::new())?;

```


Dependencies
------------

It depends on the 'anyhow' crate. Which provides the base ```anyhow::Result<T>```.
where the actual error depends type is a 
type is defined a









