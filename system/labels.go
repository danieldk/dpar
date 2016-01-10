// Copyright 2015 The dpar Authors. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

package system

import (
	"bufio"
	"fmt"
	"io"
	"strings"
)

// A LabelNumberer creates a bijection between (string-based) labels
// and numbers.
type LabelNumberer struct {
	labelNumbers map[Transition]int
	labels       []Transition
}

// NewLabelNumberer creates a new LabelNumberer that is empty (it
// has no mappings yet).
func NewLabelNumberer() LabelNumberer {
	return LabelNumberer{make(map[Transition]int), make([]Transition, 0)}
}

// Number returns the (unique) number for for a label (transition).
func (l *LabelNumberer) Number(label Transition) int {
	idx, ok := l.labelNumbers[label]

	if !ok {
		idx = len(l.labelNumbers)
		l.labelNumbers[label] = idx
		l.labels = append(l.labels, label)
	}

	return idx
}

// Label returns the label (transition) for a number.
func (l LabelNumberer) Label(number int) Transition {
	return l.labels[number]
}

// Size returns the number of labels known in the bijection.
func (l LabelNumberer) Size() int {
	return len(l.labels)
}

// Read a label <-> number bijection from a Reader.
func (l *LabelNumberer) Read(reader io.Reader, serializer TransitionSerializer) error {
	var labels []Transition
	bufReader := bufio.NewReader(reader)

	eof := false
	for !eof {
		line, err := bufReader.ReadString('\n')
		if err != nil {
			if err != io.EOF {
				return err
			}

			eof = true
		}

		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if trans, err := serializer.DeserializeTransition(strings.TrimSpace(line)); err == nil {
			labels = append(labels, trans)
		} else {
			return err
		}
	}

	numbers := make(map[Transition]int)
	for idx, label := range labels {
		numbers[label] = idx
	}

	l.labels = labels
	l.labelNumbers = numbers

	return nil
}

// WriteLabelNumberer writes the bijection in a LabelNumberer to a file.
func (l *LabelNumberer) WriteLabelNumberer(writer io.Writer, serializer TransitionSerializer) error {
	for _, trans := range l.labels {
		if transStr, err := serializer.SerializeTransition(trans); err == nil {
			fmt.Fprintf(writer, "%s\n", transStr)
		} else {
			return err
		}
	}

	return nil
}
