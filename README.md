[![Report card](http://goreportcard.com/badge/danieldk/dpar)](http://goreportcard.com/report/danieldk/dpar)

# dpar - Decaffeinated/Dependency Parser

## Introduction

dpar is a transition-based dependency parser in Go. The goal of dpar is
threefold:

1. To provide a fast, traditional, feature-based, dependency parser.
2. To provide a dependency parser that can be embedded easily in Go
   applications.
3. To provide the basic building blocks for new research/experiments
   in dependency parsing.

To encourage and demonstrate the modularity of dpar, the
[dparnn](http://github.com/danieldk/dparnn) parser was developed in
parallel. dparnn is a neural net dependency parser using word/tag/relation
embeddings that dpar packages for transition systems, feature specification,
and parsing.

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
