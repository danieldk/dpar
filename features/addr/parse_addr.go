
//line parse_addr.rl:1
// Generated using ragel; DO NOT EDIT!

package addr

// Parse addresses

import (
  "bytes"
  "errors"
  "fmt"
  "strconv"
)


//line parse_addr.go:18
var _addressed_value_actions []byte = []byte{
	0, 1, 0, 1, 1, 1, 3, 1, 5, 
	2, 1, 0, 2, 2, 7, 2, 3, 
	8, 2, 4, 8, 2, 6, 8, 
}

var _addressed_value_key_offsets []byte = []byte{
	0, 0, 3, 5, 6, 7, 8, 9, 
	10, 14, 18, 24, 27, 33, 34, 35, 
	36, 37, 38, 39, 40, 46, 47, 48, 
	49, 51, 55, 59, 63, 64, 65, 66, 
	67, 68, 69, 70, 71, 72, 73, 74, 
	76, 86, 88, 89, 90, 91, 92, 97, 
	100, 103, 
}

var _addressed_value_trans_keys []byte = []byte{
	9, 32, 91, 66, 83, 85, 70, 70, 
	69, 82, 9, 32, 48, 57, 9, 32, 
	48, 57, 9, 32, 44, 93, 48, 57, 
	9, 32, 44, 9, 32, 66, 76, 82, 
	83, 68, 69, 80, 84, 65, 67, 75, 
	9, 32, 67, 68, 70, 84, 72, 65, 
	82, 9, 32, 9, 32, 48, 57, 9, 
	32, 48, 57, 9, 32, 48, 57, 69, 
	80, 82, 69, 76, 69, 65, 84, 85, 
	82, 69, 9, 32, 9, 32, 42, 95, 
	48, 57, 65, 90, 97, 122, 65, 79, 
	71, 75, 69, 78, 9, 32, 91, 48, 
	57, 9, 32, 91, 9, 32, 91, 9, 
	32, 42, 91, 95, 48, 57, 65, 90, 
	97, 122, 
}

var _addressed_value_single_lengths []byte = []byte{
	0, 3, 2, 1, 1, 1, 1, 1, 
	2, 2, 4, 3, 6, 1, 1, 1, 
	1, 1, 1, 1, 6, 1, 1, 1, 
	2, 2, 2, 2, 1, 1, 1, 1, 
	1, 1, 1, 1, 1, 1, 1, 2, 
	4, 2, 1, 1, 1, 1, 3, 3, 
	3, 5, 
}

var _addressed_value_range_lengths []byte = []byte{
	0, 0, 0, 0, 0, 0, 0, 0, 
	1, 1, 1, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 1, 1, 1, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	3, 0, 0, 0, 0, 0, 1, 0, 
	0, 3, 
}

var _addressed_value_index_offsets []byte = []byte{
	0, 0, 4, 7, 9, 11, 13, 15, 
	17, 21, 25, 31, 35, 42, 44, 46, 
	48, 50, 52, 54, 56, 63, 65, 67, 
	69, 72, 76, 80, 84, 86, 88, 90, 
	92, 94, 96, 98, 100, 102, 104, 106, 
	109, 117, 120, 122, 124, 126, 128, 133, 
	137, 141, 
}

var _addressed_value_indicies []byte = []byte{
	0, 0, 2, 1, 3, 4, 1, 5, 
	1, 6, 1, 7, 1, 8, 1, 9, 
	1, 10, 10, 11, 1, 12, 12, 13, 
	1, 14, 14, 15, 16, 13, 1, 17, 
	17, 18, 1, 18, 18, 3, 19, 19, 
	4, 1, 20, 1, 21, 1, 9, 1, 
	22, 1, 23, 1, 24, 1, 9, 1, 
	25, 25, 26, 27, 28, 29, 1, 30, 
	1, 31, 1, 32, 1, 33, 33, 1, 
	34, 34, 35, 1, 36, 36, 35, 1, 
	37, 37, 38, 1, 39, 1, 40, 1, 
	41, 1, 42, 1, 43, 1, 44, 1, 
	45, 1, 46, 1, 47, 1, 48, 1, 
	49, 1, 50, 50, 1, 51, 51, 52, 
	52, 52, 52, 52, 1, 53, 54, 1, 
	43, 1, 55, 1, 56, 1, 43, 1, 
	57, 57, 58, 38, 1, 59, 59, 2, 
	1, 60, 60, 61, 1, 62, 62, 52, 
	63, 52, 52, 52, 52, 1, 
}

var _addressed_value_trans_targs []byte = []byte{
	1, 0, 2, 3, 16, 4, 5, 6, 
	7, 8, 9, 10, 9, 10, 11, 12, 
	20, 11, 12, 13, 14, 15, 17, 18, 
	19, 20, 21, 28, 33, 41, 22, 23, 
	24, 25, 25, 26, 27, 27, 46, 29, 
	30, 31, 32, 48, 34, 35, 36, 37, 
	38, 39, 40, 40, 49, 42, 43, 44, 
	45, 47, 2, 47, 47, 2, 47, 2, 
}

