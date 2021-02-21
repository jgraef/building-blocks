//! Lattice map access traits implemented for functions and closures.
//!
//! This is particularly useful for sampling from signed-distance fields.
//!
//! ```
//! use building_blocks_core::prelude::*;
//! use building_blocks_storage::prelude::*;
//!
//! let sample_extent = Extent3i::from_min_and_max(Point3i::fill(-15), Point3i::fill(15));
//! let mut sampled_sphere = Array3::fill(sample_extent, 0.0);
//!
//! copy_extent(&sample_extent, &|p: Point3i| (p.dot(p) - 10) as f32, &mut sampled_sphere);
//!```

use crate::{ForEach, Get, ReadExtent};

use building_blocks_core::prelude::*;

use core::iter::{once, Once};

impl<F, T, Coord> Get<Coord> for F
where
    F: Fn(Coord) -> T,
{
    type Data = T;

    fn get(&self, c: Coord) -> T {
        (self)(c)
    }
}

impl<'a, F, N, T> ForEach<N, PointN<N>> for F
where
    F: Fn(PointN<N>) -> T,
    PointN<N>: IntegerPoint<N>,
{
    type Data = T;

    fn for_each(&self, extent: &ExtentN<N>, mut f: impl FnMut(PointN<N>, Self::Data)) {
        for p in extent.iter_points() {
            f(p, (self)(p))
        }
    }
}

impl<'a, F, N, T> ReadExtent<'a, N> for F
where
    F: 'a + Fn(PointN<N>) -> T,
    PointN<N>: IntegerPoint<N>,
{
    type Src = &'a Self;
    type SrcIter = Once<(ExtentN<N>, Self::Src)>;

    fn read_extent(&'a self, extent: &ExtentN<N>) -> Self::SrcIter {
        once((*extent, self))
    }
}

// ████████╗███████╗███████╗████████╗
// ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
//    ██║   █████╗  ███████╗   ██║
//    ██║   ██╔══╝  ╚════██║   ██║
//    ██║   ███████╗███████║   ██║
//    ╚═╝   ╚══════╝╚══════╝   ╚═╝

#[cfg(test)]
mod test {
    use super::*;
    use crate::{copy_extent, Array3};

    #[test]
    fn copy_extent_from_func() {
        let extent = Extent3i::from_min_and_shape(Point3i::ZERO, Point3i::fill(5));
        let mut array = Array3::fill(extent, 0);

        copy_extent(&extent, &|_p| 1, &mut array);
    }
}
