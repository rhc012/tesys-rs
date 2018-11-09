/// This trait should be applied to structs that are to be used as the basis frame to a coordinates
/// system in the SkyCoordinate struct. 
///

use CoordinateTransform;
use SkyCoordinate;

pub trait Frame: Sized + Clone {

	fn new() -> Self;
}

pub trait CanTransformTo<T: Frame>: Clone {
	type From: Frame;

	fn transform(self) -> SkyCoordinate<T>;

	fn transform_to(&self, _target: T) -> CoordinateTransform<Self::From, T> where
	Self::From: Frame, T: Frame, Self: Clone;
}

// Import the reference frame stbibructs from the submodule.
pub use frames::icrs::ICRS;
pub use frames::fk5::FK5;