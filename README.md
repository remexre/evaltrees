# evaltrees

[![Build Status](https://travis-ci.org/remexre/evaltrees.svg?branch=master)](https://travis-ci.org/remexre/evaltrees)
[![Dependency Status](https://deps.rs/repo/github/remexre/evaltrees/status.svg)](https://deps.rs/repo/github/remexre/evaltrees)

A simple functional language, and a small-step interpreter for it.
Additionally, shows the evaluation of the "main expression" as it proceeds.
Intended as an instructional aid for CSCI2041 at the University of Minnesota.

The interpreter can be run in call-by-name, call-by-need, or call-by-value modes.
Expression evaluations can be show as concrete or abstract syntax.

See `doc/syntax.md` for a description of the language's syntax.

## Missing Features

(These are features that will probably not be implemented (although PRs are welcome))

 - Mutual Recursion
   - Currently, this breaks typechecking
 - Polymorphic Recursion
   - This makes type inference undecidable
 - Higher-rank Polymorphism
   - The reification procedure for types assumes rank-1 polymorphism
   - This makes type inference undecidable

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
