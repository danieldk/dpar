package features

// Parse addresses

import (
  "bytes"
  "errors"
  "fmt"
  "strconv"
)

%%{
  machine addressed_value;
  write data;
}%%

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
  cs, p,pe, eof := 0, 0, len(data), len(data)

  components := make([]AddressComponent, 0)
  templates := make([]AddressedValue, 0)

  var buf bytes.Buffer
  var source Source
  var index uint64
  var layer Layer
  var layerArg string
  var err error

%%{
 
  action str_char { buf.WriteByte(fc) }
  action source {
    source, err = readSource(buf.String())
    if err != nil {
      return nil, err
    }

    buf.Reset()
  }

  action index {
    index, err = strconv.ParseUint(buf.String(), 10, 64)
    if err != nil {
      return nil, err
    }

    buf.Reset()
  }

  action layer {
    layer, err = readLayer(buf.String())
    if err != nil {
      return nil, err
    }

    buf.Reset()
  }

  action layerArg {
    layerArg = buf.String()
    buf.Reset()
  }

  action component {
    components = append(components, AddressComponent{source, uint(index)})
  }

  action addressedValue {
    templates = append(templates, AddressedValue{components, layer, layerArg, ""})
    components = make([]AddressComponent, 0)
    layerArg = ""
  }

  ws = [\t ]+;
  initialSource = ("STACK"|"BUFFER") $ str_char % source;
  source = ("STACK"|"BUFFER"|"LDEP"|"RDEP") $ str_char % source;
  index = [0-9]+ $ str_char % index;

  layerArg = [a-zA-Z0-9]+ $ str_char % layerArg;
  noArgLayer = ("TOKEN"|"TAG"|"DEPREL") $ str_char % layer;
  argLayer = "FEATURE" $str_char % layer;
  layer = (argLayer ws* layerArg | noArgLayer);

  initialAddrComponent = initialSource ws* index % component;
  addrComponent = source ws* index % component;
  addrComponents = '[' initialAddrComponent (ws* ',' ws* addrComponent)* ']';
  addressedValue = addrComponents ws* layer % addressedValue;
  main := ws* addressedValue (ws* addressedValue)* ws*;

  write init;
  write exec;
}%%

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
