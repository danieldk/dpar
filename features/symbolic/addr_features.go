// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package symbolic

import (
	"github.com/danieldk/dpar/features/addr"
	"github.com/danieldk/dpar/system"
)

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

func (g AddressedValueGenerator) Generate(c *system.Configuration, hf FeatureHashFun,
	fvb FeatureVectorBuilder) {
	// We'll cheat here. Since we never have to generate the actual feature,
	// do the hashing here to avoid extra allocations.
	h := hf()
	h.Write([]byte("avf"))

	for _, av := range g.templates {
		// If the feature is not addressable, exit early.
		hav := HashableAddressedValue{av}
		if ok := hav.appendHash(c, h); !ok {
			return
		}
	}

	fvb.Add(int(h.Sum32()), 1)
}
