# dpar - Decaffeinated/Dependency Parser

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

## Documentation

* [Usage information](https://github.com/danieldk/dpar/wiki)
* [API documentation](https://godoc.org/github.com/danieldk/dpar) (minor changes expected)
* [Todo items](https://github.com/danieldk/dpar/labels/enhancement)

## Status

The dpar command-line tools are fully functional. The public API can be used
to embed dpar in other applications, but will see some breaking changes. If
you rely on the current API, use vendoring.

## License

dpar is governed by a BSD-style license that can be found in the LICENSE
file. The list of authors is in the AUTHORS file.

## Contact

Use the dpar issue tracker to report problems with dpar:

https://github.com/danieldk/dpar/issues
