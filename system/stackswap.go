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
var archetypeSSWShift Transition = sswShift{}
var archetypeSSWSwap Transition = sswSwap{}
var archetypeSSWLeftArc Transition = sswLeftArc{"<archetype>"}
var archetypeSSWRightArc Transition = sswRightArc{"<archetype>"}

// Assert TransitionSystem/TransitionSerializer conformance.
var _ TransitionSystem = NewStackSwap()
var _ TransitionSerializer = NewStackSwap()

// The StackSwap transition-system is similar to the StackProjective system,
// but adds a swap transition that allows it to produce non-projective trees.
//
// This system is described in:
//
// Joakim Nivre, Non-projective dependency parsing in expected linear time, 2009
type StackSwap struct {
	archetypeTransitions TransitionSet
}

// NewStackSwap creates a stack-swap transition system.
func NewStackSwap() *StackSwap {
	trans := map[Transition]interface{}{
		archetypeSSWShift:    nil,
		archetypeSSWSwap:     nil,
		archetypeSSWLeftArc:  nil,
		archetypeSSWRightArc: nil,
	}

	return &StackSwap{trans}
}

// IsTerminal checks whether a parser configuration is a terminal
// configuration for the system. When this is the case, no further
// transitions should be attempted.
func (ts *StackSwap) IsTerminal(c Configuration) bool {
	return len(c.Stack) == 1 && len(c.Buffer) == 0
}

// PossibleTransitions returns the set of possible transitions in a
// particular parser configuration. The returned transitions are
// archetypal transitions (they do not parametrized over a dependency
// relation).
func (ts *StackSwap) PossibleTransitions(configuration Configuration) TransitionSet {
	possible := make(TransitionSet)

	for trans := range ts.archetypeTransitions {
		if trans.IsPossible(configuration) {
			possible[trans] = nil
		}
	}

	return possible
}

// SerializeTransition serializes a transition of the stack-swap
// system to a string. An error is returned when it is attempted to
// serialize a transition of another system.
func (ts *StackSwap) SerializeTransition(t Transition) (string, error) {
	switch t := t.(type) {
	case sswLeftArc:
		return fmt.Sprintf("LEFT_ARC %s", t.relation), nil
	case sswRightArc:
		return fmt.Sprintf("RIGHT_ARC %s", t.relation), nil
	case sswSwap:
		return "SWAP", nil
	case sswShift:
		return "SHIFT", nil
	default:
		return "", fmt.Errorf("Not a transition of the stack-swap system: %T", t)
	}
}

// DeserializeTransition deserializes a transition from the
// stack-swap system. If the provided transition is not known to
// the system, an error is returned.
func (ts *StackSwap) DeserializeTransition(transString string) (Transition, error) {
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
		return sswLeftArc{parts[1]}, nil
	case "RIGHT_ARC":
		if len(parts) == 1 {
			return nil, errors.New("Right-Arc transition requires label argument")
		}
		return sswRightArc{parts[1]}, nil
	case "SWAP":
		return sswSwap{}, nil
	case "SHIFT":
		return sswShift{}, nil
	}
}

type sswLeftArc struct {
	relation string
}

func (l sswLeftArc) IsPossible(c Configuration) bool {
	stackSize := len(c.Stack)
	return stackSize > 1 && c.Stack[stackSize-2] != 0
}

func (l sswLeftArc) Apply(c *Configuration) {
	stack := c.Stack

	stackSize := len(stack)
	head := stack[stackSize-1]
	dependent := stack[stackSize-2]
	dependency := Dependency{head, l.relation, dependent}

	c.AddDependency(&dependency)

	c.Stack = append(stack[:stackSize-2], head)
}

type sswRightArc struct {
	relation string
}

func (r sswRightArc) IsPossible(configuration Configuration) bool {
	return len(configuration.Stack) > 1
}

