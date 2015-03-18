// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"github.com/danieldk/conllx"
	"sort"
)

type Configuration struct {
	Tokens     []string
	Tags       []string
	Stack      []uint
	Buffer     []uint
	tokenHeads []*Dependency
	headDeps   [][]uint
}

func NewConfiguration(tokens []conllx.Token) (Configuration, error) {
	forms, err := SentenceToForms(tokens)
	if err != nil {
		return Configuration{}, err
	}

	tags, err := SentenceToTags(tokens)
	if err != nil {
		return Configuration{}, err
	}

	stack := []uint{0}
	buffer := make([]uint, len(tokens), len(tokens))
	tokenHeads := make([]*Dependency, len(tokens)+1)
	headDeps := make([][]uint, len(tokens)+1)

	for i := 0; i < len(tokens); i++ {
		buffer[i] = uint(i + 1)
	}

	for i := 0; i < len(headDeps); i++ {
		headDeps[i] = make([]uint, 0, 3)
	}

	rootedTokens := make([]string, len(tokens)+1)
	rootedTokens[0] = "ROOT"
	copy(rootedTokens[1:], forms)

	rootedTags := make([]string, len(tags)+1)
	rootedTags[0] = "ROOT"
	copy(rootedTags[1:], tags)

	return Configuration{rootedTokens, rootedTags, stack, buffer,
		tokenHeads, headDeps}, nil
}

func (c *Configuration) AddDependency(d *Dependency) {
	c.tokenHeads[d.Dependent] = d

	deps := c.headDeps[d.Head]
	ip := sort.Search(len(deps), func(i int) bool { return deps[i] > d.Dependent })
	c.headDeps[d.Head] = insert(deps, ip, d.Dependent)
}

func (c *Configuration) Dependencies() DependencySet {
	ds := make(DependencySet)

	for _, dependency := range c.tokenHeads {
		if dependency != nil {
			ds[*dependency] = nil
		}
	}

	return ds
}

// Get the head of a token. If a token does not have a head
// attached yet, the second value of the return tuple is false.
func (c *Configuration) Head(token uint) (Dependency, bool) {
	dep := c.tokenHeads[token]
	if dep == nil {
		return Dependency{}, false
	}

	return *dep, true
}

func insert(slice []uint, index int, value uint) []uint {
	slice = append(slice, 0)
	copy(slice[index+1:], slice[index:])
	slice[index] = value
	return slice
}

// Get the token that is the leftmost dependent of the given token. If no
// such token exists, false is returned as the second value.
func (c *Configuration) LeftmostDependent(head uint) (uint, bool) {
	deps := c.headDeps[head]
	if len(deps) == 0 {
		return ^uint(0), false
	}

	return deps[0], true
}

// Get the token that is the rightmost dependent of the given token. If no
// such token exists, false is returned as the second value.
func (c *Configuration) RightmostDependent(head uint) (uint, bool) {
	deps := c.headDeps[head]
	if len(deps) == 0 {
		return 0, false
	}

	return deps[len(deps)-1], true
}
