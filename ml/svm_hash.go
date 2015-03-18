// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package ml

import (
	"math"
	"reflect"

	"github.com/danieldk/dpar/features"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

type HashingSVMGuide struct {
	model            *golinear.Model
	featureGenerator features.FeatureGenerator
	labelNumberer    features.LabelNumberer
	hashFunc         features.FeatureHashFun
	maxFeatures      uint
}

func NewHashingSVMGuide(model *golinear.Model, featureGenerator features.FeatureGenerator,
	labelNumberer features.LabelNumberer, hf features.FeatureHashFun, maxFeatures uint) system.Guide {
	return &HashingSVMGuide{model, featureGenerator, labelNumberer, hf, maxFeatures}
}

func (g *HashingSVMGuide) BestTransition(configuration *system.Configuration, transitions system.TransitionSet) system.Transition {
	possibleLabels := make(map[reflect.Type]interface{})
	for transition := range transitions {
		possibleLabels[reflect.TypeOf(transition)] = nil
	}

	vecBuilder := features.NewFeatureVectorBuilder()
	g.featureGenerator.GenerateHashed(configuration, g.hashFunc, vecBuilder)
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
		_, permitted := possibleLabels[reflect.TypeOf(label)]
		if !permitted {
			continue
		}

		bestValue = value
		bestLabel = label
	}

	return bestLabel
}
