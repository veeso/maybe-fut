//! Utility macros to automatically implement the async/sync fn

/// A macro to create a constructor function that can be used in both async and sync contexts.
///
/// It takes the documentation attributes, the function name, the arguments, the return type,
/// the module is the internal path of a module (e.g. fs, net, etc),
/// and finally the type name you want to create.
///
/// ## Examples
///
/// ```rust,ignore
/// impl File {
///     maybe_fut_constructor_result!(
///         /// Attempts to open a file in read-only mode.
///         /// See [`OpenOptions`] for more details.
///         ///
///         /// ## Errors
///         ///
///         /// This function will return an error if called from outside of the Tokio runtime (if async) or if path does not already exist.
///         /// Other errors may also be returned according to OpenOptions::open.
///         ///
///         /// See <https://docs.rs/rustc-std-workspace-std/latest/std/fs/struct.File.html#method.open>
///         open(path: impl AsRef<Path>) -> std::io::Result<Self>,
///         std::fs::File::open,
///         tokio::fs::File::open,
///         tokio_fs
///     );
/// }
/// ```
#[macro_export]
macro_rules! maybe_fut_constructor_result {
    ($(#[$meta:meta])*
        $name:ident
        (
            $ ( $arg_name:ident : $arg_type:ty ),*
            $(,)?
        )
        -> $ret:ty,
        $std_module:path,
        $tokio_module:path,
        $feature:ident
    ) => {
            $(#[$meta])*
            pub async fn $name( $( $arg_name : $arg_type ),* ) -> $ret {
                #[cfg($feature)]
                {
                    if $crate::context::is_async_context() {
                        $tokio_module( $( $arg_name ),* ).await.map(Self::from)
                    } else {
                        $std_module( $( $arg_name ),* ).map(Self::from)
                    }
                }
                #[cfg(not($feature))]
                {
                    $std_module( $( $arg_name ),* ).map(Self::from)
                }
            }
        };
}

/// A macro to create a method that can be used in both async and sync contexts.
#[macro_export]
macro_rules! maybe_fut_method {
    ($(#[$meta:meta])*
        $name:ident
        (
            $( $arg_name:ident : $arg_type:ty ),* $(,)?
        )
        -> $ret:ty,
        $sync_inner_type:path,
        $async_inner_type:path,
        $feature:ident
    ) => {
            $(#[$meta])*
            pub async fn $name( &self, $( $arg_name : $arg_type ),* ) -> $ret {
                match &self.0 {
                    $sync_inner_type(inner) => inner.$name( $( $arg_name ),* ),
                    #[cfg($feature)]
                    $async_inner_type(inner) => inner.$name( $( $arg_name ),* ).await,
                }
            }
        };
}

/// A macro to create a mutable method that can be used in both async and sync contexts.
#[macro_export]
macro_rules! maybe_fut_method_mut {
    (
        $(#[$meta:meta])*
        $name:ident
        (
            $( $arg_name:ident : $arg_type:ty ),* $(,)?
        )
        -> $ret:ty,
        $sync_inner_type:path,
        $async_inner_type:path,
        $feature:ident
    ) => {
            $(#[$meta])*
            pub async fn $name( &mut self, $( $arg_name : $arg_type ),* ) -> $ret {
                match &mut self.0 {
                    $sync_inner_type(inner) => inner.$name( $( $arg_name ),* ),
                    #[cfg($feature)]
                    $async_inner_type(inner) => inner.$name( $( $arg_name ),* ).await,
                }
            }
        };
}

#[macro_export]
/// A macro to create a function that can be used in both async and sync contexts.
macro_rules! maybe_fut_function {
    (
        $(#[$meta:meta])*
        $name:ident
        (
            $( $arg_name:ident : $arg_type:ty ),* $(,)?
        )
        -> $ret:ty,
        $sync_function:path,
        $async_function:path,
        $feature:ident
    ) => {
        $(#[$meta])*
        pub async fn $name( $( $arg_name : $arg_type ),* ) -> $ret {
            #[cfg($feature)]
            {
                if $crate::context::is_async_context() {
                    $async_function( $( $arg_name ),* ).await
                } else {
                    $sync_function( $( $arg_name ),* )
                }
            }
            #[cfg(not($feature))]
            {
                $sync_function( $( $arg_name ),* )
            }
        }
    };
}
