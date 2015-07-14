// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package symbolic

import (
	"bytes"
	"encoding/binary"
	"hash"
	"strconv"

	"github.com/danieldk/dpar/features/addr"
)

type AppendableAddressedValue addr.AddressedValue

func (a AppendableAddressedValue) appendAddress(buf *bytes.Buffer) {
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

func (a AppendableAddressedValue) appendFeature(buf *bytes.Buffer) {
	buf.WriteString("av(")
	a.appendAddress(buf)
	buf.WriteByte(',')
	buf.WriteString(strconv.FormatUint(uint64(a.Layer), 10))
	if a.Layer == addr.FEATURE {
		buf.WriteByte(':')
		buf.WriteString(a.LayerArg)
	}
	buf.WriteString(a.Value)
	buf.WriteByte(')')
}

type HashableAddressedValue addr.AddressedValue

func (a HashableAddressedValue) appendHash(hash hash.Hash) {
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

	hash.Write(buf)
	hash.Write([]byte(a.Value))
}
