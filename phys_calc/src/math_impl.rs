macro_rules! impl_math {
    ($struct_name:ident<$($generic:ident : $trait:path),+>) => {

        impl<$($generic),+> std::ops::Add for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                $struct_name {
                    inner: self.inner + rhs.inner,
                    ..self
                }
            }
        }

        impl<$($generic),+> std::ops::AddAssign for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            fn add_assign(&mut self, rhs: Self){
                *self = *self + rhs;
            }
        }

        impl<$($generic),+> std::ops::Sub for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                $struct_name {
                    inner: self.inner - rhs.inner,
                    ..self
                }
            }
        }

        impl<$($generic),+> std::ops::SubAssign for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        // impl<$($generic),+> std::ops::Mul<i64> for $struct_name <  $($generic),+ >
        // where $($generic : $trait),+
        // {
        //     type Output = Self;

        //     fn mul(self, rhs: i64) -> Self::Output {
        //         $struct_name {
        //             inner: self.inner * (rhs as number),
        //             ..self
        //         }
        //     }
        // }

        // impl<$($generic),+> std::ops::MulAssign<i64> for $struct_name <  $($generic),+ >
        // where $($generic : $trait),+
        // {
        //     fn mul_assign(&mut self, rhs: i64) {
        //         *self = *self * rhs;
        //     }
        // }

        impl<$($generic),+> std::ops::Mul<number> for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            type Output = Self;

            fn mul(self, rhs: number) -> Self::Output {
                $struct_name {
                    inner: self.inner * rhs,
                    ..self
                }
            }
        }

        impl<$($generic),+> std::ops::MulAssign<number> for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            fn mul_assign(&mut self, rhs: number) {
                *self = *self * rhs;
            }
        }

        impl<$($generic),+> std::ops::Div<number> for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            type Output = Self;

            fn div(self, rhs: number) -> Self::Output {
                $struct_name {
                    inner: self.inner / rhs,
                    ..self
                }
            }
        }

        impl<$($generic),+> std::ops::DivAssign<number> for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            fn div_assign(&mut self, rhs: number) {
                *self = *self / rhs;
            }
        }

        impl<$($generic),+> std::ops::Div for $struct_name <  $($generic),+ >
        where $($generic : $trait),+
        {
            type Output = number;

            fn div(self, rhs: Self) -> Self::Output {
                self.inner / rhs.inner
            }
        }

        // impl<$($generic),+> std::ops::Div<i64> for $struct_name <  $($generic),+ >
        // where $($generic : $trait),+
        // {
        //     type Output = Self;

        //     fn div(self, rhs: i64) -> Self::Output {
        //         $struct_name {
        //             inner: self.inner / (rhs as number),
        //             ..self
        //         }
        //     }
        // }

        // impl<$($generic),+> std::ops::DivAssign<i64> for $struct_name <  $($generic),+ >
        // where $($generic : $trait),+
        // {
        //     fn div_assign(&mut self, rhs: i64) {
        //         *self = *self / rhs;
        //     }
        // }

    }

}

pub(crate) use impl_math;
