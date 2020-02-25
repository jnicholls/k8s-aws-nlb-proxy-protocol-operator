//! The `join` macro.

use proc_macro_hack::proc_macro_hack;

#[doc(hidden)]
#[macro_export]
macro_rules! document_join_macro {
    ($join:item $try_join:item) => {
        /// Polls multiple futures simultaneously, returning a tuple
        /// of all results once complete.
        ///
        /// While `join!(a, b)` is similar to `(a.await, b.await)`,
        /// `join!` polls both futures concurrently and therefore is more efficent.
        ///
        /// This macro is only usable inside of async functions, closures, and blocks.
        /// It is also gated behind the `async-await` feature of this library, which is
        /// activated by default.
        ///
        /// # Examples
        ///
        /// ```
        /// # futures::executor::block_on(async {
        /// use futures::join;
        ///
        /// let a = async { 1 };
        /// let b = async { 2 };
        ///
        /// assert_eq!(join!(a, b), (1, 2));
        /// # });
        /// ```
        $join

        /// Polls multiple futures simultaneously, resolving to a [`Result`] containing
        /// either a tuple of the successful outputs or an error.
        ///
        /// `try_join!` is similar to [`join!`], but completes immediately if any of
        /// the futures return an error.
        ///
        /// This macro is only usable inside of async functions, closures, and blocks.
        /// It is also gated behind the `async-await` feature of this library, which is
        /// activated by default.
        ///
        /// # Examples
        ///
        /// When used on multiple futures that return `Ok`, `try_join!` will return
        /// `Ok` of a tuple of the values:
        ///
        /// ```
        /// # futures::executor::block_on(async {
        /// use futures::try_join;
        ///
        /// let a = async { Ok::<i32, i32>(1) };
        /// let b = async { Ok::<u64, i32>(2) };
        ///
        /// assert_eq!(try_join!(a, b), Ok((1, 2)));
        /// # });
        /// ```
        ///
        /// If one of the futures resolves to an error, `try_join!` will return
        /// that error:
        ///
        /// ```
        /// # futures::executor::block_on(async {
        /// use futures::try_join;
        ///
        /// let a = async { Ok::<i32, i32>(1) };
        /// let b = async { Err::<u64, i32>(2) };
        ///
        /// assert_eq!(try_join!(a, b), Err(2));
        /// # });
        /// ```
        $try_join
    }
}

document_join_macro! {
    #[proc_macro_hack(support_nested)]
    pub use futures_macro::join;

    #[proc_macro_hack(support_nested)]
    pub use futures_macro::try_join;
}