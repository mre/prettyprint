# prettyprint - syntax highlighting with batteries included

...a surprisingly creative name for a humble syntax highlighting package.

## Quick start



The above output was created with the following code:

```rust
let printer = PrettyPrinter::default()
    .language("rust")
    .build()?;

printer.file("fixtures/fib.rs")?;
```

Note that `prettyprint` is a [builder](https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md) and can be customized. For example, if you don't like the grid or the header, you can disable those:

```rust
let printer = PrettyPrinter::default()
    .header(false)
    .grid(false)
    .language("ruby")
    .build()?;

let example = r#"
def fib(n)        
    return 1 if n <= 1
    fib(n-1) + fib(n-2)
end
"#;
printer.string_with_header(example, "fib.rb")?;
```

## Why?

[`syntect`](https://github.com/trishume/syntect/) is a great package for highlighting text.  
When writing a command-line tool that prints text however, you might be looking for some more functionality. This includes the following:

* Line numbers
* More built-in color-themes
* Automatic pagination
* Printing the filename at the top
* Proper terminal handling
* Showing non-printable characters
* Windows support

`prettyprint` offers all of this in one crate.  

## Credits

`prettyprint` is simply a fork of [`bat`](https://github.com/sharkdp/bat/), with some functionality stripped out and bundled up as a library. All credits go to the original authors.