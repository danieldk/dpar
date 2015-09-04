// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package svm

import (
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
