// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package features

import (
	"bufio"
	"bytes"
	"errors"
	"fmt"
	"hash"
	"io"
	"strings"

	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

type FeatureHashFun func() hash.Hash32

type FeatureVectorBuilder struct {
	featureVector golinear.FeatureVector
}

func NewFeatureVectorBuilder() *FeatureVectorBuilder {
	// Allocate opportunistically.
	return &FeatureVectorBuilder{make(golinear.FeatureVector, 0, 10)}
}

func (fvb *FeatureVectorBuilder) Add(fv golinear.FeatureValue) {
	fvb.featureVector = append(fvb.featureVector, fv)
}

func (fvb *FeatureVectorBuilder) Build() golinear.FeatureVector {
	return fvb.featureVector
}

type Feature interface {
	Hash(hf FeatureHashFun) uint32
	String() string
}

type FeatureSet map[string]float64

type FeatureGenerator interface {
	Generate(c *system.Configuration) FeatureSet
	GenerateHashed(c *system.Configuration, hf FeatureHashFun, fvb *FeatureVectorBuilder)
}

// Functions that create a feature generators from a (possibly
// empty) list of arguments.
type FeatureGeneratorFactory func([]string) (FeatureGenerator, error)

type FeatureGeneratorFactories map[string]FeatureGeneratorFactory

type FeatureVectorizer struct {
	featureNumbers map[string]int
}

func NewFeatureVectorizer() FeatureVectorizer {
	return FeatureVectorizer{make(map[string]int)}
}

func (v FeatureVectorizer) Vectorize(features FeatureSet, addNew bool) golinear.FeatureVector {
	fVector := make(golinear.FeatureVector, 0, len(features))

	for feature, value := range features {
		idx, ok := v.featureNumbers[feature]

		if !ok {
			if addNew {
				idx = len(v.featureNumbers) + 1
				v.featureNumbers[feature] = idx
			} else {
				// Unknown feature
				continue
			}
		}

		fVector = append(fVector, golinear.FeatureValue{Index: idx, Value: value})
	}

	return fVector
}

func (v FeatureVectorizer) MarshalText() (text []byte, err error) {
	var buf bytes.Buffer

	for feature, value := range v.featureNumbers {
		buf.WriteString(fmt.Sprintf("%s|%d\n", feature, value))
	}

	return buf.Bytes(), nil
}

type LabelNumberer struct {
	labelNumbers map[system.Transition]int
	labels       []system.Transition
}

func NewLabelNumberer() LabelNumberer {
	return LabelNumberer{make(map[system.Transition]int), make([]system.Transition, 0)}
}

func (l *LabelNumberer) Number(label system.Transition) int {
	idx, ok := l.labelNumbers[label]

	if !ok {
		idx = len(l.labelNumbers)
		l.labelNumbers[label] = idx
		l.labels = append(l.labels, label)
	}

	return idx
}

func (l LabelNumberer) Label(number int) system.Transition {
	return l.labels[number]
}

func (l LabelNumberer) Size() int {
	return len(l.labels)
}

func (l *LabelNumberer) Read(reader io.Reader, serializer system.TransitionSerializer) error {
	labels := make([]system.Transition, 0)
	bufReader := bufio.NewReader(reader)

	eof := false
	for !eof {
		line, err := bufReader.ReadString('\n')
		if err != nil {
			if err != io.EOF {
				return err
			}

			eof = true
		}

		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if trans, err := serializer.DeserializeTransition(strings.TrimSpace(line)); err == nil {
			labels = append(labels, trans)
		} else {
			return err
		}
	}

	numbers := make(map[system.Transition]int)
	for idx, label := range labels {
		numbers[label] = idx
	}

	l.labels = labels
	l.labelNumbers = numbers

	return nil
}

func (l *LabelNumberer) WriteLabelNumberer(writer io.Writer, serializer system.TransitionSerializer) error {
	for _, trans := range l.labels {
		if transStr, err := serializer.SerializeTransition(trans); err != nil {
			return err
		} else {
			fmt.Fprintf(writer, "%s\n", transStr)
		}
	}

	return nil
}

type AggregateGenerator struct {
	featureGenerators []FeatureGenerator
}

func NewAggregateGenerator(generators []FeatureGenerator) FeatureGenerator {
	return AggregateGenerator{generators}
}

func (a AggregateGenerator) Generate(c *system.Configuration) FeatureSet {
	combined := make(FeatureSet)

	for _, generator := range a.featureGenerators {
		for feature, value := range generator.Generate(c) {
			combined[feature] = value
		}
	}

	return combined
}

func (a AggregateGenerator) GenerateHashed(c *system.Configuration, hf FeatureHashFun,
	fvb *FeatureVectorBuilder) {
	for _, generator := range a.featureGenerators {
		generator.GenerateHashed(c, hf, fvb)
	}
}

// Read feature descriptions with the default set of generators.
func ReadFeatureGeneratorsDefault(reader *bufio.Reader) (FeatureGenerator, error) {
	return ReadFeatureGenerators(FeatureGeneratorFactories{
		"addr": parseAddressedValueGenerator,
	}, reader)
}

func ReadFeatureGenerators(fs FeatureGeneratorFactories,
	reader *bufio.Reader) (FeatureGenerator, error) {
	var eof = false

	generators := make([]FeatureGenerator, 0)

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

		parts := strings.Split(line, " ")
		g, err := parseGenerator(fs, parts)
		if err != nil {
			return nil, err
		}

		generators = append(generators, g)
	}

	return AggregateGenerator{generators}, nil
}

func parseGenerator(fs FeatureGeneratorFactories, parts []string) (FeatureGenerator, error) {
	if len(parts) == 0 {
		return nil, errors.New("Line should at the very least specify a generator.")
	}

	factory, ok := fs[parts[0]]
	if !ok {
		return nil, fmt.Errorf("Unknown generator: %s", parts[0])
	}

	return factory(parts[1:])
}
