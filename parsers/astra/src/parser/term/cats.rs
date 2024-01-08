use std::collections::HashSet;

use super::Type;

pub trait Category {
    #[allow(non_snake_case)]
    fn New() -> Self
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
            fn New() -> Self {
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
        Key,
        Number,
        Delimited,
    ]
}

cat! {Operator
    for Operators [
        Unknown,
        Between(Between): Unknown (as Betweens),
        Spaced,
        Chained,
        Prefix(Prefix): Unknown (as Prefixes),
        Suffix(Suffix): Unknown (as Suffixes),
    ]
}

cat! {Between
    in Operator
    for Betweens [
        Unknown,
        Pipe, // ;;
        MutableVarAssigner, // ~
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
    ]
}

cat! {Prefix
    in Operator
    for Prefixes [
        Unknown,
        Alias, // |
        Input, // >
        Output, // >>
    ]
}

cat! {Delimiter
    for Delimiters [
        MapStart,
        MapEnd,
        ArrayStart,
        ArrayEnd,
        GroupStart,
        GroupEnd,
        GenericStart,
        GenericEnd,
        EntrySeperator, // ,
        ExpressionTerminator, // ;
    ]
}

cat! {Whitespace
    for Whitespaces [
        Indent
        Dedent
    ]
}
