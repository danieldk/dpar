// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"fmt"

	"github.com/danieldk/conllx"
)

// SentenceToDependencies extracts the dependencies in a given
// CoNLL-X sentence and stores them in a set.
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

// SentenceToForms extracts the forms from a given CoNLL-X sentence in
// sentence order.
func SentenceToForms(s []conllx.Token) ([]string, error) {
	forms := make([]string, len(s))

	for idx, token := range s {
		form, avail := token.Form()
		if !avail {
			// XXX: CoNLL-X annoyance. Not sure wether the reader should be
			//      smarter or we should handle such cases ad-hoc (as here).
			forms[idx] = "_"
		} else {
			forms[idx] = form
		}
	}

	return forms, nil
}

// SentenceToTags extracts the tags from a given CoNLL-X sentence in
// sentence order.
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
