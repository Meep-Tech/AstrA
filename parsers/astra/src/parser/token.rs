use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::LazyLock,
};

static _EMPTY_KEYS: LazyLock<HashMap<String, usize>> = LazyLock::new(|| HashMap::new());
// static mut _ALL_CATS: LazyLock<Mutex<HashMap<TypeId, Box<dyn Any>>>> =
//     LazyLock::new(|| Mutex::new(HashMap::new()));

// pub struct Cats;
// impl Cats {
//     #[allow(non_snake_case)]
//     fn Get<TCategory>() -> &'static TCategory
//     where
//         TCategory: Category,
//     {
//         let type_id = TypeId::of::<TCategory>();
//         let cat = unsafe { &_ALL_CATS }.try_lock().unwrap().get(&type_id);

//         if let Some(cat) = cat {
//             return cat.downcast_ref::<TCategory>().unwrap();
//         } else {
//             let cat = Box::new(TCategory::New());
//             unsafe { &_ALL_CATS }.lock().unwrap().insert(type_id, cat);
//             return Self::Get::<TCategory>();
//         }
//     }
// }

pub trait Category {
    #[allow(non_snake_case)]
    fn New() -> Self
    where
        Self: Sized;

    fn has(&self, ttype: &Type) -> bool {
        self.all().contains(ttype)
            || self.subs().iter().any(|cat| cat.has(ttype))
            || self.sups().iter().any(|cat| cat.has(ttype))
    }

    fn all(&self) -> HashSet<Type>;

    fn subs(&self) -> Vec<Box<dyn Category>> {
        vec![]
    }

    fn any(&self) -> Type {
        Type::Ambiguous(self.all().into_iter().collect())
    }

    fn sups(&self) -> Vec<Box<dyn Category>> {
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    None,
    Ambiguous(Vec<Type>),
    Comment(Comment),
    Attribute(Attribute),
    Structure(Structure),
    Procedural(Procedural),
    Entry(Entry),
    Modifier(Modifier),
}

impl Type {
    pub type Comments = Comments;
    pub type Tags = Tags;
    pub type Aliases = Aliases;
    pub type Attributes = Attributes;
    pub type Structures = Structures;
    pub type Procedurals = Procedurals;
    pub type Entries = Entries;
    pub type Modifiers = Modifiers;
}

macro_rules! cat_item {
    ($cat:ident, $type:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$cat($cat::$type);
    };
    ($cat:ident, $type:ident: $impl:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$cat($cat::$type($type::$impl));
    };
    ($cat:ident, $source:ident, $type:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$source($source::$cat($cat::$type));
    };
    ($cat:ident, $source:ident, $type:ident: $impl:ident) => {
        #[allow(non_upper_case_globals)]
        pub const $type: Type = Type::$source($source::$cat($impl));
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
    ($cat:ident, $cats:ident, $($types:ident)* $(, $source:ident)?) => {
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
                fn sources() -> Vec<Box<Category>> {
                    let mut set = HashSet::new();
                    set.insert($source);
                    set
                }
            )?
        }
    };
}

macro_rules! cat {
    ($cat:ident for $cats:ident [$($types:ident $(($args:ident))? $(: $impl:ident)? $(,)?)*]) => {
        _def_cat!($cat, $cats, $($types $(, $args)?)*);

        impl $cats {
            $(cat_item!($cat, $types $(: $impl)?);)*
        }

        _impl_cat!($cat, $cats, $($types)*);
    };

    ($cat:ident in $source:ident for $cats:ident [$($types:ident $(($args:ident))? $(: $impl:ident)? $(,)?)*]) => {
        _def_cat!($cat, $cats, $($types $(, $args)?)*);

        impl $cats {
            $(cat_item!($cat, $source, $types $(: $impl)?);)*
        }

        _impl_cat!($cat, $cats, $($types)*);
    };
}

