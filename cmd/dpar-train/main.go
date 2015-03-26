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
	"sort"

	"github.com/danieldk/conllx"
	"github.com/danieldk/dpar/cmd/common"
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

var libsvmOutput = flag.String("svmoutput", "", "Dump training data in liblinear/libsvm format")

func main() {
	flag.Parse()

	if flag.NArg() != 2 {
		log.Fatal("Usage: train config train_data")
	}

	configFile, err := os.Open(flag.Arg(0))
	exitIfError(err)
	defer configFile.Close()
	config, err := common.ParseConfig(configFile)
	exitIfError(err)

	log.Printf("Transition system: %s", config.Parser.System)
	if config.Parser.HashKernelSize > 0 {
		log.Printf("Hash kernel size: %d", config.Parser.HashKernelSize)
	}

	featureFile, err := os.Open(config.Parser.Features)
	exitIfError(err)
	defer featureFile.Close()

	generator, err := features.ReadFeatureGeneratorsDefault(bufio.NewReader(featureFile))
	exitIfError(err)

	transitionSystem, ok := transitionSystems[config.Parser.System]
	if !ok {
		log.Fatalf("Unknown transition system: %s", config.Parser.System)
	}

	oracleConstructor, ok := oracles[config.Parser.System]
	if !ok {
		log.Fatalf("Unknown transition system: %s", config.Parser.System)
	}

	log.Println("Creating training instances...")
	var collector ml.GoLinearCollector
	if config.Parser.HashKernelSize == 0 {
		collector = featureParsing(transitionSystem, generator, oracleConstructor)
	} else {
		collector = hashKernelParsing(transitionSystem, generator, oracleConstructor,
			config.Parser.HashKernelSize)
	}

	if *libsvmOutput != "" {
		writeLibSVMOutput(collector.Problem())
	}

	log.Println("Training classifier...")
	if config.Parser.Model != "" {
		model := train(collector.Problem())
		err := model.Save(config.Parser.Model)
		exitIfError(err)
	}

	writeTransitions(transitionSystem, collector, config.Parser.Transitions)

	log.Println("Done!")
}

func exitIfError(err error) {
	if err != nil {
		log.Fatal(err.Error())
	}
}

func featureParsing(transitionSystem system.TransitionSystem,
	generator features.FeatureGenerator, oracleConstructor oracleConstructor) ml.GoLinearCollector {
	collector := ml.NewFeatureCollector(generator)
	trainer := ml.NewGreedyTrainer(transitionSystem, collector)
	createTrainingInstances(trainer, collector, oracleConstructor)

	return collector
}

func hashKernelParsing(transitionSystem system.TransitionSystem,
	generator features.FeatureGenerator, oracleConstructor oracleConstructor,
	hashKernelSize uint) ml.GoLinearCollector {
	collector := ml.NewHashCollector(generator, fnv.New32, hashKernelSize)
	trainer := ml.NewGreedyTrainer(transitionSystem, collector)
	createTrainingInstances(trainer, collector, oracleConstructor)

	return collector
}

func train(problem *golinear.Problem) *golinear.Model {
	param := golinear.DefaultParameters()
	param.Cost = 0.1
	param.SolverType = golinear.NewMCSVMCSDefault()
	model, err := golinear.TrainModel(param, problem)
	exitIfError(err)

	return model
}

func writeLibSVMOutput(problem *golinear.Problem) {
	f, err := os.Create(*libsvmOutput)
	exitIfError(err)
	defer f.Close()

	problem.Iterate(func(instance *golinear.TrainingInstance) bool {
		fmt.Fprintf(f, "%.0f", instance.Label)

		for _, fv := range instance.Features {
			fmt.Fprintf(f, " %d:%f", fv.Index, fv.Value)
		}

		fmt.Fprintln(f)

		return true
	})
}

func writeTransitions(ts system.TransitionSystem, collector ml.InstanceCollector,
	transitionsFilename string) {
	serializer, ok := ts.(system.TransitionSerializer)
	if !ok {
		log.Fatal("Transition system does not implement transition serialization")
	}

	f, err := os.Create(transitionsFilename)
	exitIfError(err)
	defer f.Close()

	err = collector.LabelNumberer().WriteLabelNumberer(f, serializer)
	exitIfError(err)
}

func sortedFeatureVector(fv golinear.FeatureVector) golinear.FeatureVector {
	sorted := make(golinear.FeatureVector, len(fv))
	copy(sorted, fv)

	sort.Sort(byIndex{sorted})

	return sorted
}

type byIndex struct{ golinear.FeatureVector }

func (fv byIndex) Len() int {
	return len(fv.FeatureVector)
}

func (fv byIndex) Swap(i, j int) {
	fv.FeatureVector[i], fv.FeatureVector[j] =
		fv.FeatureVector[j], fv.FeatureVector[i]
}

func (fv byIndex) Less(i, j int) bool {
	return fv.FeatureVector[i].Index < fv.FeatureVector[j].Index
}

func createTrainingInstances(trainer ml.GreedyTrainer, collector ml.InstanceCollector,
	oracleConstructor oracleConstructor) {
	f, err := os.Open(flag.Arg(1))
	defer f.Close()
	if err != nil {
		panic("Cannot open training data")
	}

	r := conllx.NewReader(bufio.NewReader(f))

	for {
		s, err := r.ReadSentence()
		if err != nil {
			break
		}

		goldDependencies, err := system.SentenceToDependencies(s)
		exitIfError(err)

		oracle := oracleConstructor(goldDependencies)
		trainer.Parse(s, oracle)
	}

}
