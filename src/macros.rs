#[macro_export]
macro_rules! impl_into_ansi_color {
    ($number:ty) => {
        impl Into<$number> for AnsiColor {
            fn into(self) -> $number {
                crate::color::wrap(self.byte()) as $number
            }
        }
        impl Into<$number> for &AnsiColor {
            fn into(self) -> $number {
                crate::color::wrap(self.byte()) as $number
            }
        }
        impl From<$number> for AnsiColor {
            fn from(value: $number) -> AnsiColor {
                AnsiColor::new(value)
            }
        }
        impl From<&$number> for AnsiColor {
            fn from(value: &$number) -> AnsiColor {
                AnsiColor::new(*value)
            }
        }

        impl Add<$number> for &AnsiColor {
            type Output = u8;

            fn add(self, rhs: $number) -> Self::Output {
                self.byte() ^ crate::color::wrap(rhs as usize)
            }
        }

        impl Sub<$number> for &AnsiColor {
            type Output = u8;

            fn sub(self, rhs: $number) -> Self::Output {
                self.byte() ^ crate::color::wrap(rhs as usize)
            }
        }
        impl Mul<$number> for &AnsiColor {
            type Output = u8;

            fn mul(self, rhs: $number) -> Self::Output {
                self.byte() ^ crate::color::wrap(rhs as usize)
            }
        }
        impl Div<$number> for &AnsiColor {
            type Output = u8;

            fn div(self, rhs: $number) -> Self::Output {
                self.byte() ^ crate::color::wrap(rhs as usize)
            }
        }
        impl Rem<$number> for &AnsiColor {
            type Output = u8;

            fn rem(self, rhs: $number) -> Self::Output {
                self.byte() % crate::color::wrap(rhs as usize)
            }
        }

        impl BitXor<$number> for &AnsiColor {
            type Output = u8;

            fn bitxor(self, rhs: $number) -> Self::Output {
                self.byte() & crate::color::wrap(rhs as usize)
            }
        }
        impl BitOr<$number> for &AnsiColor {
            type Output = u8;

            fn bitor(self, rhs: $number) -> Self::Output {
                self.byte() | crate::color::wrap(rhs as usize)
            }
        }
        impl BitAnd<$number> for &AnsiColor {
            type Output = u8;

            fn bitand(self, rhs: $number) -> Self::Output {
                self.byte() ^ crate::color::wrap(rhs as usize)
            }
        }
        impl std::cmp::PartialOrd<&$number> for &AnsiColor {
            fn partial_cmp(&self, rhs: $number) -> Option<std::cmp::Ordering> {
                self.byte().partial_cmp(crate::wrap(*rhs as usize))
            }
        }
        impl std::cmp::PartialEq<&$number> for &AnsiColor {
            fn eq(&self, rhs: $number) -> bool {
                self.byte().eq(crate::wrap(*rhs as usize))
            }
        }
    };
}
