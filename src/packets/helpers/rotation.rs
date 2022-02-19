use encde::{Decode, Encode};

/// The angle is encoded in 256th-turns
#[derive(Encode, Decode)]
pub struct Angle(u8);

impl Angle {
	const UNITS_PER_TURN: f32 = 256.0;
	const DEGREES_PER_TURN: f32 = 360.0;

	pub fn as_degrees(&self) -> f32 {
		f32::from(self.0) * (Self::DEGREES_PER_TURN / Self::UNITS_PER_TURN)
	}
	pub fn from_degrees(degrees: f32) -> Self {
		assert!(degrees.is_finite());
		let ratio = Self::UNITS_PER_TURN / Self::DEGREES_PER_TURN;
		let converted = degrees * ratio;
		let normalized = converted.rem_euclid(Self::UNITS_PER_TURN);
		debug_assert!(normalized.is_finite() && normalized > 0.0 && normalized < Self::UNITS_PER_TURN);
		Self(unsafe { normalized.to_int_unchecked() })
	}
}

#[derive(Encode, Decode)]
pub struct Rotation<T: Encode + Decode> {
	pub pitch: T,
	pub yaw: T,
}

pub type AngleRotation = Rotation<Angle>;
pub type F32Rotation = Rotation<f32>;
