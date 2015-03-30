# dpar - Decaffeinated/Dependency Parser

**Warning:** dpar is not ready for use yet, APIs will break and burn.

## Introduction

dpar started out as a simple dependency parser in Java for teaching
purposes. It was then rewritten in Go, because I like Go more :). The goal
of dpar is twofold:

1. To provide basic building blocks to construct transition-based dependency
   parsers.
2. To provide a fast, traditional, feature-based, dependency parser.
3. To provide a dependency parser that can be embedded easily in Go
   applications.

As a demonstration of dpar's modularity, the separate (soon to-be released)
*dparnn* project provides a dependency parser that only uses
word/tag/relation embeddings as input and uses a multi-layer perceptron for
classification.

## Usage

No usage information is provided yet, since dpar is not ready for use. If
you want to hack on dpar, you are smart enough to figure it out :).

## TODO

There is still a lot of work to be done:

* Polish the API
* Write more godoc
* Implementation of dynamic oracles
* Pseudo-projective parsing
* Optimization
* ~~Come up with an archive format?~~ Encourage model per-directory.
* Provide models

Also missing is an actual 'parse' command, which will only be added when
I am happy with the API and performance.

## License

dpar is governed by a BSD-style license that can be found in the LICENSE
file. The list of authors is in the AUTHORS file.

## Contact

Use the dpar issue tracker to report problems with dpar:

https://github.com/danieldk/dpar/issues
