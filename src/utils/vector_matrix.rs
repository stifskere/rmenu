use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_vector {
    ($trait:path { $($tt:tt)* }) => {
        impl<T> $trait for Vector2<T> where T:
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Div<Output = T> +
            Copy {
                $($tt)*
            }
    };

    ($($tt:tt)*) => {
        impl<T> Vector2<T> where T:
            Add<Output = T> +
            Sub<Output = T> +
            Mul<Output = T> +
            Div<Output = T> +
            Copy {
                $($tt)*
            }
    }
}

#[allow(unused)]
pub type Vector2F = Vector2<f32>;
#[allow(unused)]
pub type Vector2I = Vector2<i32>;
#[allow(unused)]
pub type Vector2U = Vector2<u32>;

#[derive(Clone, Copy, Debug)]
pub struct Vector2<T: Add + Sub + Mul + Div + Copy> {
    x: T,
    y: T,
}

impl_vector! {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub fn y(&self) -> T {
        self.y
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }
}

impl_vector!(From<(T, T)> {
    #[inline]
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
});

impl_vector!(Add {
    type Output = Vector2<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
});

impl_vector!(Sub {
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
});

impl_vector!(Mul {
    type Output = Vector2<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
});

impl_vector!(Div {
    type Output = Vector2<T>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
});

impl_vector!(Add<T> {
    type Output = Vector2<T>;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
});

impl_vector!(Sub<T> {
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
});

impl_vector!(Mul<T> {
    type Output = Vector2<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
});

impl_vector!(Div<T> {
    type Output = Vector2<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
});
