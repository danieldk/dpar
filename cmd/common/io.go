// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package common

import (
	"bufio"
	"errors"
	"os"

	"github.com/danieldk/dpar/features"
	"github.com/danieldk/dpar/system"
)

func ReadFeatures(filename string) (features.FeatureGenerator, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	reader := bufio.NewReader(f)

	return features.ReadFeatureGeneratorsDefault(reader)
}

func ReadTransitions(filename string, ts system.TransitionSystem) (*features.LabelNumberer, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	defer f.Close()

	var numberer features.LabelNumberer

	serializer, ok := ts.(system.TransitionSerializer)
	if !ok {
		return nil, errors.New("Transition system does not implement transition serialization")
	}

	if err := numberer.Read(f, serializer); err == nil {
		return &numberer, nil
	}

	return nil, err
}
