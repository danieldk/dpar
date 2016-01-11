package addr

import (
	"reflect"
	"testing"

	"github.com/danieldk/dpar/system"
)

var charTestConfiguration = &system.Configuration{
	Stack:  []uint{0},
	Tokens: []string{"Orientierungswoche"},
}

var shortCharTestConfiguration = &system.Configuration{
	Stack:  []uint{0},
	Tokens: []string{"zu"},
}

var shortCharTestCorrect = []rune{122, 117, 0, 0, 0, 122, 117}
var charTestAddr = AddressedValue{
	Address:      []AddressComponent{AddressComponent{STACK, 0}},
	Layer:        CHAR,
	LayerInt0Arg: 3,
	LayerInt1Arg: 4,
}

func TestChar(t *testing.T) {
	value, _ := charTestAddr.Get(charTestConfiguration)
	if value != "Orioche" {
		t.Errorf("Expected value 'Orioche', got: '%s'", value)
	}

	value, _ = charTestAddr.Get(shortCharTestConfiguration)
	if !reflect.DeepEqual([]rune(value), shortCharTestCorrect) {
		t.Errorf("Expected value %v, got: %v", shortCharTestCorrect, []rune(value))
	}
}
