# kjspkg

A package manager & registry for KubeJS.

## Why is this here?

KJSPKG is the reason this was developed in the first place (huge thank you to @GCat101
and @tizu69 for making the original version, it's insanely cool), and it also serves as
a great example of how to set up a proper ModHost-based server. It's pretty simple (only
74 lines of Rust), and the rest is handled in the config, which is written in
[`pkl`](https://pkl-lang.org/) for type safety, validation, and ease of use (as well as
documentation of all the values!!).

The entire executable (compiled on Linux/aarch64), at the time of writing, weighs in at
39 megabytes (at the time of writing), and includes the entire UI source code as well as
all the normal binary stuff. Yeah, it's a big one. But all that... girth... allows you to
customize the server on the fly with only needing to restart the server, not recompile it.
It compiles the UI (only in release mode) on startup for you, everything handled, automatically.
