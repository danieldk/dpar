// Generated using ragel; DO NOT EDIT!

// Parse addresses

use {ErrorKind, Result};
use features::addr::{AddressedValue, Layer, Source};

%%{
  machine addressed_value;
  write data;
}%%

fn create_component(keyword: &str, args: &[String]) -> Result<Source> {
  if args.len() != 1 {
     bail!(ErrorKind::ConfigError(format!("Expected one argument for {}, got {}", keyword, args.len())));
  }

  let idx = args[0].parse()?;

  let source = match keyword {
    "STACK" => Source::Stack(idx),
    "BUFFER" => Source::Buffer(idx),
    "LDEP" => Source::LDep(idx),
    "RDEP" => Source::RDep(idx),
    _ => bail!(ErrorKind::ConfigError(format!("Unknown source type: {}", keyword)))
  };

  Ok(source)
}

fn create_layer(keyword: &str, args: &[String]) -> Result<Layer> {
   let layer = match keyword {
     "TOKEN" => {
       if !args.is_empty() {
         bail!(ErrorKind::ConfigError(format!("Expected no arguments for {}, got {}", keyword, args.len())));
       }
       Layer::Token
     }
     "TAG" => {
       if !args.is_empty() {
         bail!(ErrorKind::ConfigError(format!("Expected no arguments for {}, got {}", keyword, args.len())));
       }
       Layer::Tag
     }
     "DEPREL" => {
       if !args.is_empty() {
         bail!(ErrorKind::ConfigError(format!("Expected no arguments for {}, got {}", keyword, args.len())));
       }
       Layer::DepRel
     }
     "FEATURE" => {
       if args.len() != 1 {
         bail!(ErrorKind::ConfigError(format!("Expected one argument for {}, got {}", keyword, args.len())));
       }

       Layer::Feature(args[0].clone())
     }
     "CHAR" => {
       if args.len() != 2 {
         bail!(ErrorKind::ConfigError(format!("Expected two arguments for {}, got {}", keyword, args.len())));
       }

       Layer::Char(args[0].parse()?, args[1].parse()?)
     }
    _ => bail!(ErrorKind::ConfigError(format!("Unknown layer type: {}", keyword)))
   };

   Ok(layer)
}

// ParseAddressedValueTemplates parses templates of AddressedValue-based
// features.
pub fn parse_addressed_value_templates(data: &[u8]) -> Result<Vec<AddressedValue>> {
  let mut cs: i32;
  let mut p = 0;
  let pe = data.len();
  let eof = data.len();
  //cs, p,pe, eof := 0, 0, len(data), len(data)

  let mut buf = Vec::new();
  let mut keyword = String::new();
  let mut args = Vec::new();
  let mut components = Vec::new();
  let mut addressed_values = Vec::new();

%%{
 
  action str_char { buf.push(fc) }

  action keyword {
    keyword = String::from_utf8(buf)?;
    buf = Vec::new();
  }

  action arg {
    args.push(String::from_utf8(buf)?);
    buf = Vec::new();
  }

  action component {
    components.push(create_component(&keyword, &args)?);
    keyword.clear();
    args.clear();
  }

  action addressedValue {
    let layer = create_layer(&keyword, &args)?;
    addressed_values.push(
      AddressedValue {
        address: components,
        layer: layer,
      }
    );
    components = Vec::new();
    keyword.clear();
    args.clear();
  }

  ws = [\t ]+;
  initialSource = ("STACK"|"BUFFER") $ str_char % keyword;
  source = ("STACK"|"BUFFER"|"LDEP"|"RDEP") $ str_char % keyword;
  index = [0-9]+ $ str_char % arg;

  layerArg = [_a-zA-Z0-9*]+ $ str_char % arg;
  layerIntArg = ([0-9]+) $ str_char % arg;
  noArgLayer = ("TOKEN"|"TAG"|"DEPREL") $ str_char % keyword;
  argLayer = "FEATURE" $ str_char % keyword;
  intIntArgLayer = "CHAR" $str_char % keyword;
  layer = (argLayer ws+ layerArg | intIntArgLayer ws+ layerIntArg ws+ layerIntArg | noArgLayer);

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
      bail!(ErrorKind::ConfigError("Unexpected end of file".to_owned()));
    }

    bail!(format!("Error in feature line at position {}: {}", p, String::from_utf8(data.to_owned())?));
  }

  //return templates, nil
  Ok(addressed_values)
}