func (r sswRightArc) Apply(c *Configuration) {
	stack := c.Stack

	stackSize := len(stack)
	dependent := stack[stackSize-1]
	head := stack[stackSize-2]
	dependency := Dependency{head, r.relation, dependent}

	c.AddDependency(&dependency)

	c.Stack = stack[:stackSize-1]
}

type sswSwap struct{}

func (s sswSwap) IsPossible(c Configuration) bool {
	stack := c.Stack
	stackSize := len(stack)
	return stackSize > 1 && stack[stackSize-1] > stack[stackSize-2]
}

func (s sswSwap) Apply(c *Configuration) {
	stack := c.Stack
	stackSize := len(stack)

	// Move second stack element to the buffer...
	// Todo: exploit possible gap at the beginning of the backing array...
	newBuffer := make([]uint, len(c.Buffer)+1)
	copy(newBuffer[1:], c.Buffer)
	newBuffer[0] = stack[stackSize-2]
	c.Buffer = newBuffer

	// ... and remove that element from the stack.
	head := stack[stackSize-1]
	c.Stack = append(stack[:stackSize-2], head)
}

type sswShift struct{}

func (s sswShift) IsPossible(c Configuration) bool {
	return len(c.Buffer) != 0
}

func (s sswShift) Apply(c *Configuration) {
	if !s.IsPossible(*c) {
		panic("Tried to shift on an empty buffer.")
	}

	stack := c.Stack
	buffer := c.Buffer
	c.Stack = append(stack, buffer[0])
	c.Buffer = buffer[1:]
}

// The StackSwapOracle returns a correct transition for a particular
// configuration in the stack-swap system. The transition choice in
// an oracle is based on the gold-standard parse.
type StackSwapOracle struct {
	dependentHeadMapping map[uint]Dependency
	projectiveOrder      []int
}

// NewStackSwapOracle returns a new oracle from the stack-swap
// system. The gold dependency set is used to choose a correct transition
// from each configuration.
func NewStackSwapOracle(goldDependencies DependencySet) Guide {
	oracle := StackSwapOracle{
		dependentHeadMapping: goldDependencies.CreateDependentHeadMapping(),
		projectiveOrder:      extractProjectiveOrder(goldDependencies),
	}
	return &oracle
}

func extractProjectiveOrder(dependencies DependencySet) []int {
	g := dependenciesToGraph(dependencies)

	projectiveOrder := make([]uint, 0, len(dependencies)+1)

	dfs(g, 0, func(token int) {
		projectiveOrder = append(projectiveOrder, uint(token))
	})

	orderMapping := make([]int, len(projectiveOrder))
	for idx, token := range projectiveOrder {
		orderMapping[token] = idx
	}

	return orderMapping
}

// BestTransition returns the best transition from the current
// configuration, as inferred from the gold standard dependency
// structure.
func (o *StackSwapOracle) BestTransition(c *Configuration) Transition {
	stack := c.Stack

	if len(stack) > 1 {
		stackSize := len(stack)
		stack0 := stack[stackSize-1]
		stack1 := stack[stackSize-2]

		la := sswLeftArc{o.dependentHeadMapping[stack1].Relation}
		if la.IsPossible(*c) && o.dependentHeadMapping[stack1].Head == stack0 && !o.neededForAttachment(c, stack1) {
			return la
		}

		ra := sswRightArc{o.dependentHeadMapping[stack0].Relation}
		if ra.IsPossible(*c) &&
			o.dependentHeadMapping[stack0].Head == stack1 &&
			!o.neededForAttachment(c, stack0) {
			return ra
		}

		if o.projectiveOrder[stack0] < o.projectiveOrder[stack1] {
			return sswSwap{}
		}
	}

	return sswShift{}
}

func (o *StackSwapOracle) neededForAttachment(c *Configuration, token uint) bool {
	for _, bufToken := range c.Buffer {
		if o.dependentHeadMapping[bufToken].Head == token {
			return true
		}
	}

	for _, bufToken := range c.Stack {
		if o.dependentHeadMapping[bufToken].Head == token {
			return true
		}
	}

	return false
}
