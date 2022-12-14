use crate::{
    common::Real,
    geometries::{bound_by_box::BoundByBox, bounding_volume::BoundingVolume, intersectable::Intersectable, ray::Ray},
    math::{vector::Vector, vector3::Vector3},
};

/// Represents an AABB (Axis Aligned Bounding Box) that encloses points or geometry.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// The minimum extent of the bounding box.
    pub minimum: Vector3,

    /// The maximum extent of the bounding box.
    pub maximum: Vector3,
}

impl BoundingBox {
    /// Creates a new [`BoundingBox`] with a specific minimum and maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// assert_eq!(minimum, bounding_box.minimum);
    /// assert_eq!(maximum, bounding_box.maximum);
    /// ```
    pub fn new(minimum: &Vector3, maximum: &Vector3) -> BoundingBox {
        Self {
            minimum: *minimum,
            maximum: *maximum,
        }
    }

    /// Creates a new [`BoundingBox`] with the minimum set to negative infinity and the maximum set to positive
    /// infinity. This would encompass all points and geometry in all space.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let bounding_box = BoundingBox::new_infinity();
    ///
    /// assert_eq!(-Vector3::new_infinity(), bounding_box.minimum);
    /// assert_eq!(Vector3::new_infinity(), bounding_box.maximum);
    /// ```
    pub fn new_infinity() -> BoundingBox {
        Self::new(&-Vector3::new_infinity(), &Vector3::new_infinity())
    }

    /// Creates a new [`BoundingBox`] with the minimum set to positive infinity and the maximum set to negative
    /// infinity. This would encompass no points or geometry.
    ///
    /// This is useful for starting with a bounding box that doesn't contain anything so you can union points and
    /// geometry into it allowing it to grow to fit its contents.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let bounding_box = BoundingBox::new_inverse_infinity();
    ///
    /// assert_eq!(Vector3::new_infinity(), bounding_box.minimum);
    /// assert_eq!(-Vector3::new_infinity(), bounding_box.maximum);
    /// ```
    pub fn new_inverse_infinity() -> BoundingBox {
        Self::new(&Vector3::new_infinity(), &-Vector3::new_infinity())
    }

