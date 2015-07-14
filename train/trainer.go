// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package train

import (
	"github.com/danieldk/conllx"
	"github.com/danieldk/dpar/system"
)

type InstanceCollector interface {
	Collect(t system.Transition, c *system.Configuration)
	LabelNumberer() *system.LabelNumberer
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
		nextTransition := oracle.BestTransition(c)

		// Collect training instance.
		t.collector.Collect(nextTransition, c)

		// Follow the transition
		nextTransition.Apply(c)
	}
}
