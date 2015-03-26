// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package common

import (
	"io"

	"github.com/BurntSushi/toml"
	"github.com/danieldk/dpar/system"
)

type DParConfig struct {
	Parser Parser
}

type Parser struct {
	System         string
	Features       string
	Model          string
	Transitions    string
	HashKernelSize uint `toml:"hash_kernel_size"`
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
	}
}

func ParseConfig(reader io.Reader) (*DParConfig, error) {
	config := defaultConfiguration()
	if _, err := toml.DecodeReader(reader, config); err == nil {
		return config, nil
	} else {
		return config, err
	}
}

var TransitionSystems = map[string]system.TransitionSystem{
	"arceager":    system.NewArcEager(),
	"arcstandard": system.NewArcStandard(),
	"stackproj":   system.NewStackProjective(),
}

type OracleConstructor func(system.DependencySet) system.Guide

var Oracles = map[string]OracleConstructor{
	"arceager":    system.NewArcEagerOracle,
	"arcstandard": system.NewArcStandardOracle,
	"stackproj":   system.NewStackProjectiveOracle,
}
