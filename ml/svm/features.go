// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package svm

import (
	"bytes"
	"fmt"

	"github.com/danieldk/dpar/features/symbolic"
	"gopkg.in/danieldk/golinear.v1"
)

var _ symbolic.FeatureVectorBuilder = &GolinearVectorBuilder{}

type GolinearVectorBuilder struct {
	featureVector golinear.FeatureVector
}

func NewGolinearVectorBuilder() *GolinearVectorBuilder {
	// Allocate opportunistically.
	return &GolinearVectorBuilder{make(golinear.FeatureVector, 0, 10)}
}

func (fvb *GolinearVectorBuilder) Add(index int, value float64) {
	fvb.featureVector = append(fvb.featureVector, golinear.FeatureValue{index, value})
}

func (fvb *GolinearVectorBuilder) Build() golinear.FeatureVector {
	return fvb.featureVector
}

// A feature vectorizer creates a bijection between (string-based)
// features and numbers. This bijection is used to translate
// feature sets to feature vectors.
type FeatureVectorizer struct {
	featureNumbers map[string]int
}

func NewFeatureVectorizer() FeatureVectorizer {
	return FeatureVectorizer{make(map[string]int)}
}

func (v FeatureVectorizer) Vectorize(features symbolic.FeatureSet, addNew bool) golinear.FeatureVector {
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