var _addressed_value_trans_actions []byte = []byte{
	0, 0, 0, 1, 1, 1, 1, 1, 
	1, 1, 3, 9, 0, 1, 12, 12, 
	12, 0, 0, 1, 1, 1, 1, 1, 
	1, 0, 1, 1, 1, 1, 1, 1, 
	1, 5, 0, 1, 7, 0, 1, 1, 
	1, 1, 1, 1, 1, 1, 1, 1, 
	1, 1, 5, 0, 1, 1, 1, 1, 
	1, 21, 21, 0, 15, 15, 18, 18, 
}

var _addressed_value_eof_actions []byte = []byte{
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 21, 0, 
	15, 18, 
}

const addressed_value_start int = 1
const addressed_value_first_final int = 46
const addressed_value_error int = 0

const addressed_value_en_main int = 1


//line parse_addr.rl:17


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
  case "CHAR":
    return CHAR, nil
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
  cs, p,pe, eof := 0, 0, len(data), len(data)

  components := make([]AddressComponent, 0)
  templates := make([]AddressedValue, 0)

  var buf bytes.Buffer
  var source Source
  var index uint64
  var layer Layer
  var layerArg string
  var layerInt0Arg int
  var layerInt1Arg int
  var err error


//line parse_addr.go:195
	{
	cs = addressed_value_start
	}

//line parse_addr.go:200
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
			case data[p] > _addressed_value_trans_keys[_mid + 1]:
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
	_nacts = uint(_addressed_value_actions[_acts]); _acts++
	for ; _nacts > 0; _nacts-- {
		_acts++
		switch _addressed_value_actions[_acts-1] {
		case 0:
//line parse_addr.rl:68
 buf.WriteByte(data[p]) 
		case 1:
//line parse_addr.rl:69

    source, err = readSource(buf.String())
    if err != nil {
      return nil, err
    }

    buf.Reset()
  
		case 2:
//line parse_addr.rl:78

    index, err = strconv.ParseUint(buf.String(), 10, 64)
    if err != nil {
      return nil, err
    }

    buf.Reset()
  
		case 3:
//line parse_addr.rl:87

    layer, err = readLayer(buf.String())
    if err != nil {
      return nil, err
    }

    buf.Reset()
  
		case 4:
//line parse_addr.rl:96

    layerArg = buf.String()
    buf.Reset()
  
		case 5:
//line parse_addr.rl:101

    val, err := strconv.ParseInt(buf.String(), 10, 64)
    if err != nil {
      return nil, err
    }
    layerInt0Arg = int(val) 
    buf.Reset()
  
		case 6:
//line parse_addr.rl:110

    val, err := strconv.ParseInt(buf.String(), 10, 64)
    if err != nil {
      return nil, err
    }
    layerInt1Arg = int(val) 
    buf.Reset()
  
		case 7:
//line parse_addr.rl:119

    components = append(components, AddressComponent{source, uint(index)})
  
		case 8:
//line parse_addr.rl:123

    templates = append(templates, AddressedValue{components, layer, layerArg, layerInt0Arg, layerInt1Arg})
    components = make([]AddressComponent, 0)
    layerArg = ""
    layerInt0Arg = 0
    layerInt1Arg = 0
  
//line parse_addr.go:352
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
	_test_eof: {}
	if p == eof {
		__acts := _addressed_value_eof_actions[cs]
		__nacts := uint(_addressed_value_actions[__acts]); __acts++
		for ; __nacts > 0; __nacts-- {
			__acts++
			switch _addressed_value_actions[__acts-1] {
			case 3:
//line parse_addr.rl:87

    layer, err = readLayer(buf.String())
    if err != nil {
      return nil, err
    }

    buf.Reset()
  
			case 4:
//line parse_addr.rl:96

    layerArg = buf.String()
    buf.Reset()
  
			case 6:
//line parse_addr.rl:110

    val, err := strconv.ParseInt(buf.String(), 10, 64)
    if err != nil {
      return nil, err
    }
    layerInt1Arg = int(val) 
    buf.Reset()
  
			case 8:
//line parse_addr.rl:123

    templates = append(templates, AddressedValue{components, layer, layerArg, layerInt0Arg, layerInt1Arg})
    components = make([]AddressComponent, 0)
    layerArg = ""
    layerInt0Arg = 0
    layerInt1Arg = 0
  
//line parse_addr.go:406
			}
		}
	}

	_out: {}
	}

//line parse_addr.rl:152


  if cs < addressed_value_first_final {
    if p == pe {
      return nil, errors.New("Unexpected end of file")
    }

    return nil, fmt.Errorf("Error in feature line at position %d: %s", p, string(data))
  }

  return templates, nil
}
