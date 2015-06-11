// line 1 "parse_addr.rl"
// Generated using ragel; DO NOT EDIT!

package features

// Parse addresses

import (
	"bytes"
	"errors"
	"fmt"
	"strconv"
)

// line 18 "parse_addr.go"
var _addressed_value_actions []byte = []byte{
	0, 1, 0, 1, 1, 1, 3, 2, 1,
	0, 2, 2, 5, 2, 3, 0, 2,
	3, 6, 2, 4, 6,
}

var _addressed_value_key_offsets []byte = []byte{
	0, 0, 3, 5, 6, 7, 8, 9,
	10, 14, 18, 24, 27, 33, 34, 35,
	36, 37, 38, 39, 40, 45, 46, 47,
	48, 49, 50, 51, 52, 53, 54, 55,
	56, 65, 74, 76, 77, 78, 79, 80,
	83, 86,
}

var _addressed_value_trans_keys []byte = []byte{
	9, 32, 91, 66, 83, 85, 70, 70,
	69, 82, 9, 32, 48, 57, 9, 32,
	48, 57, 9, 32, 44, 93, 48, 57,
	9, 32, 44, 9, 32, 66, 76, 82,
	83, 68, 69, 80, 84, 65, 67, 75,
	9, 32, 68, 70, 84, 69, 80, 82,
	69, 76, 69, 65, 84, 85, 82, 69,
	9, 32, 95, 48, 57, 65, 90, 97,
	122, 9, 32, 95, 48, 57, 65, 90,
	97, 122, 65, 79, 71, 75, 69, 78,
	9, 32, 91, 9, 32, 91, 9, 32,
	91, 95, 48, 57, 65, 90, 97, 122,
}

var _addressed_value_single_lengths []byte = []byte{
	0, 3, 2, 1, 1, 1, 1, 1,
	2, 2, 4, 3, 6, 1, 1, 1,
	1, 1, 1, 1, 5, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
	3, 3, 2, 1, 1, 1, 1, 3,
	3, 4,
}

var _addressed_value_range_lengths []byte = []byte{
	0, 0, 0, 0, 0, 0, 0, 0,
	1, 1, 1, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	3, 3, 0, 0, 0, 0, 0, 0,
	0, 3,
}

var _addressed_value_index_offsets []byte = []byte{
	0, 0, 4, 7, 9, 11, 13, 15,
	17, 21, 25, 31, 35, 42, 44, 46,
	48, 50, 52, 54, 56, 62, 64, 66,
	68, 70, 72, 74, 76, 78, 80, 82,
	84, 91, 98, 101, 103, 105, 107, 109,
	113, 117,
}

var _addressed_value_indicies []byte = []byte{
	0, 0, 2, 1, 3, 4, 1, 5,
	1, 6, 1, 7, 1, 8, 1, 9,
	1, 10, 10, 11, 1, 12, 12, 13,
	1, 14, 14, 15, 16, 13, 1, 17,
	17, 18, 1, 18, 18, 3, 19, 19,
	4, 1, 20, 1, 21, 1, 9, 1,
	22, 1, 23, 1, 24, 1, 9, 1,
	25, 25, 26, 27, 28, 1, 29, 1,
	30, 1, 31, 1, 32, 1, 33, 1,
	34, 1, 35, 1, 36, 1, 37, 1,
	38, 1, 39, 1, 40, 40, 41, 41,
	41, 41, 1, 42, 42, 43, 43, 43,
	43, 1, 44, 45, 1, 33, 1, 46,
	1, 47, 1, 33, 1, 48, 48, 49,
	1, 50, 50, 2, 1, 51, 51, 52,
	43, 43, 43, 43, 1,
}

var _addressed_value_trans_targs []byte = []byte{
	1, 0, 2, 3, 16, 4, 5, 6,
	7, 8, 9, 10, 9, 10, 11, 12,
	20, 11, 12, 13, 14, 15, 17, 18,
	19, 20, 21, 26, 34, 22, 23, 24,
	25, 39, 27, 28, 29, 30, 31, 32,
	33, 41, 33, 41, 35, 36, 37, 38,
	40, 2, 40, 40, 2,
}

var _addressed_value_trans_actions []byte = []byte{
	0, 0, 0, 1, 1, 1, 1, 1,
	1, 1, 3, 7, 0, 1, 10, 10,
	10, 0, 0, 1, 1, 1, 1, 1,
	1, 0, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1, 1,
	5, 13, 0, 1, 1, 1, 1, 1,
	16, 16, 0, 19, 19,
}

var _addressed_value_eof_actions []byte = []byte{
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 16,
	0, 19,
}

const addressed_value_start int = 1
const addressed_value_first_final int = 39
const addressed_value_error int = 0

const addressed_value_en_main int = 1

// line 17 "parse_addr.rl"

func readLayer(layerString string) (Layer, error) {
	switch layerString {
	case "TOKEN":
		return TOKEN, nil
	case "TAG":
		return TAG, nil
	case "DEPREL":
		return DEPREL, nil
	case "FEATURE":
		return FEATURE, nil
	default:
		return 0, fmt.Errorf("Unknown layer: %s", layerString)
	}
}

func readSource(sourceString string) (Source, error) {
	switch sourceString {
	case "STACK":
		return STACK, nil
	case "BUFFER":
		return BUFFER, nil
	case "LDEP":
		return LDEP, nil
	case "RDEP":
		return RDEP, nil
	default:
		return 0, fmt.Errorf("Unknown source: %s", sourceString)
	}
}

