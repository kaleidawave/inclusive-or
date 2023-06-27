/// A 'inclusive or' type
///
/// Represents a either T, U or both but never neither
///
/// Constructable using [InclusiveOrExt]
/// ```rust
/// use inclusive_or::{InclusiveOr, InclusiveOrExt};
///
/// assert!(matches!(Some(3).or_inclusive(Some(2)), Some(InclusiveOr::LeftAndRight(3, 2))));
/// assert!(matches!(Some(6).or_inclusive(None::<()>), Some(InclusiveOr::Left(6))));
/// assert!(matches!(None::<()>.or_inclusive(Some(2)), Some(InclusiveOr::Right(2))));
/// assert!(matches!(None::<()>.or_inclusive(None::<()>), None));
/// ```
#[derive(Debug)]
pub enum InclusiveOr<T, U> {
    LeftAndRight(T, U),
    Left(T),
    Right(U),
}

pub type InclusiveOrHomogeneous<T> = InclusiveOr<T, T>;

impl<T, U> InclusiveOr<T, U> {
    pub fn get_left(&self) -> Option<&T> {
        match self {
            InclusiveOr::LeftAndRight(left, _) | InclusiveOr::Left(left) => Some(left),
            InclusiveOr::Right(_) => None,
        }
    }

    pub fn get_right(&self) -> Option<&U> {
        match self {
            InclusiveOr::LeftAndRight(_, right) | InclusiveOr::Right(right) => Some(right),
            InclusiveOr::Left(_) => None,
        }
    }
}

/// Extension trait for generating [InclusiveOr] from two [Option]s
pub trait InclusiveOrExt<T, U> {
    fn or_inclusive(self, other: Option<U>) -> Option<InclusiveOr<T, U>>;
}

impl<T, U> InclusiveOrExt<T, U> for Option<T> {
    fn or_inclusive(self, other: Option<U>) -> Option<InclusiveOr<T, U>> {
        match (self, other) {
            (None, None) => None,
            (None, Some(right)) => Some(InclusiveOr::Right(right)),
            (Some(left), None) => Some(InclusiveOr::Left(left)),
            (Some(left), Some(right)) => Some(InclusiveOr::LeftAndRight(left, right)),
        }
    }
}
