// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package features

import (
	"bytes"

	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

var _ Feature = AddressedValueFeature{}

// A feature consisting of one or more addressed values.
type AddressedValueFeature struct {
	addressedValues []AddressedValue
}

func NewAddressedValueFeature(avs []AddressedValue) AddressedValueFeature {
	return AddressedValueFeature{avs}
}

// Append the feature hash to the provided hash function.
func (f AddressedValueFeature) Hash(hf FeatureHashFun) uint32 {
	h := hf()
	h.Write([]byte("avf"))

	for _, av := range f.addressedValues {
		av.appendHash(h)
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
		av.appendFeature(buffer)
	}
	buffer.WriteByte(')')

	return buffer.String()
}

var _ FeatureGenerator = AddressedValueGenerator{}

// A feature generator that creates AddressedValueFeatures.
type AddressedValueGenerator struct {
	templates []AddressedValue
}

// Constuct a AdressedValueGenerator that uses the provided
// addressed values as templates. This means that the addressing
// will be used, but values in the template will be ignored.
func NewAddressedValueGenerator(templates []AddressedValue) AddressedValueGenerator {
	return AddressedValueGenerator{templates}
}

func (g AddressedValueGenerator) Generate(c *system.Configuration) FeatureSet {
	features := make(FeatureSet)
	addressedValues := make([]AddressedValue, len(g.templates))

	for idx, template := range g.templates {
		value, ok := template.Get(c)
		if !ok {
			return features
		}

		addressedValues[idx] = AddressedValue{template.Address, template.Layer,
			template.LayerArg, value}
	}

	features[NewAddressedValueFeature(addressedValues).String()] = 1

	return features
}

func (g AddressedValueGenerator) GenerateHashed(c *system.Configuration, hf FeatureHashFun,
	fvb *FeatureVectorBuilder) {
	// We'll cheat here. Since we never have to generate the actual feature,
	// do the hashing here to avoid extra allocations.
	h := hf()
	h.Write([]byte("avf"))

	for _, template := range g.templates {
		value, ok := template.Get(c)
		if !ok {
			return
		}

		av := AddressedValue{template.Address, template.Layer, template.LayerArg, value}
		av.appendHash(h)
	}

	fvb.Add(golinear.FeatureValue{int(h.Sum32()), 1})
}
