// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"bufio"
	"github.com/danieldk/conllx"
	"os"
	"reflect"
	"testing"
)

type oracleConstructor func(DependencySet) Guide

func testSystem(t *testing.T, ts TransitionSystem, oc oracleConstructor) {
	f, err := os.Open("../testdata/cdb-test.conll")
	if err != nil {
		t.Fatal(err)
	}

	r := conllx.NewReader(bufio.NewReader(f))

	for {
		s, err := r.ReadSentence()
		if err != nil {
			break
		}

		goldDependencies, err := SentenceToDependencies(s)
		if err != nil {
			t.Fatalf("Could not read gold dependencies: %s", err)
		}

		oracle := oc(goldDependencies)
		parser := NewGreedyParser(ts, oracle)

		dependencies, err := parser.Parse(s)
		if err != nil {
			t.Fatalf("Parsing error: %s", err)
		}

		if !reflect.DeepEqual(goldDependencies, dependencies) {
			t.Fatalf("Not equal: %v - %v", goldDependencies, dependencies)
		}
	}
}
