use downcast::*;
use std::any::*;

struct Person {
    name: &'static str
}

fn main() {

    let x: Vec<&dyn Any> = vec![&0i32, &true, &"Text", &Person { name: "Bob"}];

    // С помощью std::any
    // Если передать тип Any, вернется Any

    fn get_type_info<T: Any>(t: T) -> (String, TypeId) {
        (type_name_of_val(&t).to_owned(), TypeId::of::<T>())
    }
    
    for v in x.clone() {
        let (type_name, type_id) = get_type_info(v);
        println!("{:?} type_name = {} type_id = {:?}", v, type_name, type_id);
    }

    println!();

    // Перевод объектов сначала в Any, потом возвращение в конкретный тип

    for v in x.clone() {
        if let Some((type_name, type_id)) = v.try_get_type() {
            println!("{:?} type_name = {} type_id = {:?}", v, type_name, type_id);
        } else {
            println!("Can't recognize");
        }
    }

    println!();

    // Перевод объектов сначала в Any, потом возвращение в конкретный тип
    // С использованием пользовательских типов (Person)

    let mut downcaster = Downcaster::new();
    downcaster.push(SelfType::<i32>::default());
    downcaster.push(SelfType::<&str>::default());
    downcaster.push(SelfType::<bool>::default());
    downcaster.push(SelfType::<Person>::default());

    for v in x.clone() {
        if let Some((type_name, type_id)) = v.try_get_type_from(&downcaster) {
            println!("{:?} type_name = {} type_id = {:?}", v, type_name, type_id);
        } else {
            println!("Can't recognize");
        }
    }
}

mod downcast {
    use std::any::*;
    use std::marker::PhantomData;

    pub use downcaster::Downcaster;

    trait SomeType {
        fn get_type_id(&self) -> TypeId;
        fn get_type_name(&self) -> &str;
        fn is_downcasted<'a>(&self, t: &'a dyn Any) -> bool;
    }

    #[derive(Clone, Copy)]
    pub struct SelfType<T> {
        t: PhantomData<T>
    }

    impl<T> SelfType<T> {
        pub const fn default() -> Self {
            SelfType::<T> { t: PhantomData }
        }   
    }

    impl<T: 'static> SomeType for SelfType<T> {
        fn get_type_id(&self) -> TypeId {
            TypeId::of::<T>()
        }

        fn get_type_name(&self) -> &str {
            std::any::type_name::<T>()
        }

        fn is_downcasted<'a>(&self, t: &'a dyn Any) -> bool {
            if let Some(_) = t.downcast_ref::<T>() {
                return true;
            }
            return false;
        }
    }

    mod downcaster {
        use super::*;

        const STR: SelfType::<&str> = SelfType::<&str>::default();
        const STRING: SelfType::<String> = SelfType::<String>::default();
        const BOOL: SelfType::<bool> = SelfType::<bool>::default();
        const I_32: SelfType::<i32> = SelfType::<i32>::default();
        const I_64: SelfType::<i64> = SelfType::<i64>::default();

        pub struct Downcaster(Vec<Box<dyn SomeType>>);

        impl Downcaster {
            pub fn try_get_type(&self, t: &dyn Any) -> Option<(String, TypeId)> {
                for typ in &self.0 {
                    let type_id = typ.get_type_id();
                    let type_name = typ.get_type_name();

                    if typ.is_downcasted(t) {
                        return Some((type_name.to_owned(), type_id));
                    }
                }
                return None;
            }
        }

        impl Downcaster {
            pub fn default() -> Self {
                let t1 = Box::new(BOOL);
                let t2 = Box::new(STRING);
                let t3 = Box::new(STR);
                let t4 = Box::new(I_32);
                let t5 = Box::new(I_64);
                Downcaster(vec![t1, t2, t3, t4, t5])
            }

            pub fn new() -> Self {
                Downcaster(vec![])
            }

            pub fn push<T: 'static>(&mut self, t: SelfType<T>) {
                self.0.push(Box::new(t));
            }
        }
    }

    pub trait TryDowncast {
        fn try_get_type_from(&self, d: &Downcaster) -> Option<(String, TypeId)>;
        fn try_get_type(&self) -> Option<(String, TypeId)>;
    }

    impl TryDowncast for dyn Any {
        fn try_get_type_from(&self, d: &Downcaster) -> Option<(String, TypeId)> {
            d.try_get_type(self)
        }

        fn try_get_type(&self) -> Option<(String, TypeId)> {
            Self::try_get_type_from(self, &Downcaster::default())
        }
    }
}