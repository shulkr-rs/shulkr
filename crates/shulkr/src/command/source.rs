use std::any::Any;
use std::sync::Arc;

/// Anything that can invoke a command, such as a player or the server console.
///
/// This trait carries no behaviour of its own; it exists so senders of
/// different types can share one dispatcher. Everything useful lives on the
/// underlying type, reachable via [`downcast_ref`](Self::downcast_ref).
///
/// # Examples
/// ```
/// use shulkr::command::CommandSender;
/// use std::any::Any;
///
/// struct ConsoleSender;
///
/// impl CommandSender for ConsoleSender {
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// ```
pub trait CommandSender: Any + Send + Sync {
    /// Returns `self` as a `&dyn Any`.
    fn as_any(&self) -> &dyn Any;
}

impl dyn CommandSender {
    /// Returns a reference to `T: CommandSender` if it is of type `T`, or `None` if it isn't.
    ///
    /// # Examples
    /// ```
    /// use shulkr::command::CommandSender;
    /// use std::any::Any;
    ///
    /// struct ConsoleSender;
    ///
    /// impl ConsoleSender {
    ///     fn print_message(&self) {
    ///         println!("hello from console!");
    ///     }
    /// }
    ///
    /// impl CommandSender for ConsoleSender {
    ///     fn as_any(&self) -> &dyn Any {
    ///         self
    ///     }
    /// }
    ///
    /// fn greet(sender: &dyn CommandSender) {
    ///     match sender.downcast_ref::<ConsoleSender>() {
    ///         Some(console) => console.print_message(),
    ///         None => println!("sender is not a console"),
    ///     }
    /// }
    ///
    /// greet(&ConsoleSender);
    /// ```
    pub fn downcast_ref<T: CommandSender>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    /// Returns `true` if the source is of type `T`.
    ///
    /// # Examples
    /// ```
    /// use shulkr::command::CommandSender;
    /// use std::any::Any;
    ///
    /// struct ConsoleSender;
    ///
    /// impl CommandSender for ConsoleSender {
    ///     fn as_any(&self) -> &dyn Any {
    ///         self
    ///     }
    /// }
    ///
    /// let sender: &dyn CommandSender = &ConsoleSender;
    /// assert!(sender.is::<ConsoleSender>());
    /// ```
    pub fn is<T: CommandSender>(&self) -> bool {
        self.as_any().is::<T>()
    }
}

/// The source type used by [`CommandDispatcher`](crate::command::CommandDispatcher).
pub type CommandSource = Arc<dyn CommandSender>;
