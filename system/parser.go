// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"github.com/danieldk/conllx"
)

// A Guide provides the best transition for the current configuration.
type Guide interface {
	BestTransition(configuration *Configuration) Transition
}

// A Parser returns the dependency set for a sentence.
type Parser interface {
	Parse(tokens []conllx.Token) (DependencySet, error)
}

var _ Parser = &GreedyParser{}

// GreedyParser is a parser that is deterministic -- it returns only
// one possible parse. Greedy parsers are O(n) where n is the length
// of the input sentence.
type GreedyParser struct {
	transitionSystem TransitionSystem
	guide            Guide
}

// NewGreedyParser constructs a GreedyParser using the given transition
// system and guide.
func NewGreedyParser(ts TransitionSystem, guide Guide) *GreedyParser {
	return &GreedyParser{ts, guide}
}

// Parse a sentence, returning the set of dependency relations that
// was found.
func (p *GreedyParser) Parse(tokens []conllx.Token) (DependencySet, error) {
	c, err := NewConfiguration(tokens)
	if err != nil {
		return nil, err
	}

	return p.parseConfiguration(&c), nil
}

func (p *GreedyParser) parseConfiguration(c *Configuration) DependencySet {
	for !p.transitionSystem.IsTerminal(*c) {
		p.guide.BestTransition(c).Apply(c)
	}

	return c.Dependencies()
}
