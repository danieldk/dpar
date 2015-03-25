// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//go:generate ragel -Z parse_addr.rl

package features

import (
	"bytes"
	"encoding/binary"
	"hash"
	"strconv"

	"github.com/danieldk/dpar/system"
)

type Source int

const (
	STACK Source = iota
	BUFFER
	LDEP
	RDEP
)

type Layer int

const (
	TOKEN Layer = iota
	TAG
	DEPREL
	FEATURE
)

type AddressComponent struct {
	Source Source
	Index  uint
}

type AddressedValue struct {
	Address  []AddressComponent
	Layer    Layer
	LayerArg string
	Value    string
}

func (a AddressedValue) appendAddress(buf *bytes.Buffer) {
	buf.WriteString("addr(")
	for idx, component := range a.Address {
		if idx != 0 {
			buf.WriteByte(',')
		}

		buf.WriteString(strconv.FormatUint(uint64(component.Source), 10))
		buf.WriteByte('-')
		buf.WriteString(strconv.FormatUint(uint64(component.Index), 10))

	}
	buf.WriteByte(')')
}

func (a AddressedValue) AppendFeature(buf *bytes.Buffer) {
	buf.WriteString("av(")
	a.appendAddress(buf)
	buf.WriteByte(',')
	buf.WriteString(strconv.FormatUint(uint64(a.Layer), 10))
	if a.Layer == FEATURE {
		buf.WriteByte(':')
		buf.WriteString(a.LayerArg)
	}
	buf.WriteString(a.Value)
	buf.WriteByte(')')
}

func (a AddressedValue) AppendHash(hash hash.Hash) {
	buf := make([]byte, 8)

	for _, component := range a.Address {
		binary.LittleEndian.PutUint64(buf, uint64(component.Source))
		hash.Write(buf)
		binary.LittleEndian.PutUint64(buf, uint64(component.Index))
		hash.Write(buf)
	}

	binary.LittleEndian.PutUint64(buf, uint64(a.Layer))
	if a.Layer == FEATURE {
		hash.Write([]byte(a.LayerArg))
	}

	hash.Write(buf)
	hash.Write([]byte(a.Value))
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
		} else {
			return "", false
		}
	case FEATURE:
		if c.Features[token] == nil {
			return "", false
		}

		if val, ok := c.Features[token].FeaturesMap()[a.LayerArg]; ok {
			return val, true
		}

		return "", false
	default:
		panic("Layer not implemented.")
	}

}
