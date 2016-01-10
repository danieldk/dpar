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

## Installation

The command-line utilities and library can be installed using the `go` command:

~~~
go get -u github.com/danieldk/dpar/...
~~~

**Note:** the [golinear](github.com/danieldk/golinear) dependency requires that
you have installed [liblinear](https://www.csie.ntu.edu.tw/~cjlin/liblinear/).

Precompiled binaries will be provided in the future.

## Usage

### Training

To train a parser, you need:

  * A dependency treebank in CoNLL-X format.
  * A feature specification file ([basic example](example/example.features), [example using morphology](example/clarin-2015.features))
  * A parser configuration file ([example](example/parser.conf))

You can then train the parser using `dpar-train`:

~~~
dpar-train parser.conf train-treebank.conll
~~~

Training will create a features file and a model (with the filenames specified in the parser configuration).

### Evaluation

The parser can be evaluated using `dpar-evaluate`:

~~~
dpar-evaluate parser.conf eval-treebank.conll
~~~

### Parsing

The `dpar-parse` command can be used to parse new data. The input should be in CoNLL-X format, containing
the columns used in the feature specification. The output is also in CoNLL-X format:

~~~
dpar-evaluate parser.conf unparsed.conll > parsed.conll
~~~

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
