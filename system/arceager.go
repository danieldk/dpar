// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"errors"
	"fmt"
	"strings"
)

var archetypeAEShift = aeShift{}
var archetypeAEReduce = aeReduce{}
var archetypeAELeftArc = aeLeftArc{"<archetype>"}
var archeTypeAERightArc = aeRightArc{"<archetype>"}

type ArcEager struct {
	archetypeTransitions TransitionSet
}

func NewArcEager() TransitionSystem {
	trans := map[Transition]interface{}{
		archetypeAEShift:    nil,
		archetypeAEReduce:   nil,
		archetypeAELeftArc:  nil,
		archeTypeAERightArc: nil,
	}

	return &ArcEager{trans}
}

func (ts *ArcEager) IsTerminal(c Configuration) bool {
	return len(c.Buffer) == 0
}

func (ts *ArcEager) PossibleTransitions(configuration Configuration) TransitionSet {
	possible := make(TransitionSet)

	for trans, _ := range ts.archetypeTransitions {
		if trans.IsPossible(configuration) {
			possible[trans] = nil
		}
	}

	return possible
}

func (ts *ArcEager) SerializeTransition(t Transition) (string, error) {
	switch t := t.(type) {
	case aeLeftArc:
		return fmt.Sprintf("LEFT_ARC %s", t.relation), nil
	case aeRightArc:
		return fmt.Sprintf("RIGHT_ARC %s", t.relation), nil
	case aeReduce:
		return "REDUCE", nil
	case aeShift:
		return "SHIFT", nil
	default:
		return "", fmt.Errorf("Not a transition of the arc-eager system: %T", t)
	}
}

func (ts *ArcEager) DeserializeTransition(transString string) (Transition, error) {
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
		return aeLeftArc{parts[1]}, nil
	case "RIGHT_ARC":
		if len(parts) == 1 {
			return nil, errors.New("Right-Arc transition requires label argument")
		}
		return aeRightArc{parts[1]}, nil
	case "REDUCE":
		return aeReduce{}, nil
	case "SHIFT":
		return aeShift{}, nil
	}
}

type aeLeftArc struct {
	relation string
}

func (l aeLeftArc) IsPossible(c Configuration) bool {
	stackSize := len(c.Stack)

	// We need a non-empty stack and buffer, plus root should not be the
	// tip of the stack.
	if stackSize == 0 || len(c.Buffer) == 0 || c.Stack[stackSize-1] == 0 {
		return false
	}

	// Assure that we are not giving the tip of the stack two heads.
	_, headPresent := c.Head(c.Stack[stackSize-1])
	return !headPresent
}

func (s aeLeftArc) Apply(c *Configuration) {
	stackSize := len(c.Stack)
	head := c.Buffer[0]
	dependant := c.Stack[stackSize-1]
	dependency := Dependency{head, s.relation, dependant}

	c.AddDependency(&dependency)
	c.Stack = c.Stack[:stackSize-1]
}

type aeRightArc struct {
	relation string
}

func (r aeRightArc) IsPossible(c Configuration) bool {
	return len(c.Stack) != 0 && len(c.Buffer) != 0
}

func (r aeRightArc) Apply(c *Configuration) {
	stackSize := len(c.Stack)
	dependant := c.Buffer[0]
	head := c.Stack[stackSize-1]
	dependency := Dependency{head, r.relation, dependant}

	c.AddDependency(&dependency)

	c.Stack = append(c.Stack, dependant)
	c.Buffer = c.Buffer[1:]
}

type aeReduce struct{}

func (r aeReduce) IsPossible(c Configuration) bool {
	stackSize := len(c.Stack)
	if stackSize == 0 {
		return false
	}

	_, headPresent := c.Head(c.Stack[stackSize-1])
	return headPresent
}

func (s aeReduce) Apply(c *Configuration) {
	stackSize := len(c.Stack)
	c.Stack = c.Stack[:stackSize-1]
}

type aeShift struct{}

func (s aeShift) IsPossible(c Configuration) bool {
	return len(c.Buffer) != 0
}

func (s aeShift) Apply(c *Configuration) {
	token := c.Buffer[0]
	c.Buffer = c.Buffer[1:]
	c.Stack = append(c.Stack, token)
}
