// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

// The ArcEagerOracle returns a correct transition for a particular
// configuration in the arc-eager system. The transition choice in
// an oracle is based on the gold-standard parse.
type ArcEagerOracle struct {
	dependentHeadMapping map[uint]Dependency
}

// NewArcEagerOracle returns a new oracle from the arc-eager system.
// The gold dependency set is used to choose a correct transition from
// each configuration.
func NewArcEagerOracle(goldDependencies DependencySet) Guide {
	return &ArcEagerOracle{goldDependencies.CreateDependentHeadMapping()}
}

// BestTransition returns the best transition from the current
// configuration, as inferred from the gold standard dependency
// structure.
func (o *ArcEagerOracle) BestTransition(c *Configuration) Transition {
	if len(c.Buffer) == 0 {
		panic("Applying oracle to terminal configuration")
	}

	stackSize := len(c.Stack)
	if stackSize != 0 {
		stackTip := c.Stack[stackSize-1]
		bufferHead := c.Buffer[0]

		// Left-Arc or Right-Arc is the next transition if:
		//
		// - The transition itself is possible.
		// - Following the transition would introduce a dependency
		//   that corresponds to the gold standard.
		la := aeLeftArc{o.dependentHeadMapping[stackTip].Relation}
		if la.IsPossible(*c) && o.dependentHeadMapping[stackTip].Head == bufferHead {
			return la
		}

		ra := aeRightArc{o.dependentHeadMapping[bufferHead].Relation}
		if ra.IsPossible(*c) && o.dependentHeadMapping[bufferHead].Head == stackTip {
			return ra
		}

		// reduce is the next transition if:
		//
		// - reduce is possible.
		// - The next token in the buffer is in a dependency relation with a token that precedes
		//   the token on the tip of the stack.
		r := aeReduce{}
		if r.IsPossible(*c) && o.nextAttached(c) {
			return r
		}
	}

	return aeShift{}
}

func (o *ArcEagerOracle) nextAttached(c *Configuration) bool {
	stackTip := c.Stack[len(c.Stack)-1]
	bufferHead := c.Buffer[0]

	for dependent, relation := range o.dependentHeadMapping {
		if dependent == bufferHead && relation.Head < stackTip {
			return true
		}

		if dependent < stackTip && relation.Head == bufferHead {
			return true
		}
	}

	return false
}
