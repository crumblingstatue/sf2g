use crate::{graphics::Transform, system::Vector2f};

/// Decomposed transform defined by a position, a rotation and a scale.
pub trait Transformable {
    /// Sets the position of the object.
    ///
    /// This function completely overwrites the previous position.
    /// See [`move_`](Self::move_) to apply an offset based on the previous position instead.
    /// The default position of a transformable object is (0, 0).
    fn set_position<P: Into<Vector2f>>(&mut self, position: P);
    /// Set the orientation of the object in degrees.
    ///
    /// This function completely overwrites the previous rotation.
    /// See the rotate function to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable object is 0.
    fn set_rotation(&mut self, angle: f32);
    /// Sets the scale factors of the object.
    ///
    /// This function completely overwrites the previous scale.
    /// See the scale function to add a factor based on the previous scale instead.
    /// The default scale of a transformable object is (1, 1).
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S);
    /// Sets the local origin of the object.
    ///
    /// The origin of an object defines the center point for all transformations
    /// (position, scale, rotation).
    /// The coordinates of this point must be relative to the top-left corner of the object,
    /// and ignore all transformations (position, scale, rotation).
    /// The default origin of a transformable object is (0, 0).
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O);
    /// Gets the position of the object.
    fn position(&self) -> Vector2f;
    /// Gets the rotation of the object in degrees.
    ///
    /// The rotation is always in the range [0, 360].
    fn rotation(&self) -> f32;
    /// Gets the current scale of the object.
    fn get_scale(&self) -> Vector2f;
    /// Gets the local origin of the object.
    fn origin(&self) -> Vector2f;
    /// Moves the object by a given offset.
    ///
    /// This function adds to the current position of the object,
    /// unlike [`set_position`](Self::set_position) which overwrites it.
    fn move_<O: Into<Vector2f>>(&mut self, offset: O);
    /// Rotates the object.
    ///
    /// This function adds to the current rotation of the object, unlike
    /// [`set_rotation`](Self::set_rotation), which overwrites it.
    fn rotate(&mut self, angle: f32);
    /// Scales the object.
    ///
    /// This function multiplies the current scale of the object, unlike
    /// [`set_scale`](Self::set_scale), which overwrites it.
    fn scale<F: Into<Vector2f>>(&mut self, factors: F);
    /// Gets the combined transform of the object.
    fn transform(&self) -> &Transform;
    /// Gets the inverse combined transform of the object.
    fn inverse_transform(&self) -> &Transform;
}

#[derive(Debug, Clone, Copy)]
#[cfg(feature = "text")]
pub struct TransformableData {
    pub origin: [f32; 2],
    pub position: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
}

#[cfg(feature = "text")]
impl Default for TransformableData {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            position: Default::default(),
            rotation: Default::default(),
            scale: [1., 1.],
        }
    }
}

#[cfg(feature = "text")]
impl TransformableData {
    pub fn get(&self) -> Transform {
        let angle = -self.rotation * std::f32::consts::PI / 180.0;
        let cosine = angle.cos();
        let sine = angle.sin();
        let sxc = self.scale[0] * cosine;
        let syc = self.scale[1] * cosine;
        let sxs = self.scale[0] * sine;
        let sys = self.scale[1] * sine;
        let tx = -self.origin[0] * sxc - self.origin[1] * sys + self.position[0];
        let ty = self.origin[0] * sxs - self.origin[1] * syc + self.position[1];
        Transform::new(sxc, sys, tx, -sxs, syc, ty, 0.0, 0.0, 1.0)
    }
}
