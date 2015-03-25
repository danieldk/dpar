// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"fmt"

	"github.com/danieldk/conllx"
)

func SentenceToDependencies(s []conllx.Token) (DependencySet, error) {
	dependencies := make(DependencySet)

	for idx, token := range s {
		head, avail := token.Head()
		if !avail {
			return nil, fmt.Errorf("Token does not have a head: %v", token)
		}

		headRel, avail := token.HeadRel()
		if !avail {
			return nil, fmt.Errorf("Token does not have a head relation: %v", token)
		}

		dependency := Dependency{head, headRel, uint(idx) + 1}
		dependencies[dependency] = nil
	}

	return dependencies, nil
}

func SentenceToForms(s []conllx.Token) ([]string, error) {
	forms := make([]string, len(s))

	for idx, token := range s {
		form, avail := token.Form()
		if !avail {
			return nil, fmt.Errorf("Token does not have a form: %v", token)
		}

		forms[idx] = form
	}

	return forms, nil
}

func SentenceToTags(s []conllx.Token) ([]string, error) {
	forms := make([]string, len(s))

	for idx, token := range s {
		tag, avail := token.PosTag()
		if !avail {
			return nil, fmt.Errorf("Token does not have a tag: %v", token)
		}
		forms[idx] = tag
	}

	return forms, nil
}

func sentenceToFeatures(s []conllx.Token) []*conllx.Features {
	features := make([]*conllx.Features, len(s))

	for idx := range s {
		if f, avail := s[idx].Features(); avail {
			features[idx] = f
		}
	}

	return features
}
