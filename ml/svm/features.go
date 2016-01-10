// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package svm

import (
	"github.com/danieldk/dpar/features/symbolic"
	"gopkg.in/danieldk/golinear.v1"
)

var _ symbolic.FeatureVectorBuilder = &goLinearVectorBuilder{}

type goLinearVectorBuilder struct {
	featureVector golinear.FeatureVector
}

func newGoLinearVectorBuilder() *goLinearVectorBuilder {
	// Allocate opportunistically.
	return &goLinearVectorBuilder{make(golinear.FeatureVector, 0, 10)}
}

func (fvb *goLinearVectorBuilder) Add(index int, value float64) {
	fvb.featureVector = append(fvb.featureVector, golinear.FeatureValue{index, value})
}

func (fvb *goLinearVectorBuilder) Build() golinear.FeatureVector {
	return fvb.featureVector
}