func ParseAddressedValueTemplates(data []byte) ([]AddressedValue, error) {
	cs, p, pe, eof := 0, 0, len(data), len(data)

	components := make([]AddressComponent, 0)
	templates := make([]AddressedValue, 0)

	var buf bytes.Buffer
	var source Source
	var index uint64
	var layer Layer
	var layerArg string
	var err error

	// line 179 "parse_addr.go"
	{
		cs = addressed_value_start
	}

	// line 184 "parse_addr.go"
	{
		var _klen int
		var _trans int
		var _acts int
		var _nacts uint
		var _keys int
		if p == pe {
			goto _test_eof
		}
		if cs == 0 {
			goto _out
		}
	_resume:
		_keys = int(_addressed_value_key_offsets[cs])
		_trans = int(_addressed_value_index_offsets[cs])

		_klen = int(_addressed_value_single_lengths[cs])
		if _klen > 0 {
			_lower := int(_keys)
			var _mid int
			_upper := int(_keys + _klen - 1)
			for {
				if _upper < _lower {
					break
				}

				_mid = _lower + ((_upper - _lower) >> 1)
				switch {
				case data[p] < _addressed_value_trans_keys[_mid]:
					_upper = _mid - 1
				case data[p] > _addressed_value_trans_keys[_mid]:
					_lower = _mid + 1
				default:
					_trans += int(_mid - int(_keys))
					goto _match
				}
			}
			_keys += _klen
			_trans += _klen
		}

		_klen = int(_addressed_value_range_lengths[cs])
		if _klen > 0 {
			_lower := int(_keys)
			var _mid int
			_upper := int(_keys + (_klen << 1) - 2)
			for {
				if _upper < _lower {
					break
				}

				_mid = _lower + (((_upper - _lower) >> 1) & ^1)
				switch {
				case data[p] < _addressed_value_trans_keys[_mid]:
					_upper = _mid - 2
				case data[p] > _addressed_value_trans_keys[_mid+1]:
					_lower = _mid + 2
				default:
					_trans += int((_mid - int(_keys)) >> 1)
					goto _match
				}
			}
			_trans += _klen
		}

	_match:
		_trans = int(_addressed_value_indicies[_trans])
		cs = int(_addressed_value_trans_targs[_trans])

		if _addressed_value_trans_actions[_trans] == 0 {
			goto _again
		}

		_acts = int(_addressed_value_trans_actions[_trans])
		_nacts = uint(_addressed_value_actions[_acts])
		_acts++
		for ; _nacts > 0; _nacts-- {
			_acts++
			switch _addressed_value_actions[_acts-1] {
			case 0:
				// line 64 "parse_addr.rl"

				buf.WriteByte(data[p])
			case 1:
				// line 65 "parse_addr.rl"

				source, err = readSource(buf.String())
				if err != nil {
					return nil, err
				}

				buf.Reset()

			case 2:
				// line 74 "parse_addr.rl"

				index, err = strconv.ParseUint(buf.String(), 10, 64)
				if err != nil {
					return nil, err
				}

				buf.Reset()

			case 3:
				// line 83 "parse_addr.rl"

				layer, err = readLayer(buf.String())
				if err != nil {
					return nil, err
				}

				buf.Reset()

			case 4:
				// line 92 "parse_addr.rl"

				layerArg = buf.String()
				buf.Reset()

			case 5:
				// line 97 "parse_addr.rl"

				components = append(components, AddressComponent{source, uint(index)})

			case 6:
				// line 101 "parse_addr.rl"

				templates = append(templates, AddressedValue{components, layer, layerArg, ""})
				components = make([]AddressComponent, 0)
				layerArg = ""

				// line 321 "parse_addr.go"
			}
		}

	_again:
		if cs == 0 {
			goto _out
		}
		p++
		if p != pe {
			goto _resume
		}
	_test_eof:
		{
		}
		if p == eof {
			__acts := _addressed_value_eof_actions[cs]
			__nacts := uint(_addressed_value_actions[__acts])
			__acts++
			for ; __nacts > 0; __nacts-- {
				__acts++
				switch _addressed_value_actions[__acts-1] {
				case 3:
					// line 83 "parse_addr.rl"

					layer, err = readLayer(buf.String())
					if err != nil {
						return nil, err
					}

					buf.Reset()

				case 4:
					// line 92 "parse_addr.rl"

					layerArg = buf.String()
					buf.Reset()

				case 6:
					// line 101 "parse_addr.rl"

					templates = append(templates, AddressedValue{components, layer, layerArg, ""})
					components = make([]AddressComponent, 0)
					layerArg = ""

					// line 366 "parse_addr.go"
				}
			}
		}

	_out:
		{
		}
	}

	// line 125 "parse_addr.rl"

	if cs < addressed_value_first_final {
		if p == pe {
			return nil, errors.New("Unexpected end of file")
		}

		return nil, fmt.Errorf("Error in feature line at position %d: %s", p, string(data))
	}

	return templates, nil
}

func parseAddressedValueGenerator(data []byte) (FeatureGenerator, error) {
	templates, err := ParseAddressedValueTemplates(data)
	if err != nil {
		return nil, err
	}

	return NewAddressedValueGenerator(templates), nil
}