    /// Creates a new [`BoundingBox`] that exactly contains all of the points provided.
    ///
    /// Due to floating point rounding errors, it is often a good idea to add margin using the
    /// [`BoundingBox::add_margin()`] method to ensure tests against the bounding box will succeed even in the case of
    /// small floating point errors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let points = vec![
    ///     Vector3::from_value(0.0),
    ///     Vector3::from_value(-3.0),
    ///     Vector3::from_value(7.0),
    ///     Vector3::from_value(2.0),
    /// ];
    ///
    /// let bounding_box = BoundingBox::from_points(points.iter());
    ///
    /// assert_eq!(Vector3::from_value(-3.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(7.0), bounding_box.maximum);
    /// ```
    pub fn from_points<'a>(points: impl Iterator<Item = &'a Vector3>) -> BoundingBox {
        let mut minimum = Vector3::new_infinity();
        let mut maximum = -Vector3::new_infinity();

        for point in points {
            minimum = Vector3::min(&minimum, point);
            maximum = Vector3::max(&maximum, point);
        }

        BoundingBox::new(&minimum, &maximum)
    }

    pub fn from_geometries<'a>(geometries: impl Iterator<Item = &'a dyn Intersectable>) -> BoundingBox {
        let mut bounding_box = BoundingBox::new_inverse_infinity();

        for geometry in geometries {
            bounding_box.add_bounding_box(&geometry.calculate_bounding_box());
        }

        bounding_box
    }

    /// Increases the [`BoundingBox`] by the specified margin amount. Returns a mutable reference to itself for easy
    /// chaining of method calls.
    ///
    /// The operation performed is that the minimum is decreased by the margin amount and the maximum is increased by
    /// the margin amount.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let margin = Vector3::from_value(5.0);
    /// bounding_box.add_margin(&margin);
    ///
    /// assert_eq!(Vector3::from_value(-7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(8.0), bounding_box.maximum);
    /// ```
    pub fn add_margin(&mut self, margin_amount: &Vector3) -> &mut BoundingBox {
        self.minimum -= margin_amount;
        self.maximum += margin_amount;

        self
    }

    /// Increase the [`BoundingBox`] dimensions so that it contains the provided point. If the point is already
    /// contained within the [`BoundingBox`] is not changed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let point = Vector3::new(0.0, 9.0, -7.0);
    /// bounding_box.add_point(&point);
    ///
    /// assert_eq!(Vector3::new(-2.0, -2.0, -7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::new(3.0, 9.0, 3.0), bounding_box.maximum);
    /// ```
    pub fn add_point(&mut self, point: &Vector3) -> &mut BoundingBox {
        self.minimum = Vector3::min(point, &self.minimum);
        self.maximum = Vector3::max(point, &self.maximum);

        self
    }

    /// Increases the [`BoundingBox`] dimensions so that it contains the provided bounding box.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let mut bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let other_bounding_box = BoundingBox::new(&Vector3::from_value(-7.0), &Vector3::from_value(9.0));
    /// bounding_box.add_bounding_box(&other_bounding_box);
    ///
    /// assert_eq!(Vector3::from_value(-7.0), bounding_box.minimum);
    /// assert_eq!(Vector3::from_value(9.0), bounding_box.maximum);
    /// ```
    pub fn add_bounding_box(&mut self, other_bounding_box: &BoundingBox) -> &mut BoundingBox {
        self.minimum = Vector3::min(&other_bounding_box.minimum, &self.minimum);
        self.maximum = Vector3::max(&other_bounding_box.maximum, &self.maximum);

        self
    }

    /// Determines if the given point is contained within the [`BoundingBox`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let point_inside = Vector3::new(2.0, -1.0, 0.0);
    /// let point_outside = Vector3::new(3.0, 5.0, -6.0);
    ///
    /// assert_eq!(true, bounding_box.contains_point(&point_inside));
    /// assert_eq!(false, bounding_box.contains_point(&point_outside));
    /// ```
    pub fn contains_point(&self, point: &Vector3) -> bool {
        point.x >= self.minimum.x
            && point.y >= self.minimum.y
            && point.z >= self.minimum.z
            && point.x <= self.maximum.x
            && point.y <= self.maximum.y
            && point.z <= self.maximum.z
    }

    /// Determines if the given [`BoundingBox`] is contained within the [`BoundingBox`]. The other ['BoundingBox'] must
    /// be completely inside of the [`BoundingBox`].
    ///
    /// The operations performed are greater than or equal to and less than or equal to. So if the edges are exactly
    /// aligned with each other, it will still count as being contained. However, floating point errors could cause two
    /// edges that you would think should be aligned to not be exactly aligned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let bounding_box_inside = BoundingBox::new(&Vector3::from_value(-1.0), &Vector3::from_value(2.0));
    /// let bounding_box_overlapping = BoundingBox::new(&Vector3::from_value(-1.0), &Vector3::from_value(7.0));
    /// let bounding_box_outside = BoundingBox::new(&Vector3::from_value(4.0), &Vector3::from_value(7.0));
    ///
    /// assert_eq!(true, bounding_box.contains_bounding_box(&bounding_box_inside));
    /// assert_eq!(false, bounding_box.contains_bounding_box(&bounding_box_overlapping));
    /// assert_eq!(false, bounding_box.contains_bounding_box(&bounding_box_outside));
    /// ```
    pub fn contains_bounding_box(&self, bounding_box: &BoundingBox) -> bool {
        bounding_box.minimum.x >= self.minimum.x
            && bounding_box.minimum.x <= self.maximum.x
            && bounding_box.maximum.x >= self.minimum.x
            && bounding_box.maximum.x <= self.maximum.x
            && bounding_box.minimum.y >= self.minimum.y
            && bounding_box.minimum.y <= self.maximum.y
            && bounding_box.maximum.y >= self.minimum.y
            && bounding_box.maximum.y <= self.maximum.y
            && bounding_box.minimum.z >= self.minimum.z
            && bounding_box.minimum.z <= self.maximum.z
            && bounding_box.maximum.z >= self.minimum.z
            && bounding_box.maximum.z <= self.maximum.z
    }

    /// Determines if the given [`BoundingBox`] intersects either fully or partially with the [`BoundingBox`]. This
    /// method will return true if the other [`BoundingBox`] is completely contained within the [`BoundingBox`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let bounding_box_overlapping = BoundingBox::new(&Vector3::from_value(-1.0), &Vector3::from_value(7.0));
    /// let bounding_box_not_overlapping = BoundingBox::new(&Vector3::from_value(4.0), &Vector3::from_value(7.0));
    ///
    /// assert_eq!(true, bounding_box.overlaps_bounding_box(&bounding_box_overlapping));
    /// assert_eq!(false, bounding_box.overlaps_bounding_box(&bounding_box_not_overlapping));
    /// ```
    pub fn overlaps_bounding_box(&self, bounding_box: &BoundingBox) -> bool {
        bounding_box.minimum.x <= self.maximum.x
            && bounding_box.maximum.x >= self.minimum.x
            && bounding_box.minimum.y <= self.maximum.y
            && bounding_box.maximum.y >= self.minimum.y
            && bounding_box.minimum.z <= self.maximum.z
            && bounding_box.maximum.z >= self.minimum.z
    }

    /// Calculates the point that is in the center of the [`BoundingBox`]. If any of the dimensions of the bounding box
    /// are infinity or nan, the result is undefined but guarnateed to succeed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(3.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    /// let center_point = bounding_box.calculate_center_point();
    ///
    /// assert_eq!(Vector3::from_value(0.5), center_point);
    /// ```
    pub fn calculate_center_point(&self) -> Vector3 {
        self.minimum + (self.maximum - self.minimum) * 0.5
    }

    /// Determines if a [`Ray`] intersects with the [`BoundingBox`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// # use yart::geometries::ray::Ray;
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(2.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let ray_hits = Ray::new(&Vector3::new(0.0, 0.0, -4.0), &Vector3::new(0.0, 0.0, 1.0));
    /// let ray_misses_facing_away = Ray::new(&Vector3::new(0.0, 0.0, -4.0), &Vector3::new(0.0, 0.0, -1.0));
    /// let ray_misses_completely = Ray::new(&Vector3::new(0.0, 7.0, -4.0), &Vector3::new(0.0, 0.0, 1.0));
    ///
    /// assert_eq!(true, bounding_box.ray_intersects(&ray_hits));
    /// assert_eq!(false, bounding_box.ray_intersects(&ray_misses_facing_away));
    /// assert_eq!(false, bounding_box.ray_intersects(&ray_misses_completely));
    /// ```
    pub fn ray_intersects(&self, ray: &Ray) -> bool {
        let min = Vector3::component_mul(&(self.minimum - ray.position()), ray.inverse_direction());
        let max = Vector3::component_mul(&(self.maximum - ray.position()), ray.inverse_direction());

        let exit_distance = Real::min(
            Real::min(Real::max(min.x, max.x), Real::max(min.y, max.y)),
            Real::max(min.z, max.z),
        );

        let entrance_distance = Real::max(
            Real::max(Real::min(min.x, max.x), Real::min(min.y, max.y)),
            Real::min(min.z, max.z),
        );

        exit_distance >= 0.0 && entrance_distance <= exit_distance
    }
}

