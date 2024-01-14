use std::collections::HashSet;

use super::Type;

pub trait Category {
    #[allow(non_snake_case)]
    fn Get() -> Self
    where
        Self: Sized;

    fn has(&self, ttype: &Type) -> bool {
        self.all().contains(ttype) || self.subs().iter().any(|cat| cat.has(ttype))
    }

    fn all(&self) -> HashSet<Type>;

    fn any(&self) -> Type {
        Type::Ambiguous(self.all().into_iter().collect())
    }

    fn sups(&self) -> Vec<Box<dyn Category>> {
        vec![]
    }

    fn subs(&self) -> Vec<Box<dyn Category>> {
        vec![]
    }
}

macro_rules! cat_item {
    ($cat:ident, $(($sup:ident), )? $type:ident $(($arg:ident))? $(: $impl:ident)? $((as $sub:ident))? $(,)?) => {
        cat_item!(::
            $cat,
            $type,
            $($sup)?,
            $($arg)?,
            $($impl)?
        );
        $(pub type $sub = $sub;)?
    };
    (:: $cat:ident, $type:ident,,,) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$cat($cat::$type);
    };
    (:: $cat:ident, $type:ident,,,$impl:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$cat($cat::$type($type::$impl));
    };
    (:: $cat:ident, $type:ident, $sup:ident,,) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$sup($sup::$cat($cat::$type));
    };
    (:: $cat:ident, $type:ident, $sup:ident,,$impl:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$sup($sup::$cat($impl));
    };
}

macro_rules! _def_cat {
    ($cat:ident, $cats:ident, $($types:ident $(, $args:ident)?)*) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $cat {
            $($types$(($args))?,)*
        }

        #[derive(Default)]
        pub struct $cats;
    };
}

macro_rules! _impl_cat {
    ($cat:ident, $cats:ident, $($types:ident)* $(, $sup:ident)?) => {
        impl Category for $cats {
            #[allow(non_snake_case)]
            fn Get() -> Self {
                Self {}
            }


            fn all(&self) -> HashSet<Type> {
                let mut set = HashSet::new();
                $(set.insert($cats::$types);)*
                set
            }

            $(
                fn sups() -> Vec<Box<Category>> {
                    let mut set = HashSet::new();
                    set.insert($sup);
                    set
                }
            )?
        }
    };
}

macro_rules! cat {
    ($cat:ident for $cats:ident [$($types:ident $(($args:ident))? $(: $impl:ident)?  $((as $sub:ident))? $(,)?)*]) => {
        _def_cat!($cat, $cats, $($types $(, $args)?)*);

        impl $cats {
            $(cat_item!($cat, $types $(: $impl)? $((as $sub))?);)*
        }

        _impl_cat!($cat, $cats, $($types)*);
    };

    ($cat:ident in $sup:ident for $cats:ident [$($types:ident $(($args:ident))? $(: $impl:ident)? $((as $sub:ident))? $(,)?)*]) => {
        _def_cat!($cat, $cats, $($types $(, $args)?)*);

        impl $cats {
            $(cat_item!($cat, ($sup), $types $(: $impl)?) $(as $sub)?;)*
        }

        _impl_cat!($cat, $cats, $($types)*);
    };
}

macro_rules! _cat_subs {
    ($cats:ident, $sub:ident) => {
        impl $cats {}
    };
}

cat! {Word
    for Words [
        Whole,
        Delimited,
    ]
}

cat! {Operator
    for Operators [
        Unknown,
        Spaced(Spaced): Unknown (as Spaceds),
        Chained(Chained): Unknown (as Chaineds),
        Prefix(Prefix): Unknown (as Prefixes),
        Suffix(Suffix): Unknown (as Suffixes),
        // Spaced &| Chained
        Between(Between): Unknown (as Betweens),
        // Prefix &| Chained
        Lookup(Lookup): Unknown (as Lookups),
    ]
}

cat! {Spaced
    in Operator
    for Spaceds [
        Unknown,
        Or, // |
        TraitMod, // #
    ]
}

cat! {Chained
    in Operator
    for Chaineds [
        Unknown,
        Or, // ||
        Caller, // :
        Range, // ...
    ]
}

cat! {Between
    in Operator
    for Betweens [
        Unknown,
        Pipe, // ;;
        MutableVarAssigner, // ~=
        ConstVarAssigner,   // =
        ProcAssigner, // >>
        FuncAssigner, // =>
    ]
}

cat! {Suffix
    in Operator
    for Suffixes [
        Unknown,
        MutableFieldAssigner, // :
        ConstFieldAssigner,   // ::
        FinalFieldAssigner,   // :::
    ]
}

cat! {Prefix
    in Operator
    for Prefixes [
        Unknown,
        Tag, // #
        TagLiteral, // ##
        Alias, // |
        Input, // >
        Output, // >>
        Spread, // ...
        Arg, // :
        ArgLiteral, // ::
    ]
}

cat! {Lookup
    in Operator
    for Lookups [
        Unknown,
        Dot, // .
        Slash, // /
        Parent, // ..
        Tag, // .#
        Query, // .?
    ]
}

cat! {Delimiter
    for Delimiters [
        Start(Start): Unknown (as Starts),
        End(End): Unknown (as Ends),
        Separator(Separator): Unknown (as Separators),
        Line(Line): Unknown (as Lines),
    ]
}

cat! {Start
    in Delimiter
    for Starts [
        Unknown,
        Map,
        Array,
        Group,
        Generic,
        Comment,
    ]
}

cat! {End
    in Delimiter
    for Ends [
        Unknown,
        Map,
        Array,
        Group,
        Generic,
        Comment,
    ]
}

cat! {Separator
    in Delimiter
    for Separators [
        Unknown,
        Entry,
        Expression,
    ]
}

cat! {Line
    in Delimiter
    for Lines [
        Unknown,
        Input,
        Local,
        Comment,
        Doc,
        Section,
        Title,
    ]
}

cat! {Whitespace
    for Whitespaces [
        Indent
        Dedent
    ]
}
