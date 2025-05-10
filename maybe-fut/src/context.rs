/// Returns whether the current code is being executed in an async context.
///
/// If tokio is disabled, this function will always return false.
#[inline]
pub fn is_async_context() -> bool {
    #[cfg(tokio)]
    {
        tokio::runtime::Handle::try_current().is_ok()
    }
    #[cfg(not(tokio))]
    {
        false
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_should_return_false_if_not_in_async_context() {
        assert!(!is_async_context(),);
    }

    #[tokio::test]
    async fn test_should_return_true_if_in_async_context() {
        assert!(is_async_context());
    }
}