impl BoundingVolume for BoundingBox {
    /// Determines if a [`Ray`] intersects with the [`BoundingBox`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use yart::geometries::bounding_box::BoundingBox;
    /// # use yart::math::{vector::Vector, vector3::Vector3};
    /// # use yart::geometries::ray::Ray;
    /// #
    /// let minimum = Vector3::from_value(-2.0);
    /// let maximum = Vector3::from_value(2.0);
    ///
    /// let bounding_box = BoundingBox::new(&minimum, &maximum);
    ///
    /// let ray_hits = Ray::new(&Vector3::new(0.0, 0.0, -4.0), &Vector3::new(0.0, 0.0, 1.0));
    /// let ray_misses_facing_away = Ray::new(&Vector3::new(0.0, 0.0, -4.0), &Vector3::new(0.0, 0.0, -1.0));
    /// let ray_misses_completely = Ray::new(&Vector3::new(0.0, 7.0, -4.0), &Vector3::new(0.0, 0.0, 1.0));
    ///
    /// assert_eq!(true, bounding_box.ray_intersects(&ray_hits));
    /// assert_eq!(false, bounding_box.ray_intersects(&ray_misses_facing_away));
    /// assert_eq!(false, bounding_box.ray_intersects(&ray_misses_completely));
    /// ```
    fn ray_intersects(&self, ray: &Ray) -> bool {
        let min = Vector3::component_mul(&(self.minimum - ray.position()), ray.inverse_direction());
        let max = Vector3::component_mul(&(self.maximum - ray.position()), ray.inverse_direction());

        let exit_distance = Real::min(
            Real::min(Real::max(min.x, max.x), Real::max(min.y, max.y)),
            Real::max(min.z, max.z),
        );

        let entrance_distance = Real::max(
            Real::max(Real::min(min.x, max.x), Real::min(min.y, max.y)),
            Real::min(min.z, max.z),
        );

        exit_distance >= 0.0 && entrance_distance <= exit_distance
    }
}

impl BoundByBox for BoundingBox {
    fn calculate_bounding_box(&self) -> BoundingBox {
        *self
    }
}
