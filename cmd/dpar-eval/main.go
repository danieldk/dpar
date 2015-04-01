// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package main

import (
	"bufio"
	"flag"
	"fmt"
	"hash/fnv"
	"log"
	"os"
	"time"

	"github.com/danieldk/conllx"
	"github.com/danieldk/dpar/cmd/common"
	"github.com/danieldk/dpar/features"
	"github.com/danieldk/dpar/ml"
	"github.com/danieldk/dpar/system"
	"gopkg.in/danieldk/golinear.v1"
)

func init() {
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [options] config eval_data\n\n", os.Args[0])
		flag.PrintDefaults()
	}
}

func main() {
	flag.Parse()

	if flag.NArg() != 2 {
		flag.Usage()
		os.Exit(1)
	}

	configFile, err := os.Open(flag.Arg(0))
	common.ExitIfError(err)
	defer configFile.Close()
	config, err := common.ParseConfig(configFile)
	common.ExitIfError(err)

	generator, err := common.ReadFeatures(config.Parser.Features)
	common.ExitIfError(err)

	transitionSystem, ok := common.TransitionSystems[config.Parser.System]
	if !ok {
		log.Fatalf("Unknown transition system: %s", config.Parser.System)
	}

	labelNumberer, err := common.ReadTransitions(config.Parser.Transitions, transitionSystem)
	common.ExitIfError(err)

	model, err := golinear.LoadModel(config.Parser.Model)
	common.ExitIfError(err)

	if config.Parser.HashKernelSize == 0 {
		log.Fatal("Currently only models using a hash kernel are supported")
	} else {
		hashKernelParsing(transitionSystem, generator, model, labelNumberer,
			config.Parser.HashKernelSize)
	}

}

func hashKernelParsing(transitionSystem system.TransitionSystem,
	generator features.FeatureGenerator, model *golinear.Model,
	labelNumberer *features.LabelNumberer, hashKernelSize uint) {
	guide := ml.NewHashingSVMGuide(model, generator, *labelNumberer, fnv.New32,
		hashKernelSize)
	parser := system.NewGreedyParser(transitionSystem, guide)

	start := time.Now()
	evaluate(parser)
	elapsed := time.Since(start)
	log.Printf("Parsing took %s\n", elapsed)
}

func evaluate(parser system.Parser) {
	testFile, err := os.Open(flag.Arg(1))
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
		common.ExitIfError(err)

		deps, err := parser.Parse(s)
		common.ExitIfError(err)

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
