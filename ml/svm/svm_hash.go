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

type HashingSVMGuide struct {
	model            *golinear.Model
	featureGenerator symbolic.FeatureGenerator
	labelNumberer    system.LabelNumberer
	hashFunc         symbolic.FeatureHashFun
	maxFeatures      uint
}

func NewHashingSVMGuide(model *golinear.Model, featureGenerator symbolic.FeatureGenerator,
	labelNumberer system.LabelNumberer, hf symbolic.FeatureHashFun, maxFeatures uint) *HashingSVMGuide {
	return &HashingSVMGuide{model, featureGenerator, labelNumberer, hf, maxFeatures}
}

func (g *HashingSVMGuide) BestTransition(configuration *system.Configuration) system.Transition {
	vecBuilder := NewGolinearVectorBuilder()
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
