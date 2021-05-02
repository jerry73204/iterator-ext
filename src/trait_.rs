use crate::{
    and_then::AndThen, map_err::MapErr, try_filter_map::TryFilterMap, try_flat_map::TryFlatMap,
};

pub trait IteratorExt {
    fn try_flat_map<F, T, U, V, E>(self, f: F) -> TryFlatMap<Self, F, U::IntoIter>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<U, E>,
        U: IntoIterator<Item = Result<V, E>>;

    fn try_filter_map<F, T, U, E>(self, f: F) -> TryFilterMap<Self, F>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<Option<U>, E>;

    fn map_err<F, Ein, Eout>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Ein) -> Eout;

    fn and_then<F, T, U, E>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<U, E>;
}

impl<I> IteratorExt for I
where
    I: Iterator,
{
    fn try_flat_map<F, T, U, V, E>(self, f: F) -> TryFlatMap<Self, F, U::IntoIter>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<U, E>,
        U: IntoIterator<Item = Result<V, E>>,
    {
        TryFlatMap {
            iter: Some(self),
            sub_iter: None,
            f,
        }
    }

    fn try_filter_map<F, T, U, E>(self, f: F) -> TryFilterMap<Self, F>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<Option<U>, E>,
    {
        TryFilterMap {
            iter: Some(self),
            f,
        }
    }

    fn map_err<F, Ein, Eout>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Ein) -> Eout,
    {
        MapErr { iter: self, f }
    }

    fn and_then<F, T, U, E>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(T) -> Result<U, E>,
    {
        AndThen {
            iter: Some(self),
            f,
        }
    }
}
