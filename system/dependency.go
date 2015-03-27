// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

// A labeled dependency.
type Dependency struct {
	Head      uint
	Relation  string
	Dependent uint
}

// A set of labeled dependencies.
type DependencySet map[Dependency]interface{}

// Create a mapping from a dependent to its head. This method assumes
// (like the rest of dpar currently) that each token is single-headed.
func (ds DependencySet) CreateDependentHeadMapping() map[uint]Dependency {
	mapping := make(map[uint]Dependency)
	for dep := range ds {
		mapping[dep.Dependent] = dep
	}

	return mapping
}
