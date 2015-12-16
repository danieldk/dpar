// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package common

import (
	"io"

	"github.com/BurntSushi/toml"
	"github.com/danieldk/dpar/system"
)

// DParConfig stores the configuration of a dpar command-line parser.
type DParConfig struct {
	Parser    Parser
	LibLinear LibLinear
}

// Parser store the parser configuration. The configuration consts
// of a string indicating the transition system ('arcstandard', 'arceager',
// or 'stackproj'); paths to the feature specification, model, and transition
// files; and the size of the hash kernel.
type Parser struct {
	System         string
	Features       string
	Model          string
	Transitions    string
	HashKernelSize uint `toml:"hash_kernel_size"`
}

// LibLinear stores the configuration for golinear/liblinear. At the moment,
// the only configurable parameter is the constraint violation cost.
type LibLinear struct {
	Cost float64
}

func defaultConfiguration() *DParConfig {
	return &DParConfig{
		Parser: Parser{
			System:         "stackproj",
			Features:       "parser.features",
			Model:          "parser.model",
			Transitions:    "parser.transitions",
			HashKernelSize: 300000,
		},
		LibLinear: LibLinear{
			Cost: 0.1,
		},
	}
}

// ParseConfig attempts to parse the configuration from the given reader.
func ParseConfig(reader io.Reader) (*DParConfig, error) {
	config := defaultConfiguration()
	if _, err := toml.DecodeReader(reader, config); err != nil {
		return config, err
	}

	return config, nil
}

// TransitionSystems is a mapping from the known transition system names
// to instances of those transition systems.
var TransitionSystems = map[string]system.TransitionSystem{
	"arceager":    system.NewArcEager(),
	"arcstandard": system.NewArcStandard(),
	"stackproj":   system.NewStackProjective(),
	"stackswap":   system.NewStackSwap(),
}

// An OracleConstructor constructs an oracle for a set of gold standard
// depdendencies.
type OracleConstructor func(system.DependencySet) system.Guide

// Oracles provides a mapping of known transition systems (also see
// TransitionSystems) and their oracle constructors.
var Oracles = map[string]OracleConstructor{
	"arceager":    system.NewArcEagerOracle,
	"arcstandard": system.NewArcStandardOracle,
	"stackproj":   system.NewStackProjectiveOracle,
	"stackswap":   system.NewStackSwapOracle,
}
