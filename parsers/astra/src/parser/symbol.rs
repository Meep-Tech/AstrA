use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Cadence {
    Alone,
    Intro,
    Prefix,
    Suffix,
    Infix,
    Spaced,
    Sub,
    Between,
    Open,
    Capture,
    Close,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Symbol {
    name: String,
    kind: Kind,
    chars: Vec<char>,
    cadences: HashSet<Cadence>,
}

impl Symbol {
    #[allow(non_snake_case)]
    pub(crate) fn Type(name: &str, kind: Kind, chars: Vec<char>, cads: &[Cadence]) -> Self {
        Self {
            name: name.to_string(),
            kind,
            chars: chars.to_vec(),
            cadences: cads.iter().cloned().collect(),
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Unknown() -> Self {
        Self {
            name: "unknown".to_string(),
            kind: Kind::Unknown,
            chars: vec![],
            cadences: HashSet::new(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Kind {
    Delimiter(Delimiter),
    Operator(Operator),
    Unknown,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Delimiter {
    Bound,
    Start,
    End,
    Line,
    Modifier,
    Lookup,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Operator {
    Assigner,   // assign value to variable
    Caller,     // call value as proc with potential args
    Relational, // compare values
    Flow,       // effect the flow of code
    Logical,    // deal with logic based on values
}

pub enum Node {
    Type(Symbol),
    Map(Map),
    Set(Set),
}

pub struct Map {
    __: HashMap<char, Node>,
}

impl Map {
    pub fn get(&self, c: &char) -> Option<&Node> {
        self.__.get(c)
    }
}

pub struct Set {
    __: Vec<Node>,
}

pub static ALL: Vec<Symbol> = get_all();
pub static MAP: Map = get_map();

fn get_all() -> Vec<Symbol> {
    let mut out = vec![];
    out.extend(get_all_delims());
    out.extend(get_all_oprs());
    out
}

fn get_map() -> Map {
    let all = get_all();
    let mut map = Map { __: HashMap::new() };

    for symbol in all {
        let mut curr_map = &mut map;
        for (i, c) in symbol.chars.iter().enumerate() {
            if i == symbol.chars.len() - 1 {
                match curr_map.__.get_mut(&c) {
                    Some(Node::Set(set)) => {
                        set.__.push(Node::Type(symbol.clone()));
                        continue;
                    }
                    Some(_) => {
                        let mut set = Set { __: vec![] };
                        let node = curr_map.__.remove(&c).unwrap();
                        set.__.push(node);
                        set.__.push(Node::Type(symbol.clone()));
                        curr_map.__.insert(*c, Node::Set(set));
                    }
                    None => {
                        curr_map.__.insert(*c, Node::Type(symbol.clone()));
                        continue;
                    }
                }
            } else {
                match curr_map.__.get(&c) {
                    None => {
                        curr_map
                            .__
                            .insert(*c, Node::Map(Map { __: HashMap::new() }));
                        curr_map = match curr_map.__.get_mut(&c).unwrap() {
                            Node::Map(map) => map,
                            _ => unreachable!(),
                        };
                    }
                    Some(_) => {
                        curr_map = match curr_map.__.get(&c) {
                            None => {
                                curr_map
                                    .__
                                    .insert(*c, Node::Map(Map { __: HashMap::new() }));
                                match curr_map.__.get_mut(&c).unwrap() {
                                    Node::Map(map) => map,
                                    _ => unreachable!(),
                                }
                            }
                            Some(node) => match node {
                                Node::Map(mut map) => &mut map,
                                Node::Set(set) => {
                                    // if the first item in the set is a map, use that as the next curr_map,
                                    // otherwise create a new map and use that
                                    match set.__.first().unwrap() {
                                        Node::Map(mut map) => &mut map,
                                        _ => {
                                            let mut map = Map { __: HashMap::new() };
                                            let mut new_set = Set {
                                                __: vec![Node::Map(map)],
                                            };
                                            new_set.__.extend(set.__.drain(..));

                                            curr_map.__.insert(*c, Node::Set(new_set));
                                            curr_map = match curr_map.__.get_mut(&c).unwrap() {
                                                Node::Set(set) => match set.__.first().unwrap() {
                                                    Node::Map(mut map) => &mut map,
                                                    _ => unreachable!(),
                                                },
                                                _ => unreachable!(),
                                            };
                                            continue;
                                        }
                                    }
                                }
                                Node::Type(mut symbol) => {
                                    // if we find a type node, we need to create a set and map node
                                    // and add the map to the set before the existing symbol
                                    let mut set = Set { __: vec![] };
                                    set.__.push(Node::Map(Map { __: HashMap::new() }));
                                    set.__.push(Node::Type(symbol));
                                    curr_map.__.insert(*c, Node::Set(set));

                                    match curr_map.__.get_mut(&c).unwrap() {
                                        Node::Set(set) => match set.__.last().unwrap() {
                                            Node::Map(mut map) => &mut map,
                                            _ => unreachable!(),
                                        },
                                        _ => unreachable!(),
                                    }
                                }
                            },
                        };
                    }
                }
            }
        }
    }

    map
}

delims! [
    Lookup:
        local_lookup                   '.'    [Alone, Prefix, Infix, Sub]
        outer_lookup                   ".."   [Alone, Prefix, Infix, Sub]
        div_lookup                     '/'    [Alone, Prefix, Infix, Sub]
        trait_lookup                   ".#"   [Alone, Prefix, Infix, Sub]
        query_lookup                   ".?"   [Alone, Prefix, Infix, Sub]
        maybe_lookup                   "?."   [Prefix, Infix, Sub]
        tag_literal_lookup             ".##"  [Prefix, Infix, Sub]
        file_lookup                    "./"   [Alone, Prefix]
        folder_lookup                  "../"  [Alone, Prefix];
    Modifier:
        tag_prefix                     "#"    [Prefix]
        tag_literal_prefix             "##"   [Prefix]
        optional_tag_suffix            "?"    [Suffix]
        required_tag_suffix            "!"    [Suffix]
        param_prefix                   ">"    [Prefix]
        alias_prefix                   "|"    [Prefix]
        key_prefix                     "$"    [Prefix]
        target_prefix                  "@"    [Prefix];
    Bound:
        simple_string_delimiter        "'"    [Open, Capture, Close]
        markup_string_delimiter        '"'    [Open, Capture, Close]
        region_delimiter               "###"  [Intro];
    Line:
        doc_line_delimiter             "##"   [Intro]
        param_line_delimiter           "##"   [Intro];
    Start:
        map_start_delimiter            '{'    [Open]
        map_capture_delimiter          '{'    [Capture]
        block_start_delimiter          '['    [Open]
        block_capture_delimiter        '['    [Capture]
        group_start_delimiter          '('    [Open]
        group_capture_delimiter        '('    [Capture]
        pattern_start_delimiter        '`'    [Open]
        pattern_capture_delimiter      '`'    [Open];
    End:
        map_end_delimiter              '}'    [Close]
        block_end_delimiter            ']'    [Close]
        group_end_delimiter            ')'    [Close]
        pattern_end_delimiter          '`'    [Close];
];

oprs![
    Relational:
        tag_operator                   "#"   [Infix]
        value_equality_operator        "=="  [Between]
        reference_equality_operator    "===" [Between]
        value_inequality_operator      "!="  [Between]
        reference_inequality_operator  "!==" [Between]
        single_or_operator             "|"   [Infix]
        double_or_operator             "||"  [Between]
        single_and_operator            "&"   [Infix]
        double_and_operator            "&&"  [Between];
    Flow:
        if_missing_operator            "??"  [Between]
        if_exists_operator             "!!"  [Between]
        if_truthy_operator             "?:"  [Between]
        if_falsy_operator              "!:"  [Between]
        each_operator                  "*"   [Between];
    Logical:
        falsy_prefix                   "!"   [Prefix]
        truthy_prefix                  "?"   [Prefix]
        spread_operator                "..." [Prefix]
        single_add_operator            "+"   [Spaced]
        double_add_operator            "++"  [Infix]
        single_dash_operator           "-"   [Spaced]
        double_dash_operator           "--"  [Infix]
        single_times_operator          "*"   [Spaced]
        double_times_operator          "**"  [Infix]
        single_div_operator            "/"   [Spaced]
        double_div_operator            "//"  [Infix];
];

macro_rules! _delim {
    ($name:ident $chars:literal $type:ident:[$($cads:ident)+]) => {
        fn $name() -> Symbol {
            Symbol::Type(
                stringify!($name),
                Kind::Delimiter(Delimiter::$type),
                $chars.to_string().chars().collect(),
                &[$(Cadence::$cads,)+],
            )
        }
    };
}
pub(super) use _delim;

macro_rules! _delim_sec {
    ($type:ident: $($name:ident $chars:literal [$($cads:ident$(,)?)+]$(,)?)+) => {
        $(_delim!($name $chars $type:[$($cads)+]);)+
    };
}
pub(super) use _delim_sec;

macro_rules! delims {
    ($($type:ident:
        $($name:ident $chars:literal [$($cads:ident$(,)?)+]$(,)?)+;
    )*) => {
        $(_delim_sec!($type: $($name $chars [$($cads,)+])+);)+

        fn get_all_delims() -> Vec<Symbol> {
            vec![
                $(
                    $(
                        $name(),
                    )+
                )+
            ]
        }
    };
}
pub(super) use delims;

macro_rules! _opr {
    ($type:ident: $name:ident $chars:literal [$($cads:ident$(,)?)+]$(,)?) => {
        fn $name() -> Symbol {
            Symbol::Type(
                stringify!($name),
                Kind::Operator(Operator::$type),
                $chars.to_string().chars().collect(),
                &[$(Cadence::$cads,)+],
            )
        }
    };
}
pub(super) use _opr;

macro_rules! _opr_sec {
    ($type:ident: $($name:ident $chars:literal [$($cads:ident$(,)?)+]$(,)?)+) => {
        $(_opr!($type: $name $chars [$($cads,)+]);)+
    };
}
pub(super) use _opr_sec;

macro_rules! oprs {
    ($($type:ident:
        $($name:ident $chars:literal [$($cads:ident$(,)?)+]$(,)?)+;
    )*) => {
        $(_opr_sec!($type: $($name $chars [$($cads,)+])+);)+

        fn get_all_oprs() -> Vec<Symbol> {
            vec![
                $(
                    $(
                        $name(),
                    )+
                )+
            ]
        }
    };
}
pub(super) use oprs;
