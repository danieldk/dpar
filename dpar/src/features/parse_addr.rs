use features::addr::{AddressedValue, Layer, Source};
use {ErrorKind, Result};

static _addressed_value_actions: [i8; 21] = [
    0, 1, 0, 1, 1, 1, 2, 2, 1, 0, 2, 1, 4, 2, 2, 3, 2, 2, 4, 0, 0,
];
static _addressed_value_key_offsets: [i8; 52] = [
    0, 0, 3, 5, 6, 7, 8, 9, 10, 14, 18, 24, 27, 33, 34, 35, 36, 37, 38, 39, 40, 46, 47, 48, 49, 51,
    55, 59, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 76, 86, 88, 89, 90, 91, 92, 97, 100,
    103, 0, 0,
];
static _addressed_value_trans_keys: [u8; 116] = [
    9, 32, 91, 66, 83, 85, 70, 70, 69, 82, 9, 32, 48, 57, 9, 32, 48, 57, 9, 32, 44, 93, 48, 57, 9,
    32, 44, 9, 32, 66, 76, 82, 83, 68, 69, 80, 84, 65, 67, 75, 9, 32, 67, 68, 70, 84, 72, 65, 82,
    9, 32, 9, 32, 48, 57, 9, 32, 48, 57, 9, 32, 48, 57, 69, 80, 82, 69, 76, 69, 65, 84, 85, 82, 69,
    9, 32, 9, 32, 42, 95, 48, 57, 65, 90, 97, 122, 65, 79, 71, 75, 69, 78, 9, 32, 91, 48, 57, 9,
    32, 91, 9, 32, 91, 9, 32, 42, 91, 95, 48, 57, 65, 90, 97, 122, 0, 0,
];
static _addressed_value_single_lengths: [i8; 52] = [
    0, 3, 2, 1, 1, 1, 1, 1, 2, 2, 4, 3, 6, 1, 1, 1, 1, 1, 1, 1, 6, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 2, 4, 2, 1, 1, 1, 1, 3, 3, 3, 5, 0, 0,
];
static _addressed_value_range_lengths: [i8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1, 0, 0, 3, 0, 0,
];
static _addressed_value_index_offsets: [i16; 52] = [
    0, 0, 4, 7, 9, 11, 13, 15, 17, 21, 25, 31, 35, 42, 44, 46, 48, 50, 52, 54, 56, 63, 65, 67, 69,
    72, 76, 80, 84, 86, 88, 90, 92, 94, 96, 98, 100, 102, 104, 106, 109, 117, 120, 122, 124, 126,
    128, 133, 137, 141, 0, 0,
];
static _addressed_value_trans_cond_spaces: [i8; 152] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, 0, 0,
];
static _addressed_value_trans_offsets: [i16; 152] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 0, 0,
];
static _addressed_value_trans_lengths: [i8; 152] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
];
static _addressed_value_cond_keys: [i8; 152] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _addressed_value_cond_targs: [i8; 152] = [
    1, 1, 2, 0, 3, 16, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 9, 10, 0, 9, 9, 10, 0, 11, 11, 12, 20,
    10, 0, 11, 11, 12, 0, 12, 12, 3, 13, 13, 16, 0, 14, 0, 15, 0, 8, 0, 17, 0, 18, 0, 19, 0, 8, 0,
    20, 20, 21, 28, 33, 41, 0, 22, 0, 23, 0, 24, 0, 25, 25, 0, 25, 25, 26, 0, 27, 27, 26, 0, 27,
    27, 46, 0, 29, 0, 30, 0, 31, 0, 32, 0, 48, 0, 34, 0, 35, 0, 36, 0, 37, 0, 38, 0, 39, 0, 40, 40,
    0, 40, 40, 49, 49, 49, 49, 49, 0, 42, 43, 0, 48, 0, 44, 0, 45, 0, 48, 0, 47, 47, 2, 46, 0, 47,
    47, 2, 0, 47, 47, 2, 0, 47, 47, 49, 2, 49, 49, 49, 49, 0, 0, 0,
];
static _addressed_value_cond_actions: [i8; 152] = [
    0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 3, 3, 7, 0, 0, 0, 1, 0, 13, 13, 13, 13, 1,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1,
    0, 1, 0, 1, 0, 1, 0, 3, 3, 0, 0, 0, 1, 0, 5, 5, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 3, 3, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0,
    1, 0, 16, 16, 16, 1, 0, 0, 0, 0, 0, 10, 10, 10, 0, 16, 16, 1, 16, 1, 1, 1, 1, 0, 0, 0,
];
static _addressed_value_eof_actions: [i8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 10, 16, 0, 0,
];
static _addressed_value_nfa_targs: [i8; 3] = [0, 0, 0];
static _addressed_value_nfa_offsets: [i8; 52] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _addressed_value_nfa_push_actions: [i8; 3] = [0, 0, 0];
static _addressed_value_nfa_pop_trans: [i8; 3] = [0, 0, 0];
static addressed_value_start: i32 = 1;
static addressed_value_first_final: i32 = 46;
static addressed_value_error: i32 = 0;
static addressed_value_en_main: i32 = 1;
fn create_component(keyword: &str, args: &[String]) -> Result<Source> {
    if args.len() != 1 {
        bail!(ErrorKind::ConfigError(format!(
            "Expected one argument for {}, got {}",
            keyword,
            args.len()
        )));
    }

    let idx = args[0].parse()?;

    let source = match keyword {
        "STACK" => Source::Stack(idx),
        "BUFFER" => Source::Buffer(idx),
        "LDEP" => Source::LDep(idx),
        "RDEP" => Source::RDep(idx),
        _ => bail!(ErrorKind::ConfigError(format!(
            "Unknown source type: {}",
            keyword
        ),)),
    };

    Ok(source)
}

