// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import "github.com/danieldk/conllx"

// An InstanceCollector collects training instances. Typically,
// they are written to a file or stored in memory for training a
// classifier.
type InstanceCollector interface {
	Collect(t Transition, c *Configuration)
	LabelNumberer() *LabelNumberer
}

// GreedyTrainer is a trainer for greedy transition-based parsers.
// Such greedy parsers only follow one transition from a given state.
type GreedyTrainer struct {
	transitionSystem TransitionSystem
	collector        InstanceCollector
}

// NewGreedyTrainer creates a GreedyTrainer instance, using a
// transition system and training instance collector.
func NewGreedyTrainer(transitionSystem TransitionSystem,
	collector InstanceCollector) GreedyTrainer {
	return GreedyTrainer{transitionSystem, collector}
}

// Parse a sentence using the provided guide.
func (t *GreedyTrainer) Parse(tokens []conllx.Token, oracle Guide) (DependencySet, error) {
	c, err := NewConfiguration(tokens)
	if err != nil {
		return nil, err
	}

	t.parseConfiguration(&c, oracle)
	return c.Dependencies(), nil
}

func (t *GreedyTrainer) parseConfiguration(c *Configuration, oracle Guide) {
	for !t.transitionSystem.IsTerminal(*c) {
		// Find next transition
		nextTransition := oracle.BestTransition(c)

		// Collect training instance.
		t.collector.Collect(nextTransition, c)

		// Follow the transition
		nextTransition.Apply(c)
	}
}
