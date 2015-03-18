// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package features

import (
	"bytes"
	"errors"
	"fmt"
	"strconv"

	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

type AddressedValueFeature struct {
	addressedValues []AddressedValue
}

// Append the feature hash to the provided hash function.
func (f AddressedValueFeature) Hash(hf FeatureHashFun) uint32 {
	h := hf()
	h.Write([]byte("avf"))

	for _, av := range f.addressedValues {
		av.AppendHash(h)
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
		av.AppendFeature(buffer)
	}
	buffer.WriteByte(')')

	return buffer.String()
}

func NewAddressedValueFeature(avs []AddressedValue) Feature {
	return AddressedValueFeature{avs}
}

type AddressedValueGenerator struct {
	templates []AddressedValue
}

func NewAddressedValueGenerator(templates []AddressedValue) FeatureGenerator {
	return AddressedValueGenerator{templates}
}

func parseAddressedValueGenerator(params []string) (FeatureGenerator, error) {
	if len(params)%3 != 0 {
		return nil, errors.New("AddressedValueGenerator definition should have multiple of 3 columns")
	}

	templates := make([]AddressedValue, 0, len(params)/3)

	for len(params) != 0 {
		var source Source
		switch params[0] {
		case "STACK":
			source = STACK
		case "BUFFER":
			source = BUFFER
		case "STACK_LDEP":
			source = STACK_LDEP
		case "STACK_RDEP":
			source = STACK_RDEP
		default:
			return nil, fmt.Errorf("Unknown source: %s", params[0])
		}

		depth, err := strconv.ParseUint(params[1], 10, 32)
		if err != nil {
			return nil, err
		}

		var layer Layer
		switch params[2] {
		case "TOKEN":
			layer = TOKEN
		case "TAG":
			layer = TAG
		case "DEPREL":
			layer = DEPREL
		default:
			return nil, fmt.Errorf("Unknown layer: %s", params[0])
		}

		templates = append(templates, AddressedValue{source, uint(depth), layer, ""})

		params = params[3:]
	}

	return AddressedValueGenerator{templates}, nil
}

func (g AddressedValueGenerator) Generate(c *system.Configuration) FeatureSet {
	features := make(FeatureSet)
	addressedValues := make([]AddressedValue, len(g.templates))

	for idx, template := range g.templates {
		value, ok := template.Get(c)
		if !ok {
			return features
		}

		addressedValues[idx] = AddressedValue{template.Source, template.Index,
			template.Layer, value}
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

		av := AddressedValue{template.Source, template.Index, template.Layer, value}
		av.AppendHash(h)
	}

	fvb.Add(golinear.FeatureValue{int(h.Sum32()), 1})
}
