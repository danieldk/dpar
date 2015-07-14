// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package svm

import (
	"github.com/danieldk/dpar/features/symbolic"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

type GoLinearCollector interface {
	system.InstanceCollector
	Problem() *golinear.Problem
}

type FeatureCollector struct {
	featureVectorizer FeatureVectorizer
	labelNumberer     system.LabelNumberer
	featureGenerator  symbolic.FeatureGenerator
	problem           *golinear.Problem
}

func NewFeatureCollector(featureGenerator symbolic.FeatureGenerator) *FeatureCollector {
	return &FeatureCollector{NewFeatureVectorizer(), system.NewLabelNumberer(),
		featureGenerator, golinear.NewProblem()}
}

func (fc *FeatureCollector) Collect(t system.Transition, c *system.Configuration) {
	label := fc.labelNumberer.Number(t)
	features := fc.featureVectorizer.Vectorize(fc.featureGenerator.Generate(c), true)
	fc.problem.Add(golinear.TrainingInstance{Label: float64(label), Features: features})
}

func (fc *FeatureCollector) Problem() *golinear.Problem {
	return fc.problem
}

func (fc *FeatureCollector) FeatureVectorizer() FeatureVectorizer {
	return fc.featureVectorizer
}

func (fc *FeatureCollector) LabelNumberer() *system.LabelNumberer {
	return &fc.labelNumberer
}

type HashCollector struct {
	labelNumberer    system.LabelNumberer
	featureGenerator symbolic.FeatureGenerator
	featureHash      symbolic.FeatureHashFun
	problem          *golinear.Problem
	maxFeatures      uint
}

func NewHashCollector(featureGenerator symbolic.FeatureGenerator,
	featureHash symbolic.FeatureHashFun, maxFeatures uint) *HashCollector {
	return &HashCollector{
		labelNumberer:    system.NewLabelNumberer(),
		featureGenerator: featureGenerator,
		featureHash:      featureHash,
		problem:          golinear.NewProblem(),
		maxFeatures:      maxFeatures,
	}
}

func (hc *HashCollector) Collect(t system.Transition, c *system.Configuration) {
	label := hc.labelNumberer.Number(t)
	vecBuilder := NewGolinearVectorBuilder()
	hc.featureGenerator.GenerateHashed(c, hc.featureHash, vecBuilder)

	featuresByIndex := make(map[int]float64)

	for _, fv := range vecBuilder.Build() {
		hash := fv.Index

		sign := 1.
		if hash&0x80000000 == 0x80000000 {
			sign = -1
		}

		fv.Index = ((hash & 0x7fffffff) % int(hc.maxFeatures)) + 1
		fv.Value *= sign

		if val, ok := featuresByIndex[fv.Index]; ok {
			featuresByIndex[fv.Index] = val + fv.Value
		} else {
			featuresByIndex[fv.Index] = fv.Value
		}
	}

	features := make([]golinear.FeatureValue, 0, len(featuresByIndex))
	for idx, val := range featuresByIndex {
		features = append(features, golinear.FeatureValue{idx, val})
	}

	hc.problem.Add(golinear.TrainingInstance{Label: float64(label), Features: features})
}

func (hc *HashCollector) Problem() *golinear.Problem {
	return hc.problem
}

func (hc *HashCollector) LabelNumberer() *system.LabelNumberer {
	return &hc.labelNumberer
}
