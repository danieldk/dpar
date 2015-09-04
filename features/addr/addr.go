// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//go:generate ragel -Z parse_addr.rl

package addr

import "github.com/danieldk/dpar/system"

// Sources of tokens in the parser configuration. LDEP/RDEP
// are used to address from the left-most/right-most dependency.
type Source int

const (
	STACK Source = iota
	BUFFER
	LDEP
	RDEP
)

// Information layers in the parser configuration.
type Layer int

const (
	TOKEN Layer = iota
	TAG
	DEPREL
	FEATURE
	CHAR
)

const LAST_LAYER = CHAR

const N_LAYERS = LAST_LAYER + 1

// A component of an adress used to point to tokens in
// a configuration. See AddressedValue.
type AddressComponent struct {
	Source Source
	Index  uint
}

// To create features, we need to address value in the parser
// configuration. The purpose of AddressedValue is two-fold:
// (1) as an address specifier and (2) to obtain the value at
// that address in the current parser state.
//
// If we wanted to get the part-of-speech tag of the leftmost
// dependent of the first token on the stack, we would use the
// following instance:
//
//   AddressedValue{
//       Address: []AddressComponent{
//           AddressComponent{TOKEN, 0},
//           AddressComponent{LDEP, 0},
//       },
//       Layer: TAG,
//   }
//
// The LayerArg field is currently only used for the FEATURE
// layer to obtain a specific feature.
type AddressedValue struct {
	Address      []AddressComponent
	Layer        Layer
	LayerArg     string
	LayerInt0Arg int
	LayerInt1Arg int
}

func (a AddressedValue) Get(c *system.Configuration) (string, bool) {
	var token uint

	for idx, component := range a.Address {
		switch component.Source {
		case STACK:
			stackSize := uint(len(c.Stack))

			// Not addressable
			if stackSize <= component.Index {
				return "", false
			}

			token = c.Stack[stackSize-component.Index-1]

		case BUFFER:
			// Not addressable
			if uint(len(c.Buffer)) <= component.Index {
				return "", false
			}

			token = c.Buffer[component.Index]

		case LDEP:
			if idx == 0 {
				panic("LDEP cannot be an initial address component")
			}

			var ok bool
			token, ok = c.LeftmostDependent(token, component.Index)
			if !ok {
				return "", false
			}

		case RDEP:
			if idx == 0 {
				panic("RDEP cannot be an initial address component")
			}

			var ok bool
			token, ok = c.RightmostDependent(token, component.Index)
			if !ok {
				return "", false
			}

		default:
			panic("Source unimplemented")
		}
	}

	switch a.Layer {
	case TOKEN:
		return c.Tokens[token], true
	case TAG:
		return c.Tags[token], true
	case DEPREL:
		if depRel, ok := c.Head(token); ok {
			return depRel.Relation, true
		}

		return "", false
	case FEATURE:
		if c.Features[token] == nil {
			return "", false
		}

		if a.LayerArg == "*" {
			return c.Features[token].FeaturesString(), true
		}

		if val, ok := c.Features[token].FeaturesMap()[a.LayerArg]; ok {
			return val, true
		}

		return "", false
	case CHAR:
		tokenRunes := []rune(c.Tokens[token])
		maxRunes := a.LayerInt0Arg + a.LayerInt1Arg

		// If the prefix length is 3 Suffix Length is 4, we want to encode 'zu' as:
		//
		// 'z' 'u' 0 0 0 'z' 'u'
		affixRunes := make([]rune, maxRunes)
		copy(affixRunes, tokenRunes[:min(len(tokenRunes), a.LayerInt0Arg)])
		copy(affixRunes[len(affixRunes)-min(a.LayerInt1Arg, len(tokenRunes)):],
			tokenRunes[max(0, len(tokenRunes)-a.LayerInt1Arg):])

		return string(affixRunes), true
	default:
		panic("Layer not implemented.")
	}

}

// Really? Yeah, really :(
func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
