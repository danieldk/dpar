// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package common

import (
	"bufio"
	"errors"
	"os"

	"github.com/danieldk/dpar/features/symbolic"
	"github.com/danieldk/dpar/system"
)

// ReadFeatures reads a feature specification from the specified file.
func ReadFeatures(filename string) (symbolic.FeatureGenerator, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	reader := bufio.NewReader(f)

	return symbolic.ReadFeatureGeneratorsDefault(reader)
}

// ReadTransitions reads the transitions from the specified file. Transitions are
// numbered in file order.
func ReadTransitions(filename string, ts system.TransitionSystem) (*system.LabelNumberer, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer f.Close()

	var numberer system.LabelNumberer

	serializer, ok := ts.(system.TransitionSerializer)
	if !ok {
		return nil, errors.New("Transition system does not implement transition serialization")
	}

	if err := numberer.Read(f, serializer); err == nil {
		return &numberer, nil
	}

	return nil, err
}
