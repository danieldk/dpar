// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package ml

import (
	"github.com/danieldk/conllx"
	"github.com/danieldk/dpar/features"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

type InstanceCollector interface {
	Collect(t system.Transition, c *system.Configuration)
	Problem() *golinear.Problem
	LabelNumberer() *features.LabelNumberer
}

type FeatureCollector struct {
	featureVectorizer features.FeatureVectorizer
	labelNumberer     features.LabelNumberer
	featureGenerator  features.FeatureGenerator
	problem           *golinear.Problem
}

func NewFeatureCollector(featureGenerator features.FeatureGenerator) *FeatureCollector {
	return &FeatureCollector{features.NewFeatureVectorizer(), features.NewLabelNumberer(),
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

func (fc *FeatureCollector) FeatureVectorizer() features.FeatureVectorizer {
	return fc.featureVectorizer
}

func (fc *FeatureCollector) LabelNumberer() *features.LabelNumberer {
	return &fc.labelNumberer
}

type HashCollector struct {
	labelNumberer    features.LabelNumberer
	featureGenerator features.FeatureGenerator
	featureHash      features.FeatureHashFun
	problem          *golinear.Problem
	maxFeatures      uint
}

func NewHashCollector(featureGenerator features.FeatureGenerator,
	featureHash features.FeatureHashFun, maxFeatures uint) *HashCollector {
	return &HashCollector{
		labelNumberer:    features.NewLabelNumberer(),
		featureGenerator: featureGenerator,
		featureHash:      featureHash,
		problem:          golinear.NewProblem(),
		maxFeatures:      maxFeatures,
	}
}

func (hc *HashCollector) Collect(t system.Transition, c *system.Configuration) {
	label := hc.labelNumberer.Number(t)
	vecBuilder := features.NewFeatureVectorBuilder()
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

func (hc *HashCollector) LabelNumberer() *features.LabelNumberer {
	return &hc.labelNumberer
}

type GreedyTrainer struct {
	transitionSystem system.TransitionSystem
	collector        InstanceCollector
}

func NewGreedyTrainer(transitionSystem system.TransitionSystem,
	collector InstanceCollector) GreedyTrainer {
	return GreedyTrainer{transitionSystem, collector}
}

func (t *GreedyTrainer) Parse(tokens []conllx.Token, oracle system.Guide) (system.DependencySet, error) {
	c, err := system.NewConfiguration(tokens)
	if err != nil {
		return nil, err
	}

	t.parseConfiguration(&c, oracle)
	return c.Dependencies(), nil
}

func (t *GreedyTrainer) parseConfiguration(c *system.Configuration, oracle system.Guide) {
	for !t.transitionSystem.IsTerminal(*c) {
		// Find next transition
		possible := t.transitionSystem.PossibleTransitions(*c)
		nextTransition := oracle.BestTransition(c, possible)

		// Collect training instance.
		t.collector.Collect(nextTransition, c)

		// Follow the transition
		nextTransition.Apply(c)
	}
}