fn create_layer(keyword: &str, args: &[String]) -> Result<Layer> {
    let layer = match keyword {
        "TOKEN" => {
            if !args.is_empty() {
                bail!(ErrorKind::ConfigError(format!(
                    "Expected no arguments for {}, got {}",
                    keyword,
                    args.len()
                )));
            }
            Layer::Token
        }
        "TAG" => {
            if !args.is_empty() {
                bail!(ErrorKind::ConfigError(format!(
                    "Expected no arguments for {}, got {}",
                    keyword,
                    args.len()
                )));
            }
            Layer::Tag
        }
        "DEPREL" => {
            if !args.is_empty() {
                bail!(ErrorKind::ConfigError(format!(
                    "Expected no arguments for {}, got {}",
                    keyword,
                    args.len()
                )));
            }
            Layer::DepRel
        }
        "FEATURE" => {
            if args.len() != 1 {
                bail!(ErrorKind::ConfigError(format!(
                    "Expected one argument for {}, got {}",
                    keyword,
                    args.len()
                )));
            }

            Layer::Feature(args[0].clone())
        }
        "CHAR" => {
            if args.len() != 2 {
                bail!(ErrorKind::ConfigError(format!(
                    "Expected two arguments for {}, got {}",
                    keyword,
                    args.len()
                )));
            }

            Layer::Char(args[0].parse()?, args[1].parse()?)
        }
        _ => bail!(ErrorKind::ConfigError(format!(
            "Unknown layer type: {}",
            keyword
        ),)),
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

    {
        cs = (addressed_value_start) as i32;
    }

    {
        let mut _klen = 0;
        let mut _trans = 0;
        let mut _cond = 0;
        let mut _have = 0;
        let mut _cont = 1;
        let mut _acts: i32 = 0;
        let mut _nacts = 0;
        let mut _keys: i32 = 0;
        let mut _ckeys: i32 = 0;
        let mut _cpc = 0;
        while (_cont == 1) {
            if (cs == 0) {
                _cont = 0;
            }
            _have = 0;
            if (p == pe) {
                {
                    if (p == eof) {
                        {
                            if (_have == 0) {
                                {
                                    let mut __acts: i32 = 0;
                                    let mut __nacts = 0;
                                    __acts = (_addressed_value_eof_actions[(cs) as usize]) as i32;
                                    __nacts = (_addressed_value_actions[(__acts) as usize]) as u32;
                                    __acts += 1;
                                    while (__nacts > 0) {
                                        match (_addressed_value_actions[(__acts) as usize]) {
                                            1 => {
                                                keyword = String::from_utf8(buf)?;
                                                buf = Vec::new();
                                            }
                                            2 => {
                                                args.push(String::from_utf8(buf)?);
                                                buf = Vec::new();
                                            }
                                            4 => {
                                                let layer = create_layer(&keyword, &args)?;
                                                addressed_values.push(AddressedValue {
                                                    address: components,
                                                    layer: layer,
                                                });
                                                components = Vec::new();
                                                keyword.clear();
                                                args.clear();
                                            }

                                            _ => {}
                                        }
                                        __nacts -= 1;
                                        __acts += 1;
                                    }
                                }
                            }
                        }
                    }
                    if (_have == 0) {
                        _cont = 0;
                    }
                }
            }
            if (_cont == 1) {
                {
                    if (_have == 0) {
                        {
                            _keys = (_addressed_value_key_offsets[(cs) as usize]) as i32;
                            _trans = (_addressed_value_index_offsets[(cs) as usize]) as u32;
                            _have = 0;
                            _klen = (_addressed_value_single_lengths[(cs) as usize]) as i32;
                            if (_klen > 0) {
                                {
                                    let mut _lower: i32 = 0;
                                    let mut _mid: i32 = 0;
                                    let mut _upper: i32 = 0;
                                    _lower = _keys;
                                    _upper = _keys + _klen - 1;
                                    while (_upper >= _lower && _have == 0) {
                                        _mid = _lower + ((_upper - _lower) >> 1);
                                        if ((data[(p) as usize])
                                            < _addressed_value_trans_keys[(_mid) as usize])
                                        {
                                            _upper = _mid - 1;
                                        } else if ((data[(p) as usize])
                                            > _addressed_value_trans_keys[(_mid) as usize])
                                        {
                                            _lower = _mid + 1;
                                        } else {
                                            {
                                                _trans += (_mid - _keys) as u32;
                                                _have = 1;
                                            }
                                        }
                                    }

                                    if (_have == 0) {
                                        {
                                            _keys += _klen;
                                            _trans += (_klen) as u32;
                                        }
                                    }
                                }
                            }
                            if (_have == 0) {
                                {
                                    _klen = (_addressed_value_range_lengths[(cs) as usize]) as i32;
                                    if (_klen > 0) {
                                        {
                                            let mut _lower: i32 = 0;
                                            let mut _mid: i32 = 0;
                                            let mut _upper: i32 = 0;
                                            _lower = _keys;
                                            _upper = _keys + (_klen << 1) - 2;
                                            while (_have == 0 && _lower <= _upper) {
                                                _mid = _lower + (((_upper - _lower) >> 1) & !1);
                                                if ((data[(p) as usize])
                                                    < _addressed_value_trans_keys[(_mid) as usize])
                                                {
                                                    _upper = _mid - 2;
                                                } else if ((data[(p) as usize])
                                                    > _addressed_value_trans_keys
                                                        [(_mid + 1) as usize])
                                                {
                                                    _lower = _mid + 2;
                                                } else {
                                                    {
                                                        _trans += ((_mid - _keys) >> 1) as u32;
                                                        _have = 1;
                                                    }
                                                }
                                            }

                                            if (_have == 0) {
                                                _trans += (_klen) as u32;
                                            }
                                        }
                                    }
                                }
                            }
                            _ckeys = (_addressed_value_trans_offsets[(_trans) as usize]) as i32;
                            _klen = (_addressed_value_trans_lengths[(_trans) as usize]) as i32;
                            _cond = (_addressed_value_trans_offsets[(_trans) as usize]) as u32;
                            _have = 0;
                            _cpc = 0;
                            {
                                let mut _lower: i32 = 0;
                                let mut _mid: i32 = 0;
                                let mut _upper: i32 = 0;
                                _lower = _ckeys;
                                _upper = _ckeys + _klen - 1;
                                while (_have == 0 && _lower <= _upper) {
                                    _mid = _lower + ((_upper - _lower) >> 1);
                                    if (_cpc < (_addressed_value_cond_keys[(_mid) as usize]) as i32)
                                    {
                                        _upper = _mid - 1;
                                    } else if (_cpc
                                        > (_addressed_value_cond_keys[(_mid) as usize]) as i32)
                                    {
                                        _lower = _mid + 1;
                                    } else {
                                        {
                                            _cond += (_mid - _ckeys) as u32;
                                            _have = 1;
                                        }
                                    }
                                }

                                if (_have == 0) {
                                    {
                                        cs = 0;
                                        _cont = 0;
                                    }
                                }
                            }
                        }
                    }
                    if (_cont == 1) {
                        {
                            cs = (_addressed_value_cond_targs[(_cond) as usize]) as i32;
                            if (_addressed_value_cond_actions[(_cond) as usize] != 0) {
                                {
                                    _acts =
                                        (_addressed_value_cond_actions[(_cond) as usize]) as i32;
                                    _nacts = (_addressed_value_actions[(_acts) as usize]) as u32;
                                    _acts += 1;
                                    while (_nacts > 0) {
                                        match (_addressed_value_actions[(_acts) as usize]) {
                                            0 => buf.push((data[(p) as usize])),
                                            1 => {
                                                keyword = String::from_utf8(buf)?;
                                                buf = Vec::new();
                                            }
                                            2 => {
                                                args.push(String::from_utf8(buf)?);
                                                buf = Vec::new();
                                            }
                                            3 => {
                                                components.push(create_component(&keyword, &args)?);
                                                keyword.clear();
                                                args.clear();
                                            }
                                            4 => {
                                                let layer = create_layer(&keyword, &args)?;
                                                addressed_values.push(AddressedValue {
                                                    address: components,
                                                    layer: layer,
                                                });
                                                components = Vec::new();
                                                keyword.clear();
                                                args.clear();
                                            }

                                            _ => {}
                                        }
                                        _nacts -= 1;
                                        _acts += 1;
                                    }
                                }
                            }
                            if (cs == 0) {
                                _cont = 0;
                            }
                            if (_cont == 1) {
                                p += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    if cs < addressed_value_first_final {
        if p == pe {
            bail!(ErrorKind::ConfigError("Unexpected end of file".to_owned()));
        }

        bail!(format!(
            "Error in feature line at position {}: {}",
            p,
            String::from_utf8(data.to_owned())?
        ));
    }

    //return templates, nil
    Ok(addressed_values)
}
