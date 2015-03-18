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

type SVMGuide struct {
	model             *golinear.Model
	featureGenerator  features.FeatureGenerator
	featureVectorizer features.FeatureVectorizer
	labelNumberer     features.LabelNumberer
}

func NewSVMGuide(model *golinear.Model, featureGenerator features.FeatureGenerator,
	featureVectorizer features.FeatureVectorizer, labelNumberer features.LabelNumberer) system.Guide {
	return &SVMGuide{model, featureGenerator, featureVectorizer, labelNumberer}
}

func (g *SVMGuide) BestTransition(configuration *system.Configuration, transitions system.TransitionSet) system.Transition {
	possibleLabels := make(map[reflect.Type]interface{})
	for transition := range transitions {
		possibleLabels[reflect.TypeOf(transition)] = nil
	}

	x := g.featureVectorizer.Vectorize(g.featureGenerator.Generate(configuration), false)
	_, values, _ := g.model.PredictDecisionValuesSlice(x)

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
