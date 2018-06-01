pub type PhantomData<T> = ::std::marker::PhantomData<T>;

pub trait Dispatch {
    type Call;
}

#[macro_export]
macro_rules! decl_issue_one {
    (
        $( #[$attr:meta] )*
        pub enum $name: ident <$generic_name: ident : $generic_ty: ident>;
    ) => {
        $( #[$attr] )*
        pub enum $name<$generic_name: $generic_ty> {
            __PhantomData($crate::PhantomData<$generic_name>),
        }
    }
}

#[macro_export]
macro_rules! decl_issue_two {
    (
        $( #[$attr:meta] )*
        pub enum $name: ident {
            $(
                $call_name: ident,
            )+
        }
    ) => {
        $( #[$attr] )*
        pub enum $name {
            $(
                $call_name(<$call_name as $crate::Dispatch>::Call),
            )+
        }
    }
}