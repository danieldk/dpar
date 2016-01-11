// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"errors"
	"fmt"
	"strings"
)

// Archetype transitions + interface validation.
var archetypeAEShift = aeShift{}
var archetypeAEReduce = aeReduce{}
var archetypeAELeftArc = aeLeftArc{"<archetype>"}
var archeTypeAERightArc = aeRightArc{"<archetype>"}

// Assert TransitionSystem/TransitionSerializer conformance.
var _ TransitionSystem = NewArcEager()
var _ TransitionSerializer = NewArcEager()

// The ArcEager transition system.
//
// See: Joakim Nivre, Incrementality in Deterministic Dependency Parsing, 2004
type ArcEager struct {
	archetypeTransitions TransitionSet
}

// NewArcEager creates a new arc-eager transition system.
func NewArcEager() *ArcEager {
	trans := map[Transition]interface{}{
		archetypeAEShift:    nil,
		archetypeAEReduce:   nil,
		archetypeAELeftArc:  nil,
		archeTypeAERightArc: nil,
	}

	return &ArcEager{trans}
}

// IsTerminal checks whether a parser configuration is a terminal
// configuration for the system. When this is the case, no further
// transitions should be attempted.
func (ts *ArcEager) IsTerminal(c Configuration) bool {
	return len(c.Buffer) == 0
}

// PossibleTransitions returns the set of possible transitions in a
// particular parser configuration. The returned transitions are
// archetypal transitions (they do not parametrized over a dependency
// relation).
func (ts *ArcEager) PossibleTransitions(configuration Configuration) TransitionSet {
	possible := make(TransitionSet)

	for trans := range ts.archetypeTransitions {
		if trans.IsPossible(configuration) {
			possible[trans] = nil
		}
	}

	return possible
}

// SerializeTransition serializes a transition of the arc-eager
// system to a string. An error is returned when it is attempted to
// serialize a transition of another system.
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

// DeserializeTransition deserializes a transition from the arc-eager system.
// If the provided transition is not known to the system, an error is returned.
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

func (l aeLeftArc) Apply(c *Configuration) {
	stackSize := len(c.Stack)
	head := c.Buffer[0]
	dependent := c.Stack[stackSize-1]
	dependency := Dependency{head, l.relation, dependent}

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
	dependent := c.Buffer[0]
	head := c.Stack[stackSize-1]
	dependency := Dependency{head, r.relation, dependent}

	c.AddDependency(&dependency)

	c.Stack = append(c.Stack, dependent)
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

func (r aeReduce) Apply(c *Configuration) {
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
