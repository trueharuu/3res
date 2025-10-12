use std::fmt::Display;

pub trait Saturating {
    #[must_use]
    fn saturating_add(self, t: Self) -> Self;
    #[must_use]
    fn saturating_sub(self, t: Self) -> Self;
}

macro_rules! sat {
    ($t:ident) => {
        impl Saturating for $t {
            fn saturating_add(self, t: Self) -> Self {
                $t::saturating_add(self, t)
            }

            fn saturating_sub(self, t: Self) -> Self {
                $t::saturating_sub(self, t)
            }
        }
    };

    (many $($t:ident)*) => {
        $(sat! { $t })*
    }
}

sat!(many u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize);

pub trait IterJoin {
    fn join(self, sep: &str) -> String;
}

impl<I, T> IterJoin for I
where
    I: Iterator<Item = T>,
    T: Display,
{
    fn join(self, sep: &str) -> String {
        let mut iter = self.peekable();
        let mut out = String::new();
        while let Some(item) = iter.next() {
            out.push_str(&item.to_string());
            if iter.peek().is_some() {
                out.push_str(sep);
            }
        }
        out
    }
}
