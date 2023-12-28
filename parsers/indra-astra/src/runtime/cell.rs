// use std::{
//     borrow::{Borrow, BorrowMut},
//     marker::PhantomData,
//     mem::transmute,
//     sync::MutexGuard,
// };

// use slotmap::{DefaultKey, Key};

// use super::{
//     nodes::{Any, Entry, Node, Structure, Value},
//     Runtime,
// };

// pub type Id = DefaultKey;
// pub type Opt<T> = Option<T>;
// pub type Src<T> = Source<T>;
// pub type Rfr<T> = Reference<T>;

// pub struct Source<T: Node + ?Sized> {
//     __: PhantomData<T>,
//     id: Id,
// }

// pub struct Reference<T: Node> {
//     __: PhantomData<T>,
//     id: Id,
// }

// pub enum Cell<T: Node> {
//     Src(Source<T>),
//     Rfr(Reference<T>),
// }

// impl<T: Node> Cell<T> {
//     pub fn id(&self) -> Id {
//         match self {
//             Self::Src(src) => src.id,
//             Self::Rfr(rfr) => rfr.id,
//         }
//     }

//     pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
//         match self {
//             Self::Src(src) => src.get(rt),
//             Self::Rfr(rfr) => rfr.get(rt),
//         }
//     }
// }

// impl<T: Node> Source<T> {
//     #[allow(non_snake_case)]
//     pub(crate) fn Empty() -> Self {
//         Self {
//             __: PhantomData,
//             id: Id::null(),
//         }
//     }

//     #[allow(non_snake_case)]
//     pub fn Of(value: T, rt: &mut Runtime) -> Self {
//         let id = rt._add_node(value.as_any());
//         Self {
//             __: PhantomData,
//             id,
//         }
//     }

//     #[allow(non_snake_case)]
//     pub(crate) fn As<F>(source: &Source<T>) -> Source<F>
//     where
//         F: Node + ?Sized,
//     {
//         Source {
//             __: PhantomData,
//             id: source.id,
//         }
//     }

//     pub fn id(&self) -> Id {
//         self.id
//     }

//     pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
//         let node = rt._get_node(self.id());
//         node.lock().unwrap()
//     }
// }

// impl Source<Entry> {
//     pub fn get_value<'rt>(&'rt self, rt: &'rt Runtime) -> &Src<Value> {
//         Entry::Downcast(self.get(rt).borrow_mut()).value()
//     }
// }

// impl Source<Value> {
//     pub fn as_structure(&self, rt: &Runtime) -> &Src<Structure> {
//         let mut node = self.get(rt);
//         let value = Value::Unwrap(&mut node);

//         match value {
//             Value::Stx(_) => unsafe { transmute(self) },
//             _ => panic!("Expected Value::Stx"),
//         }
//     }

//     pub fn to_structure(&self, rt: &Runtime) -> Src<Structure> {
//         let mut node = self.get(rt);
//         let value = Value::Unwrap(&mut node);

//         match value {
//             Value::Stx(_) => Source::As::<Structure>(self),
//             _ => panic!("Expected Value::Stx"),
//         }
//     }
// }

// impl<T: Node> Reference<T> {
//     #[allow(non_snake_case)]
//     pub fn To(source: &Src<T>) -> Self {
//         Self {
//             __: PhantomData,
//             id: source.id,
//         }
//     }

//     pub fn id(&self) -> Id {
//         self.id
//     }

//     pub fn get<'rt>(&'rt self, rt: &'rt Runtime) -> MutexGuard<Any> {
//         let node = rt._get_node(self.id());
//         node.lock().unwrap()
//     }
// }
