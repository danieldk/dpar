package symbolic

import (
	"bufio"
	"errors"
	"fmt"
	"hash"
	"io"
	"strings"

	"github.com/danieldk/dpar/features/addr"
	"github.com/danieldk/dpar/system"
)

// A FeatureHashFun is a function that produces a hash function.
type FeatureHashFun func() hash.Hash32

// A FeatureVectorBuilder constructs a sparse feature vector.
type FeatureVectorBuilder interface {
	Add(feature int, value float64)
}

// A FeatureGenerator generates concrete features based on a
// parser configuration. Features are represented as hash codes.
type FeatureGenerator interface {
	Generate(c *system.Configuration, hf FeatureHashFun, fvb FeatureVectorBuilder)
}

// A FeatureGeneratorFactory creates a feature generators from a (possibly
// empty) list of arguments.
type FeatureGeneratorFactory func([]byte) (FeatureGenerator, error)

// A FeatureGeneratorFactories stores the mapping from a class of feature
// generators (represented as a string) and the factory for that class of
// feature generators.
type FeatureGeneratorFactories map[string]FeatureGeneratorFactory

// An AggregateGenerator is a feature generator returns the set union of
// the output of the generators it wraps.
type AggregateGenerator struct {
	featureGenerators []FeatureGenerator
}

// NewAggregateGenerator constructs an aggregate feature generator from a
// slice of feature generators.
func NewAggregateGenerator(generators []FeatureGenerator) FeatureGenerator {
	return AggregateGenerator{generators}
}

// Generate generates features from a given configuration and using the provided
// feature hash function. The result is stored with the provided feature vector
// builder.
func (a AggregateGenerator) Generate(c *system.Configuration, hf FeatureHashFun,
	fvb FeatureVectorBuilder) {
	for _, generator := range a.featureGenerators {
		generator.Generate(c, hf, fvb)
	}
}

// ReadFeatureGeneratorsDefault reads feature descriptions with the default
// set of generators.
func ReadFeatureGeneratorsDefault(reader *bufio.Reader) (FeatureGenerator, error) {
	return ReadFeatureGenerators(FeatureGeneratorFactories{
		"addr": parseAddressedValueGenerator,
	}, reader)
}

// ReadFeatureGenerators reads feature (generator) descriptions from a reader.
func ReadFeatureGenerators(fs FeatureGeneratorFactories,
	reader *bufio.Reader) (FeatureGenerator, error) {
	var eof = false

	var generators []FeatureGenerator

	for !eof {
		line, err := reader.ReadString('\n')

		if err != nil {
			if err == io.EOF {
				eof = true
			} else {
				return nil, err
			}
		}

		line = strings.TrimSpace(line)

		if line == "" {
			continue
		}

		g, err := parseGenerator(fs, line)
		if err != nil {
			return nil, err
		}

		generators = append(generators, g)
	}

	return AggregateGenerator{generators}, nil
}

func parseGenerator(fs FeatureGeneratorFactories, line string) (FeatureGenerator, error) {
	sepIdx := strings.IndexByte(line, ' ')
	if sepIdx == -1 {
		return nil, errors.New("Line should at the very least specify a generator.")
	}

	generatorName := line[:sepIdx]
	factory, ok := fs[generatorName]
	if !ok {
		return nil, fmt.Errorf("Unknown generator: %s", generatorName)
	}

	return factory([]byte(line[sepIdx+1:]))
}

func parseAddressedValueGenerator(data []byte) (FeatureGenerator, error) {
	templates, err := addr.ParseAddressedValueTemplates(data)
	if err != nil {
		return nil, err
	}

	return NewAddressedValueGenerator(templates), nil
}
