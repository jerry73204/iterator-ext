use crate::{
    and_then::AndThen, map_err::MapErr, try_filter::TryFilter, try_filter_map::TryFilterMap,
    try_flat_map::TryFlatMap, try_flatten::TryFlatten,
};

/// An extension trait to [Iterator]
pub trait IteratorExt {
    /// Creates a fallible iterator that works like map, but flattens nested structure.
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

    /// Creates a fallible iterator that flattens nested structure.
    fn try_flatten<T, U, E>(self) -> TryFlatten<Self, T::IntoIter>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        T: IntoIterator<Item = Result<U, E>>,
    {
        TryFlatten {
            iter: Some(self),
            sub_iter: None,
        }
    }

    /// Creates a fallible iterator that filters items.
    fn try_filter<F, T, E>(self, f: F) -> TryFilter<Self, F>
    where
        Self: Sized + Iterator<Item = Result<T, E>>,
        F: FnMut(&T) -> Result<bool, E>,
    {
        TryFilter {
            iter: Some(self),
            f,
        }
    }

    /// Creates a fallible iterator that both filters and maps.
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

    /// Maps a `Result<T, E>` to `Result<T, F>` by applying a function to a contained Err value, leaving an Ok value untouched.
    fn map_err<F, Ein, Eout>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: FnMut(Ein) -> Eout,
    {
        MapErr { iter: self, f }
    }

    /// Takes a closure and creates a fallible iterator which calls that closure on each element.
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

impl<I> IteratorExt for I where I: Iterator {}
