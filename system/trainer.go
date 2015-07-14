// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import "github.com/danieldk/conllx"

type InstanceCollector interface {
	Collect(t Transition, c *Configuration)
	LabelNumberer() *LabelNumberer
}

type GreedyTrainer struct {
	transitionSystem TransitionSystem
	collector        InstanceCollector
}

func NewGreedyTrainer(transitionSystem TransitionSystem,
	collector InstanceCollector) GreedyTrainer {
	return GreedyTrainer{transitionSystem, collector}
}

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
