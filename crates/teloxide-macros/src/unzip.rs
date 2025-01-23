pub(crate) struct Unzip<A, B>(pub A, pub B);

impl<A, B, T, U> FromIterator<(T, U)> for Unzip<A, B>
where
    A: Default + Extend<T>,
    B: Default + Extend<U>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        let (mut a, mut b): (A, B) = Default::default();

        for (t, u) in iter {
            a.extend([t]);
            b.extend([u]);
        }

        Unzip(a, b)
    }
}

pub(crate) struct Unzip3<A, B, C>(pub A, pub B, pub C);

impl<A, B, C, T, U, V> FromIterator<(T, U, V)> for Unzip3<A, B, C>
where
    A: Default + Extend<T>,
    B: Default + Extend<U>,
    C: Default + Extend<V>,
{
    fn from_iter<I: IntoIterator<Item = (T, U, V)>>(iter: I) -> Self {
        let (mut a, mut b, mut c): (A, B, C) = Default::default();

        for (t, u, v) in iter {
            a.extend([t]);
            b.extend([u]);
            c.extend([v]);
        }

        Unzip3(a, b, c)
    }
}
