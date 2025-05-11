//! Unwrap trait for MaybeFut types.

/// Unwrap trait for MaybeFut types.
///
/// This trait provides methods to get the underlying implementations for the MaybeFut wrappers.
///
/// Every type implemented by the **maybe_fut** library has a corresponding `Unwrap` implementation.
pub trait Unwrap {
    type StdImpl;
    type AsyncImpl;

    /// Unwraps the std underlying implementation of the MaybeFut type.
    fn unwrap_std(self) -> Self::StdImpl;

    /// Unwraps the async underlying implementation of the MaybeFut type.
    fn unwrap_async(self) -> Self::AsyncImpl;

    /// Unwraps the std underlying implementation of the MaybeFut type as a reference.
    fn unwrap_std_ref(&self) -> &Self::StdImpl;

    /// Unwraps the async underlying implementation of the MaybeFut type as a reference.
    fn unwrap_async_ref(&self) -> &Self::AsyncImpl;

    /// Unwraps the std underlying implementation of the MaybeFut type as a mutable reference.
    fn unwrap_std_mut(&mut self) -> &mut Self::StdImpl;

    /// Unwraps the async underlying implementation of the MaybeFut type as a mutable reference.
    fn unwrap_async_mut(&mut self) -> &mut Self::AsyncImpl;

    /// Safely unwraps the std underlying implementation of the MaybeFut type.
    fn get_std(self) -> Option<Self::StdImpl>;

    /// Safely unwraps the async underlying implementation of the MaybeFut type.
    fn get_async(self) -> Option<Self::AsyncImpl>;

    /// Safely unwraps the std underlying implementation of the MaybeFut type as a reference.
    fn get_std_ref(&self) -> Option<&Self::StdImpl>;

    /// Safely unwraps the async underlying implementation of the MaybeFut type as a reference.
    fn get_async_ref(&self) -> Option<&Self::AsyncImpl>;

    /// Safely unwraps the std underlying implementation of the MaybeFut type as a mutable reference.
    fn get_std_mut(&mut self) -> Option<&mut Self::StdImpl>;

    /// Safely unwraps the async underlying implementation of the MaybeFut type as a mutable reference.
    fn get_async_mut(&mut self) -> Option<&mut Self::AsyncImpl>;
}
