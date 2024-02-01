use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
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
pub enum Check {}

impl Check {
    pub(crate) fn vs(&self, ctx: &mut Cursor) -> bool {
        match self {
            _ => {
                return true;
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Symbol {
    name: String,
    kind: Kind,
    chars: Vec<char>,
    cadences: HashSet<Cadence>,
    checks: Option<Vec<Check>>,
}

impl Symbol {
    #[allow(non_snake_case)]
    pub(crate) fn Type(name: &str, kind: Kind, chars: Vec<char>, cads: &[Cadence]) -> Self {
        Self {
            name: name.to_string(),
            kind,
            chars: chars.to_vec(),
            cadences: cads.iter().cloned().collect(),
            checks: None,
        }
    }

    #[allow(non_snake_case)]
    pub(crate) fn Unknown() -> Self {
        Self {
            name: "unknown".to_string(),
            kind: Kind::Unknown,
            chars: vec![],
            cadences: HashSet::new(),
            checks: None,
        }
    }

    pub(crate) fn cads(&self) -> &HashSet<Cadence> {
        &self.cadences
    }

    pub(crate) fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn checks(&self) -> &Option<Vec<Check>> {
        &self.checks
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Kind {
    Delimiter(Delimiter),
    Operator(Operator),
    Unknown,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Delimiter {
    Bound,
    Start,
    End,
    Line,
    Modifier,
    Lookup,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Operator {
    Assigner,   // assign value to variable
    Caller,     // call value as proc with potential args
    Relational, // compare values
    Flow,       // effect the flow of code
    Logical,    // deal with logic based on values
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Branch {
    pub leaves: Vec<Symbol>,
    pub branches: HashMap<char, Branch>,
}

pub static ALL: LazyLock<Vec<Symbol>> = LazyLock::new(|| get_all());
pub static MAP: LazyLock<Branch> = LazyLock::new(|| get_root());

fn get_all() -> Vec<Symbol> {
    let mut out = vec![];
    out.extend(get_all_delims());
    out.extend(get_all_oprs());
    out
}

fn get_root() -> Branch {
    let all = get_all();
    let mut root = Branch {
        leaves: vec![],
        branches: HashMap::new(),
    };
    let mut curr = &mut root;

    for symbol in all {
        let len = symbol.chars.len();
        let end = len;

        for i in 0..=len {
            if i == end {
                curr.leaves.push(symbol.clone());
                curr = &mut root;
                break;
            }

            let c = &symbol.chars[i];
            if curr.branches.contains_key(c) {
                curr = curr.branches.get_mut(c).unwrap();
            } else {
                let branch = Branch {
                    leaves: vec![],
                    branches: HashMap::new(),
                };

                curr.branches.insert(*c, branch);
                curr = curr.branches.get_mut(c).unwrap();
            }
        }
    }

    root
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
        tag_operator                   "#"    [Infix]
        value_equality_operator        "=="   [Between]
        reference_equality_operator    "==="  [Between]
        value_inequality_operator      "!="   [Between]
        reference_inequality_operator  "!=="  [Between]
        single_or_operator             "|"    [Infix]
        double_or_operator             "||"   [Between]
        single_and_operator            "&"    [Infix]
        double_and_operator            "&&"   [Between];
    Flow:
        if_missing_operator            "??"   [Between]
        if_exists_operator             "!!"   [Between]
        if_truthy_operator             "?:"   [Between]
        if_falsy_operator              "!:"   [Between]
        each_operator                  "*"    [Between];
    Logical:
        falsy_prefix                   "!"    [Prefix]
        truthy_prefix                  "?"    [Prefix]
        spread_operator                "..."  [Prefix]
        single_add_operator            "+"    [Spaced]
        double_add_operator            "++"   [Infix]
        single_dash_operator           "-"    [Spaced]
        double_dash_operator           "--"   [Infix]
        single_times_operator          "*"    [Spaced]
        double_times_operator          "**"   [Infix]
        single_div_operator            "/"    [Spaced]
        double_div_operator            "//"   [Infix];
    Caller:
        single_arg_caller              ":"    [Infix];
    Assigner:
        mutable_field_assigner         ":"    [Suffix, Spaced]
        constant_field_assigner        "::"   [Suffix, Spaced]
        final_field_assigner           ":::"  [Suffix, Spaced]
        mutable_proc_assigner          ">>"   [Between]
        constant_proc_assigner         ":>>"  [Between]
        final_proc_assigner            "::>>" [Between]
        mutable_var_assigner            "~="  [Between]
        constant_var_assigner           "="   [Between]
        mutable_func_assigner           "~>"  [Between]
        constant_func_assigner          "=>"  [Between]
        final_func_assigner             "==>" [Between];

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

use super::Cursor;
