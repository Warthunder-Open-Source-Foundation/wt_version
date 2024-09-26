use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A version in the form of `2.33.5.14`
#[derive(Debug, Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash)]
pub struct Version {
	/// Pretty much always just 2
	global: u16,
	major: u16,
	minor: u16,
	patch: u16,
}

impl Version {
	pub fn new(global: u16, major: u16, minor: u16, patch: u16) -> Self {
		Self {
			global,
			major,
			minor,
			patch,
		}
	}

	pub fn to_u64(&self) -> u64 {
		((self.global as u64) << 48) | ((self.major as u64) << 32) | ((self.minor as u64) << 16) | (self.patch as u64)
	}

	pub fn from_u64(value: u64) -> Version {
		Version {
			global: ((value >> 48) & 0xFFFF) as u16,
			major: ((value >> 32) & 0xFFFF) as u16,
			minor: ((value >> 16) & 0xFFFF) as u16,
			patch: (value & 0xFFFF) as u16,
		}
	}
	pub fn is_valid(s: &str) -> bool {
		let s = s.split(".").collect::<Vec<_>>();
		if s.len() != 3 {
			return false;
		}

		s.into_iter().all(|e|u16::from_str(e).is_ok())
	}
}

impl Display for Version {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}.{}", self.global, self.major, self.minor, self.patch)
	}
}


impl PartialOrd<Self> for Version {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.to_u64().cmp(&other.to_u64()))
	}
}

impl Ord for Version {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_u64().cmp(&other.to_u64())
	}
}

impl FromStr for Version {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut split = s.split(".").map(|e|u16::from_str(e).map_err(|_|()));
		let mut next = || split.next().ok_or(());
		Ok(Self {
			global: next()??,
			major:  next()??,
			minor:  next()??,
			patch:  next()??,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display_impl() {
		assert_eq!("2.33.5.14", Version::new(2, 33, 5, 14).to_string().as_str())
	}

	#[test]
	fn from_str_impl() {
		assert_eq!(Version::from_str("2.33.5.14").unwrap(), Version::new(2, 33, 5, 14))
	}

	#[test]
	#[should_panic]
	fn empty_str() {
		Version::from_str("").unwrap();
	}

	#[test]
	#[should_panic]
	fn short_str() {
		Version::from_str("2.33.5").unwrap();
	}

	#[test]
	#[should_panic]
	fn ends_early() {
		Version::from_str("2.33.5..").unwrap();
	}

	#[test]
	fn equal() {
		assert_eq!(
			Version::new(2,33,5,14),
			Version::new(2,33,5,14),
		)
	}

	#[test]
	fn greater_patch() {
		assert!(
			Version::new(2,33,5,15) > Version::new(2,33,5,14)
		)
	}

	#[test]
	fn less_patch() {
		assert!(
			Version::new(2,33,5,13) < Version::new(2,33,5,14)
		)
	}

	#[test]
	fn greater_than_zero() {
		assert!(
			Version::new(0,0,0,0) < Version::new(2,33,5,14)
		)
	}

	#[test]
	// Randomly chosen by a fair dice roll
	// https://xkcd.com/221/
	fn random() {
		assert!(
			Version::new(2,32,u16::MAX,u16::MAX) < Version::new(2,33,5,14)
		)
	}
}
