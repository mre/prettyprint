# prettyprint - syntax highlighting with batteries included

...a surpisingly creative name for a delightful syntax highlighting package.

## Why?

`syntect` is a great package for highlighting text.
When writing a commandline interface, that prints sourcecode however, you might be looking for more functionality:

* Line numbers
* More built-in color-themes
* Automatic pagination
* Printing the filename at the top
* Proper terminal handling
* Showing non-printable characters
* Windows support

`prettyprint` provides all this, thanks to [`bat`](https://github.com/sharkdp/bat/), from which is was forked.