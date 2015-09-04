// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package symbolic

import (
	"encoding/binary"
	"hash"

	"github.com/danieldk/dpar/features/addr"
	"github.com/danieldk/dpar/system"
)

type HashableAddressedValue struct {
	addr.AddressedValue
}

func (a HashableAddressedValue) appendHash(c *system.Configuration, hash hash.Hash) bool {
	value, ok := a.Get(c)
	if !ok {
		return false
	}

	buf := make([]byte, 8)

	for _, component := range a.Address {
		binary.LittleEndian.PutUint64(buf, uint64(component.Source))
		hash.Write(buf)
		binary.LittleEndian.PutUint64(buf, uint64(component.Index))
		hash.Write(buf)
	}

	binary.LittleEndian.PutUint64(buf, uint64(a.Layer))
	if a.Layer == addr.FEATURE {
		hash.Write([]byte(a.LayerArg))
	}

	if a.Layer == addr.CHAR {
		binary.LittleEndian.PutUint64(buf, uint64(a.LayerInt0Arg))
		hash.Write(buf)
		binary.LittleEndian.PutUint64(buf, uint64(a.LayerInt1Arg))
		hash.Write(buf)
	}

	hash.Write(buf)
	hash.Write([]byte(value))

	return true
}
