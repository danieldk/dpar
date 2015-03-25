package features

import (
	"reflect"
	"testing"
)

const correctString1 = "[STACK 0] TOKEN"
const correctString2 = "[BUFFER 1] TAG"
const correctString3 = "[STACK 0, LDEP 0] DEPREL"
const correctString4 = "[STACK 0, LDEP 0] DEPREL [STACK 0, RDEP 0] DEPREL"
const correctString5 = "[STACK 0] FEATURE num"

var correct1 = []AddressedValue{
	AddressedValue{
		Address: []AddressComponent{AddressComponent{STACK, 0}},
		Layer:   TOKEN,
		Value:   "",
	},
}

var correct2 = []AddressedValue{
	AddressedValue{
		Address: []AddressComponent{AddressComponent{BUFFER, 1}},
		Layer:   TAG,
		Value:   "",
	},
}

var correct3 = []AddressedValue{
	AddressedValue{
		Address: []AddressComponent{
			AddressComponent{STACK, 0},
			AddressComponent{LDEP, 0},
		},
		Layer: DEPREL,
		Value: "",
	},
}

var correct4 = []AddressedValue{
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
}

var correct5 = []AddressedValue{
	AddressedValue{
		Address:  []AddressComponent{AddressComponent{STACK, 0}},
		Layer:    FEATURE,
		LayerArg: "num",
		Value:    "",
	},
}

var correctCases = map[string][]AddressedValue{
	correctString1: correct1,
	correctString2: correct2,
	correctString3: correct3,
	correctString4: correct4,
	correctString5: correct5,
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
	"[LDEP 0] TOKEN",
	"[RDEP 0] TOKEN",
	"[STACK 0] TOKEN num",
	"[STACK 0] TAG num",
	"[STACK 0] DEPREL num",
}

func TestCorrect(t *testing.T) {
	for correctCase, correct := range correctCases {
		gen, err := ParseAddressedValueTemplates([]byte(correctCase))
		checkCorrect(t, err, correct, gen)
	}
}

func TestIncorrect(t *testing.T) {
	for _, incorrectCase := range incorrectCases {
		_, err := ParseAddressedValueTemplates([]byte(incorrectCase))
		checkIncorrect(t, err, incorrectCase)
	}
}

func checkCorrect(t *testing.T, err error, correct, candidate []AddressedValue) {
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

func check(correct, candidate []AddressedValue) bool {
	return reflect.DeepEqual(correct, candidate)
}
