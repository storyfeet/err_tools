Err Tools
=========

A library for simple but precise error handling.

By Matthew Stoodley

Purpose
------

'err tools' provides traits to make it easy to convert errors in anyhow::Err values,
in order to keep the error handling 'explicit', but out of the way.

It enables you to replace 

```rs

let mut x = fn_that_returns_option().ok_or(SomeFileError::new())?;

```
With 

```rs

let mut x = fn_that_returns_option().e_str("Could not load file")?;

```

To produce an error compatible with anyhow::Result

Moreover it allows you to wrap the errors on top of each other, so that you can build a stack trace in terms of the 
errors as they build up




Dependencies
------------

It depends on the 'anyhow' crate. Which provides the base ```anyhow::Result<T>```.
where the actual error depends type is a 
type is defined a









