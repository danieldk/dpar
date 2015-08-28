// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package symbolic

import (
	"bytes"

	"github.com/danieldk/dpar/features/addr"
	"github.com/danieldk/dpar/system"
)

var _ Feature = AddressedValueFeature{}

// A feature consisting of one or more addressed values.
type AddressedValueFeature struct {
	addressedValues []addr.AddressedValue
}

func NewAddressedValueFeature(avs []addr.AddressedValue) AddressedValueFeature {
	return AddressedValueFeature{avs}
}

// Append the feature hash to the provided hash function.
func (f AddressedValueFeature) Hash(hf FeatureHashFun) uint32 {
	h := hf()
	h.Write([]byte("avf"))

	for _, av := range f.addressedValues {
		HashableAddressedValue(av).appendHash(h)
	}

	return h.Sum32()
}

func (f AddressedValueFeature) String() string {
	buffer := new(bytes.Buffer)
	buffer.Grow(10 + 20*len(f.addressedValues))
	buffer.WriteString("avf(")
	for idx, av := range f.addressedValues {
		if idx != 0 {
			buffer.WriteRune(',')
		}
		AppendableAddressedValue(av).appendFeature(buffer)
	}
	buffer.WriteByte(')')

	return buffer.String()
}

var _ FeatureGenerator = AddressedValueGenerator{}

// A feature generator that creates AddressedValues.
type AddressedValueGenerator struct {
	templates []addr.AddressedValue
}

// Constuct a AdressedValueGenerator that uses the provided
// addressed values as templates. This means that the addressing
// will be used, but values in the template will be ignored.
func NewAddressedValueGenerator(templates []addr.AddressedValue) AddressedValueGenerator {
	return AddressedValueGenerator{templates}
}

func (g AddressedValueGenerator) Generate(c *system.Configuration) FeatureSet {
	genFeatures := make(FeatureSet)
	addressedValues := make([]addr.AddressedValue, len(g.templates))

	for idx, template := range g.templates {
		value, ok := template.Get(c)
		if !ok {
			return genFeatures
		}

		addressedValues[idx] = addr.AddressedValue{template.Address, template.Layer,
			template.LayerArg, template.LayerInt0Arg, template.LayerInt1Arg, value}
	}

	genFeatures[NewAddressedValueFeature(addressedValues).String()] = 1

	return genFeatures
}

func (g AddressedValueGenerator) GenerateHashed(c *system.Configuration, hf FeatureHashFun,
	fvb FeatureVectorBuilder) {
	// We'll cheat here. Since we never have to generate the actual feature,
	// do the hashing here to avoid extra allocations.
	h := hf()
	h.Write([]byte("avf"))

	for _, template := range g.templates {
		value, ok := template.Get(c)
		if !ok {
			return
		}

		av := addr.AddressedValue{template.Address, template.Layer, template.LayerArg,
			template.LayerInt0Arg, template.LayerInt0Arg, value}
		HashableAddressedValue(av).appendHash(h)
	}

	fvb.Add(int(h.Sum32()), 1)
}
