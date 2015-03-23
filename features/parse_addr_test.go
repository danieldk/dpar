package features

import (
	"reflect"
	"testing"
)

const correctString1 = "[STACK 0] TOKEN"
const correctString2 = "[BUFFER 1] TAG"
const correctString3 = "[STACK 0, LDEP 0] DEPREL"
const correctString4 = "[STACK 0, LDEP 0] DEPREL [STACK 0, RDEP 0] DEPREL"

var correct1 = AddressedValueGenerator{
	[]AddressedValue{
		AddressedValue{
			Address: []AddressComponent{AddressComponent{STACK, 0}},
			Layer:   TOKEN,
			Value:   "",
		},
	},
}

var correct2 = AddressedValueGenerator{
	[]AddressedValue{
		AddressedValue{
			Address: []AddressComponent{AddressComponent{BUFFER, 1}},
			Layer:   TAG,
			Value:   "",
		},
	},
}

var correct3 = AddressedValueGenerator{
	[]AddressedValue{
		AddressedValue{
			Address: []AddressComponent{
				AddressComponent{STACK, 0},
				AddressComponent{LDEP, 0},
			},
			Layer: DEPREL,
			Value: "",
		},
	},
}

var correct4 = AddressedValueGenerator{
	[]AddressedValue{
		AddressedValue{
			Address: []AddressComponent{
				AddressComponent{STACK, 0},
				AddressComponent{LDEP, 0},
			},
			Layer: DEPREL,
			Value: "",
		},
		AddressedValue{
			Address: []AddressComponent{
				AddressComponent{STACK, 0},
				AddressComponent{RDEP, 0},
			},
			Layer: DEPREL,
			Value: "",
		},
	},
}

var correctCases = map[string]AddressedValueGenerator{
	correctString1: correct1,
	correctString2: correct2,
	correctString3: correct3,
	correctString4: correct4,
}

var incorrectCases = []string{
	"[] TOKEN",
	"[STACK 0,] TOKEN",
	"[DRAWER 0] TOKEN",
	"[STACK 0] CANDY",
	",[STACK 0] TOKEN",
	"STACK 0 TOKEN",
	"[STACK 0 TOKEN",
	"STACK 0] TOKEN",
	"[STACK 0]",
	"[STACK 0] TOKEN TAG",
	"[STACK 0] TOKEN [STACK 1]",
}

func TestCorrect(t *testing.T) {
	for correctCase, correct := range correctCases {
		gen, err := parseAddressedValueGenerator([]byte(correctCase))
		checkCorrect(t, err, correct, gen)
	}
}

func TestIncorrect(t *testing.T) {
	for _, incorrectCase := range incorrectCases {
		_, err := parseAddressedValueGenerator([]byte(incorrectCase))
		checkIncorrect(t, err, incorrectCase)
	}
}

func checkCorrect(t *testing.T, err error, correct, candidate FeatureGenerator) {
	if err != nil {
		t.Errorf("Unexpected error: %s", err.Error())
	} else if !check(correct, candidate) {
		t.Errorf("Correct: %v, got: %v", correct, candidate)
	}
}

func checkIncorrect(t *testing.T, err error, fragment string) {
	if err == nil {
		t.Errorf("Test case should have been rejected: %s", fragment)
	}
}

func check(correct, candidate FeatureGenerator) bool {
	return reflect.DeepEqual(correct, candidate)
}
