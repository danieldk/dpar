// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package svm

import (
	"math"

	"github.com/danieldk/dpar/features/symbolic"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

var _ system.Guide = &HashingSVMGuide{}

// HashingSVMGuide is a parser guide that uses a linear SVM classifier
// in conjunction with a hash kernel.
type HashingSVMGuide struct {
	model            *golinear.Model
	featureGenerator symbolic.FeatureGenerator
	labelNumberer    system.LabelNumberer
	hashFunc         symbolic.FeatureHashFun
	maxFeatures      uint
}

// NewHashingSVMGuide returns a parser guide. The required arguments are:
// the linear SVM classifier (model), the feature generator, the label <=>
// number bijection (labelNumberer), the hash function used for the hash kernel,
// and the size of the hash kernel.
//
// All the arguments except the model, should be the same as what was used
// during training.
func NewHashingSVMGuide(model *golinear.Model, featureGenerator symbolic.FeatureGenerator,
	labelNumberer system.LabelNumberer, hf symbolic.FeatureHashFun, maxFeatures uint) *HashingSVMGuide {
	return &HashingSVMGuide{model, featureGenerator, labelNumberer, hf, maxFeatures}
}

// BestTransition returns the best transition according to the transition classifier
// used by the guide.
func (g *HashingSVMGuide) BestTransition(configuration *system.Configuration) system.Transition {
	vecBuilder := newGoLinearVectorBuilder()
	g.featureGenerator.Generate(configuration, g.hashFunc, vecBuilder)
	x := vecBuilder.Build()

	// Introduce feature bounds.
	for idx, fv := range x {
		hash := fv.Index

		sign := 1.
		if hash&0x80000000 == 0x80000000 {
			sign = -1
		}

		x[idx].Index = ((hash & 0x7fffffff) % int(g.maxFeatures)) + 1
		x[idx].Value *= sign
	}

	_, values, _ := g.model.PredictDecisionValuesSlice(x)

	// XXX: large overlap with SVMGuide.Bestsystem.Transition. Factor out!

	var bestLabel system.Transition
	bestValue := math.Inf(-1)
	labels := g.model.Labels()

	for idx, value := range values {
		if value < bestValue {
			continue
		}

		numLabel := labels[idx]
		label := g.labelNumberer.Label(numLabel)
		if !label.IsPossible(*configuration) {
			continue
		}

		bestValue = value
		bestLabel = label
	}

	return bestLabel
}
