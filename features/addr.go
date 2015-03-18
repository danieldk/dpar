// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package features

import (
	"bytes"
	"encoding/binary"
	"github.com/danieldk/dpar/system"
	"hash"
	"strconv"
)

type Source int

const (
	STACK Source = iota
	BUFFER
	STACK_LDEP
	STACK_RDEP
)

type Layer int

const (
	TOKEN Layer = iota
	TAG
	DEPREL
)

type AddressedValue struct {
	Source Source
	Index  uint
	Layer  Layer
	Value  string
}

func (a AddressedValue) AppendFeature(buf *bytes.Buffer) {
	buf.WriteString("av(")
	buf.WriteString(strconv.FormatUint(uint64(a.Source), 10))
	buf.WriteByte(',')
	buf.WriteString(strconv.FormatUint(uint64(a.Index), 10))
	buf.WriteByte(',')
	buf.WriteString(strconv.FormatUint(uint64(a.Layer), 10))
	buf.WriteString(a.Value)
	buf.WriteByte(')')
}

func (a AddressedValue) AppendHash(hash hash.Hash) {
	buf := make([]byte, 8)
	binary.LittleEndian.PutUint64(buf, uint64(a.Source))
	hash.Write(buf)
	binary.LittleEndian.PutUint64(buf, uint64(a.Index))
	hash.Write(buf)
	binary.LittleEndian.PutUint64(buf, uint64(a.Layer))
	hash.Write(buf)
	hash.Write([]byte(a.Value))
}

func (a AddressedValue) Get(c *system.Configuration) (string, bool) {
	var token uint

	if a.Source == STACK || a.Source == STACK_LDEP || a.Source == STACK_RDEP {
		stackSize := uint(len(c.Stack))

		// Not addressable
		if stackSize <= a.Index {
			return "", false
		}

		token = c.Stack[stackSize-a.Index-1]

		if a.Source == STACK_LDEP {
			var ok bool
			token, ok = c.LeftmostDependent(token)
			if !ok {
				return "", false
			}
		} else if a.Source == STACK_RDEP {
			var ok bool
			token, ok = c.RightmostDependent(token)
			if !ok {
				return "", false
			}
		}

	} else if a.Source == BUFFER {
		// Not addressable
		if uint(len(c.Buffer)) <= a.Index {
			return "", false
		}

		token = c.Buffer[a.Index]
	} else {
		panic("Source unimplemented")
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
	default:
		panic("Layer not implemented.")
	}

}