cat! {Comment
    for Comments [
        Block,
        Line,
        Doc,
        Region
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Comment {
//     Block,
//     Line,
//     Doc,
//     Region,
// }
// pub struct Comments;
// impl Category for Comments {
//     #[allow(non_snake_case)]
//     fn All() -> HashSet<Type> {
//         let mut set = HashSet::new();
//         set.insert(Comments::Block);
//         set.insert(Comments::Line);
//         set.insert(Comments::Doc);
//         set.insert(Comments::Region);
//         set
//     }
// }
// impl Comments {
//     #[allow(non_upper_case_globals)]
//     pub const Block: Type = Type::Comment(Comment::Block);
//     #[allow(non_upper_case_globals)]
//     pub const Line: Type = Type::Comment(Comment::Line);
//     #[allow(non_upper_case_globals)]
//     pub const Doc: Type = Type::Comment(Comment::Doc);
//     #[allow(non_upper_case_globals)]
//     pub const Region: Type = Type::Comment(Comment::Region);
// }

cat! {Attribute
    for Attributes [
        Tag(Tag): Own,
        Alias(Alias): Own,
        Input
    ]
}

impl Attributes {
    pub type Tags = Tags;
    pub type Aliases = Aliases;

    // #[allow(non_upper_case_globals)]
    // pub const Tag: Type = Type::Attribute(Attribute::Tag(Tag::Own));
    // #[allow(non_upper_case_globals)]
    // pub const Alias: Type = Type::Attribute(Attribute::Alias(Alias::Own));
    // #[allow(non_upper_case_globals)]
    // pub const Input: Type = Type::Attribute(Attribute::Input);
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Attribute {
//     Tag(Tag),
//     Alias(Alias),
//     Input,
// }
// pub struct Attributes;
// impl Category for Attributes {
//     #[allow(non_snake_case)]
//     fn All() -> HashSet<Type> {
//         let mut set = HashSet::new();
//         set.insert(Attributes::Tag);
//         set.insert(Attributes::Alias);
//         set.insert(Attributes::Input);
//         set
//     }
// }

cat! {Tag
    in Attribute
    for Tags [
        Own,
        Literal,
        Input,
        Output
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Tag {
//     Own,     // #etc
//     Literal, // ##etc
//     Input,   // >#etc
//     Output,  // >>#etc
// }
// pub struct Tags;
// impl Category for Tags {
//     #[allow(non_snake_case)]
//     fn All() -> HashSet<Type> {
//         let mut set = HashSet::new();
//         set.insert(Tags::Own);
//         set.insert(Tags::Literal);
//         set.insert(Tags::Input);
//         set.insert(Tags::Output);
//         set
//     }
// }
// impl Tags {
//     #[allow(non_upper_case_globals)]
//     pub const Own: Type = Type::Attribute(Attribute::Tag(Tag::Own));
//     #[allow(non_upper_case_globals)]
//     pub const Literal: Type = Type::Attribute(Attribute::Tag(Tag::Literal));
//     #[allow(non_upper_case_globals)]
//     pub const Input: Type = Type::Attribute(Attribute::Input);
//     #[allow(non_upper_case_globals)]
//     pub const Output: Type = Type::Attribute(Attribute::Input);
// }

cat! {Alias
    in Attribute
    for Aliases [
        Own,
        Input
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Alias {
//     Own,
//     Input,
// }
// pub struct Aliases;
// impl Category for Aliases {
//     #[allow(non_snake_case)]
//     fn All() -> HashSet<Type> {
//         let mut set = HashSet::new();
//         set.insert(Aliases::Own);
//         set.insert(Aliases::Input);
//         set
//     }
// }
// impl Aliases {
//     #[allow(non_upper_case_globals)]
//     pub const Own: Type = Type::Attribute(Attribute::Alias(Alias::Own));
//     #[allow(non_upper_case_globals)]
//     pub const Input: Type = Type::Attribute(Attribute::Alias(Alias::Input));
// }

cat! {Structure
    for Structures [
        Tree,
        List,
        Group,
        Array,
        Map
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Structure {
//     Tree,
//     List,
//     Group,
//     Array,
//     Map,
// }
// pub struct Structures;
// impl Category for Structures {}
// impl Structures {
//     #[allow(non_upper_case_globals)]
//     pub const Tree: Type = Type::Structures(Structure::Tree);
//     #[allow(non_upper_case_globals)]
//     pub const List: Type = Type::Structures(Structure::List);
//     #[allow(non_upper_case_globals)]
//     pub const Group: Type = Type::Structures(Structure::Group);
//     #[allow(non_upper_case_globals)]
//     pub const Array: Type = Type::Structures(Structure::Array);
//     #[allow(non_upper_case_globals)]
//     pub const Map: Type = Type::Structures(Structure::Map);
// }

cat! {Procedural
    for Procedurals [
        Anonymous,
        Prototype,
        Function
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Procedural {
//     Anonymous,
//     Prototype,
//     Function,
// }
// pub struct Procedurals;
// impl Category for Procedurals {}
// impl Procedurals {
//     #[allow(non_upper_case_globals)]
//     pub const Anonymous: Type = Type::Procedural(Procedural::Anonymous);
//     #[allow(non_upper_case_globals)]
//     pub const Prototype: Type = Type::Procedural(Procedural::Prototype);
//     #[allow(non_upper_case_globals)]
//     pub const Function: Type = Type::Procedural(Procedural::Function);
// }

cat! {Entry
    for Entries [
        Named,
        Ordered,
        Hybrid,
        Anonymous,
        Empty
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Entry {
//     Named,
//     Ordered,
//     Hybrid,
//     Anonymous,
//     Empty,
// }
// pub struct Entries;
// impl Category for Entries {}
// impl Entries {
//     #[allow(non_upper_case_globals)]
//     pub const Named: Type = Type::Entry(Entry::Named);
//     #[allow(non_upper_case_globals)]
//     pub const Ordered: Type = Type::Entry(Entry::Ordered);
//     #[allow(non_upper_case_globals)]
//     pub const Hybrid: Type = Type::Entry(Entry::Hybrid);
//     #[allow(non_upper_case_globals)]
//     pub const Anonymous: Type = Type::Entry(Entry::Anonymous);
//     #[allow(non_upper_case_globals)]
//     pub const Empty: Type = Type::Entry(Entry::Empty);
//     pub fn discriminant(&self) -> u8 {
//         unsafe { *<*const _>::from(self).cast::<u8>() }
//     }
// }

cat! {Modifier
    for Modifiers [
        LinePrefix,
        KeyPrefix,
        KeySuffix,
        AssignerPrefix,
        AssignerSuffix
    ]
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum Modifier {
//     LinePrefix,
//     KeyPrefix,
//     KeySuffix,
//     AssignerPrefix,
//     AssignerSuffix,
// }
// pub struct Modifiers;
// impl Category for Modifiers {}
// impl Modifiers {
//     #[allow(non_upper_case_globals)]
//     pub const LinePrefix: Type = Type::Modifier(Modifier::LinePrefix);
//     #[allow(non_upper_case_globals)]
//     pub const KeyPrefix: Type = Type::Modifier(Modifier::KeyPrefix);
//     #[allow(non_upper_case_globals)]
//     pub const KeySuffix: Type = Type::Modifier(Modifier::KeySuffix);
//     #[allow(non_upper_case_globals)]
//     pub const AssignerPrefix: Type = Type::Modifier(Modifier::AssignerPrefix);
//     #[allow(non_upper_case_globals)]
//     pub const AssignerSuffix: Type = Type::Modifier(Modifier::AssignerSuffix);
// }

// macro_rules! token_of_type {
//     (
//         $token:ident:
//         $type:path
//     ) => {
//         if let $type(_) = $token.ttype {
//             true
//         } else {
//             false
//         }
//     };
// }
// pub(crate) use token_of_type;

// macro_rules! token_is_type {
//     (
//         $token:ident:
//         $type:path
//     ) => {
//         if let $type = $token.ttype {
//             true
//         } else {
//             false
//         }
//     };
// }
// pub(crate) use token_is_type;

pub struct Token {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
    pub children: Vec<Token>,
    pub errors: Vec<Error>,
    pub keys: HashMap<String, usize>,
}

impl Token {
    pub type Type = Type;
    //pub type Category = Category;

    #[allow(non_snake_case)]
    pub fn New(start: usize) -> Token {
        Token {
            ttype: Type::None,
            start,
            end: start,
            children: vec![],
            keys: HashMap::new(),
            errors: vec![],
        }
    }

    #[allow(non_snake_case)]
    pub fn Of_Type(ttype: Type, start: usize) -> Token {
        Token {
            ttype,
            start,
            end: start,
            children: vec![],
            keys: HashMap::new(),
            errors: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn has(&self, key: &str) -> bool {
        self.keys.contains_key(key)
    }

    pub fn is(&self, ttype: Type) -> bool {
        self.ttype == ttype
    }

    pub fn is_of<T>(&self) -> bool
    where
        T: Category + 'static,
    {
        T::New().all().contains(&self.ttype)
    }

    pub fn child(&mut self, index: usize) -> &Token {
        &mut self.children[index]
    }

    pub fn prop(&self, key: &str) -> Option<&Token> {
        if let Some(index) = self.keys.get(key) {
            return Some(&self.children[*index]);
        }

        None
    }

    pub fn push(&mut self, token: Token) {
        self.children.push(token);
    }

    pub fn set(&mut self, key: &str, token: Token) {
        if let Some(index) = self.keys.get(key) {
            self.children[*index] = token;
            return;
        } else {
            self.keys.insert(key.to_string(), self.children.len());
        }

        self.children.push(token);
    }

    pub fn end(mut self, end: usize) -> Self {
        self.end = end;
        self
    }
}

pub struct Error {
    etype: String,
    ttype: Type,
    index: usize,
    data: Vec<Vec<String>>,
}

impl<'e> Error {
    pub const INVALID_KEY: &'static str = "invalid_syntax";
    pub const UNEXPECTED_KEY: &'static str = "unexpected_syntax";
    pub const IN_CHILD_KEY: &'static str = "in_child";
    pub const IN_PROP_KEY: &'static str = "in_prop";

    #[allow(non_snake_case)]
    pub fn Unexpected(
        ttype: &Token::Type,
        index: usize,
        found: impl Display,
        expected: &[&str],
    ) -> Error {
        Error {
            etype: Error::UNEXPECTED_KEY.to_string(),
            index,
            ttype: ttype.clone(),
            data: vec![
                expected.iter().map(|e| e.to_string()).collect(),
                vec![found.to_string()],
            ],
        }
    }

    pub fn key(&self) -> &str {
        &self.etype
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn message(&self) -> String {
        let mut message = String::from(format!(
            "Error in {} @ {}: *{}*; ",
            format!("{:?}", self.ttype),
            self.index,
            self.etype,
        ));

        match self.etype.as_str() {
            Error::INVALID_KEY => {
                message.push_str(&self.data[0][0]);
            }
            Error::UNEXPECTED_KEY => {
                message.push_str(&format!(
                    "Found {}, Expected: {}",
                    self.data[1][0],
                    self.data[0].join(", "),
                ));
            }
            _ => panic!("unhandled error type"),
        }

        message
    }
}
