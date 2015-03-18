// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package main

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"hash/fnv"
	"log"
	"os"
	"time"

	"github.com/danieldk/conllx"
	"github.com/danieldk/dpar/features"
	"github.com/danieldk/dpar/ml"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

var transitionSystems = map[string]system.TransitionSystem{
	"arceager":    system.NewArcEager(),
	"arcstandard": system.NewArcStandard(),
	"stackproj":   system.NewStackProjective(),
}

type oracleConstructor func(system.DependencySet) system.Guide

var oracles = map[string]oracleConstructor{
	"arceager":    system.NewArcEagerOracle,
	"arcstandard": system.NewArcStandardOracle,
	"stackproj":   system.NewStackProjectiveOracle,
}

var hashKernelSize = flag.Uint("hashkernel", 0, "hash kernel size")
var transSystem = flag.String("system", "stackproj", "Transition system: arceager, arcstandard, or stackproj (default: stackproj)")

func main() {
	flag.Parse()

	if flag.NArg() != 4 {
		log.Fatal("Usage: eval features model transitions eval_data")
	}

	generator, err := readFeatures(flag.Arg(0))
	exitIfError(err)

	transitionSystem, ok := transitionSystems[*transSystem]
	if !ok {
		log.Fatalf("Unknown transition system: %s", *transSystem)
	}

	labelNumberer, err := readTransitions(flag.Arg(2), transitionSystem)
	exitIfError(err)

	model, err := readModel(flag.Arg(1))
	exitIfError(err)

	if *hashKernelSize == 0 {
		log.Fatal("Currently only models using a hash kernel are supported")
	} else {
		hashKernelParsing(transitionSystem, generator, model, labelNumberer)
	}

}

func exitIfError(err error) {
	if err != nil {
		log.Fatal(err.Error())
	}
}

func hashKernelParsing(transitionSystem system.TransitionSystem,
	generator features.FeatureGenerator, model *golinear.Model,
	labelNumberer *features.LabelNumberer) {
	guide := ml.NewHashingSVMGuide(model, generator, *labelNumberer, fnv.New32,
		*hashKernelSize)
	parser := system.NewGreedyParser(transitionSystem, guide)

	start := time.Now()
	evaluate(parser)
	elapsed := time.Since(start)
	log.Printf("Parsing took %s\n", elapsed)
}

func train(problem *golinear.Problem) *golinear.Model {
	param := golinear.DefaultParameters()
	param.Cost = 0.1
	//param.SolverType = golinear.NewL2RL2LossSvcDualDefault()
	param.SolverType = golinear.NewMCSVMCSDefault()
	model, err := golinear.TrainModel(param, problem)
	exitIfError(err)

	return model
}

func evaluate(parser system.Parser) {
	testFile, err := os.Open(flag.Arg(3))
	defer testFile.Close()
	if err != nil {
		panic("Cannot open training data")
	}

	testReader := conllx.NewReader(bufio.NewReader(testFile))

	total := 0
	found := 0
	count := 0

	for {
		s, err := testReader.ReadSentence()
		if err != nil {
			break
		}

		goldDeps, err := system.SentenceToDependencies(s)
		exitIfError(err)

		deps, err := parser.Parse(s)
		exitIfError(err)

		total += len(goldDeps)
		found += foundAttachments(goldDeps, deps)
		count++

		if count%100 == 0 {
			printAccuracy(found, total)
		}
	}

	printAccuracy(found, total)
}

func printAccuracy(found, total int) {
	fmt.Printf("Recovered: %d/%d (Accuracy: %.4f)\n", found, total, float64(found)/float64(total))
}

func foundAttachments(gold, test system.DependencySet) int {
	found := 0

	for dep := range gold {
		_, ok := test[dep]
		if ok {
			found++
		}
	}

	return found
}

func readFeatures(filename string) (features.FeatureGenerator, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	reader := bufio.NewReader(f)

	return features.ReadFeatureGeneratorsDefault(reader)
}

func readModel(filename string) (*golinear.Model, error) {
	return golinear.LoadModel(filename)
}

func readTransitions(filename string, ts system.TransitionSystem) (*features.LabelNumberer, error) {
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
	} else {
		return nil, err
	}
}
