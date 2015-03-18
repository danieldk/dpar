// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

// A set of transitions.
type TransitionSet map[Transition]interface{}

// Check whether a transition is part a transition set.
func (ts TransitionSet) IsSetMember(transition Transition) bool {
	_, ok := ts[transition]
	return ok
}

// Transitions allow the system to move from configuration to configuration,
// until a terminal configuration is reached.
//
// The IsPossible method checks whether a transition is possible in a
// configuration. The Apply method applies the transition to the
// configuration. The Apply method should only be called when the possibility
// of following that transition has been checked.
type Transition interface {
	IsPossible(configuration Configuration) bool
	Apply(configuration *Configuration)
}

// A transition system consists of a set of transitions specific to that system.
//
// The IsTerminal method is used to check whether the configuration is a final
// configuration. The PossibleTransitions gives that transitions in the system
// that can be followed from the current configuration. Note that these are
// archetype transitions. If the transition has any parameters (such as a
// dependency relation) they are initialized to a default value.
type TransitionSystem interface {
	IsTerminal(configuration Configuration) bool
	PossibleTransitions(configuration Configuration) TransitionSet
}

type TransitionSerializer interface {
	SerializeTransition(Transition) (string, error)
	DeserializeTransition(string) (Transition, error)
}
