// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"errors"
	"fmt"
	"strings"
)

var archetypeSPShift = spShift{}
var archetypeSPLeftArc = spLeftArc{"<archetype>"}
var archetypeSPRightArc = spRightArc{"<archetype>"}

type StackProjective struct {
	archetypeTransitions TransitionSet
}

func NewStackProjective() TransitionSystem {
	trans := map[Transition]interface{}{
		archetypeSPShift:    nil,
		archetypeSPLeftArc:  nil,
		archetypeSPRightArc: nil,
	}

	return &StackProjective{trans}
}

func (ts *StackProjective) IsTerminal(c Configuration) bool {
	return len(c.Stack) == 0 && len(c.Buffer) == 0
}

func (ts *StackProjective) PossibleTransitions(configuration Configuration) TransitionSet {
	possible := make(TransitionSet)

	for trans := range ts.archetypeTransitions {
		if trans.IsPossible(configuration) {
			possible[trans] = nil
		}
	}

	return possible
}

func (ts *StackProjective) SerializeTransition(t Transition) (string, error) {
	switch t := t.(type) {
	case spLeftArc:
		return fmt.Sprintf("LEFT_ARC %s", t.relation), nil
	case spRightArc:
		return fmt.Sprintf("RIGHT_ARC %s", t.relation), nil
	case spShift:
		return "SHIFT", nil
	default:
		return "", fmt.Errorf("Not a transition of the stack-projective system: %T", t)
	}
}

func (ts *StackProjective) DeserializeTransition(transString string) (Transition, error) {
	parts := strings.SplitN(transString, " ", 2)

	if len(parts) == 0 {
		return nil, errors.New("Empty transition")
	}

	switch parts[0] {
	default:
		return nil, fmt.Errorf("Unknown transition: %s", parts[0])
	case "LEFT_ARC":
		if len(parts) == 1 {
			return nil, errors.New("Left-Arc transition requires label argument")
		}
		return spLeftArc{parts[1]}, nil
	case "RIGHT_ARC":
		if len(parts) == 1 {
			return nil, errors.New("Right-Arc transition requires label argument")
		}
		return spRightArc{parts[1]}, nil
	case "SHIFT":
		return spShift{}, nil
	}
}

type spLeftArc struct {
	relation string
}

func (l spLeftArc) IsPossible(c Configuration) bool {
	stackSize := len(c.Stack)
	return len(c.Stack) > 1 && c.Stack[stackSize-2] != 0
}

func (l spLeftArc) Apply(c *Configuration) {
	stack := c.Stack

	stackSize := len(stack)
	head := stack[stackSize-1]
	dependant := stack[stackSize-2]
	dependency := Dependency{head, l.relation, dependant}

	c.AddDependency(&dependency)

	c.Stack = append(stack[:stackSize-2], head)
}

type spRightArc struct {
	relation string
}

func (r spRightArc) IsPossible(configuration Configuration) bool {
	return len(configuration.Stack) > 1
}

func (r spRightArc) Apply(c *Configuration) {
	stack := c.Stack

	stackSize := len(stack)
	dependant := stack[stackSize-1]
	head := stack[stackSize-2]
	dependency := Dependency{head, r.relation, dependant}

	c.AddDependency(&dependency)

	c.Stack = stack[:stackSize-1]
}

type spShift struct{}

func (s spShift) IsPossible(c Configuration) bool {
	return len(c.Buffer) != 0 || len(c.Stack) != 0
}

func (s spShift) Apply(c *Configuration) {
	stack := c.Stack
	buffer := c.Buffer

	if len(buffer) != 0 {
		c.Stack = append(stack, buffer[0])
		c.Buffer = buffer[1:]
	} else if len(stack) != 0 {
		c.Stack = stack[:len(stack)-1]
	}
}

type StackProjectiveOracle struct {
	dependentHeadMapping map[uint]Dependency
}

func NewStackProjectiveOracle(goldDependencies DependencySet) Guide {
	oracle := StackProjectiveOracle{goldDependencies.CreateDependentHeadMapping()}
	return &oracle
}

func (o *StackProjectiveOracle) BestTransition(c *Configuration) Transition {
	stack := c.Stack

	if len(stack) > 1 {
		stackSize := len(stack)
		stack0 := stack[stackSize-1]
		stack1 := stack[stackSize-2]

		la := spLeftArc{o.dependentHeadMapping[stack1].Relation}
		if la.IsPossible(*c) && o.dependentHeadMapping[stack1].Head == stack0 {
			return la
		}

		ra := spRightArc{o.dependentHeadMapping[stack0].Relation}
		if ra.IsPossible(*c) &&
			o.dependentHeadMapping[stack0].Head == stack1 &&
			!o.neededForAttachment(c, stack0) {
			return ra
		}
	}

	return spShift{}
}

func (o *StackProjectiveOracle) neededForAttachment(c *Configuration, token uint) bool {
	for _, bufToken := range c.Buffer {
		if o.dependentHeadMapping[bufToken].Head == token {
			return true
		}
	}

	return false
}
