//! This small Rust crate provides a wrapper struct for generated Rust FlatBuffers that allows them to be used as owned types.</br>
//! A owned FlatBuffer does not reference its source data and can therefore be easily moved into another thread.
//!
//! ## Quickstart
//! Use the `flatbuffers_owned!` convenience macro on your FlatBuffers to implement the required trait and introduce a type alias for each owned FlatBuffer.
//!
//! Generate the `OwnedMessage` type alias for the `Message` FlatBuffer:
//! ```rust
//! flatbuffers_owned!(Message);
//! ```
//!
//! Receive a byte slice, create a boxed slice, and initialize the owned flatbuffer:
//! ```rust
//! let message_bytes: &[u8] = receive_message_bytes();
//! let message_bytes: Box<[u8]> = Box::from(message_bytes);
//!
//! let owned_message = OwnedMessage::new(message_bytes).unwrap();
//! ```
//!
//! Access the actual FlatBuffer:
//! ```rust
//! let message: Message = owned_message.as_actual();
//!
//! assert_eq!(message.get_text().unwrap(), "Hello, world!");
//! ```
//!
//! ## Error-Handling
//! The new() constructor always verifies the raw FlatBuffer bytes using the FlatBuffer's built-in run_verifier() method.</br>
//! Since there can always be a faulty byte-slice passed, you need to check the returned Result of the constructor:
//! ```rust
//! for id in message_ids {
//!     let message_bytes = Box::from(receive_message_bytes());
//!
//!     let owned_message = OwnedMessage::new(message_bytes);
//!
//!     match owned_message {
//!         Ok(message) => {
//!             // ... process message
//!         },
//!         Err(e) => {
//!             println!("Failed to parse Message: {}", e);
//!             // ... handling logic
//!         },
//!     }
//! }
//! ```
//!
//! ## Approach
//! ### The wrapper struct
//! The wrapper struct is a newtype for a Box<[u8]> that accepts a FlatBuffer as the generic type.</br>
//! With the `flatbuffers_owned!` convenience macro we get a type alias that just masks this wrapper struct.
//!
//! ```rust
//! pub type OwnedMessage = OwnedFlatBuffer<Message<'static>>;
//! ```
//!
//! So instead of `OwnedMessage`, we can just as well use `OwnedFlatBuffer<Message<'static>>`.
//!
//! ```rust
//! let owned_message = OwnedFlatBuffer::<Message<'static>>::new(message_bytes).unwrap();
//! ```
//!
//! As you may have noticed, the `'static` lifetime is then always present when working with the OwnedFlatBuffer.</br>
//! However, this can be misleading, because the OwnedFlatBuffer does not actually reference anything in the `'static` lifetime.</br>
//! The lifetime is only required by the FlatBuffer struct.</br>
//! So to make the code more readable, we have the type alias.</br>
//!
//! ### Deref to &[u8]
//! The OwnedFlatBuffer struct de-references itself to its underlying bytes slice.</br>
//! A Deref to the actual FlatBuffer struct is sadly not possible, since the associated type of the Deref trait can not carry a lifetime.
//!
//! ## Open to Feedback
//! If you have any ideas for improvements or would like to contribute to this project, please feel free to open an issue or pull request on GitHub.</br>
//! I will also be happy for any general tips or suggestions given that this is my first (published) library ever. :)

use std::ops::Deref;
use flatbuffers::{Follow, ForwardsUOffset, InvalidFlatbuffer, Verifiable, Verifier, VerifierOptions};

#[doc(hidden)]
pub use paste::paste;

/// This trait allows a `.follow()` method that returns a FlatBuffer with the lifetime of the provided byte slice.
///
/// # Example trait implementation
/// ```
/// use flatbuffers_owned::RelaxedFollow;
///
/// impl RelaxedFollow for MyStruct<'_> {
///    type Inner<'a> = MyFlatBuffer<'a>;
/// }
pub trait RelaxedFollow {
    type Inner<'a>: Follow<'a>;

    fn follow(buf: &[u8], loc: usize) -> <<Self as RelaxedFollow>::Inner<'_> as Follow<'_>>::Inner {
        unsafe { <ForwardsUOffset<Self::Inner<'_>>>::follow(buf, loc) }
    }
}

/// The trait for owned FlatBuffers.
///
/// This trait requires the [`RelaxedFollow`] trait bound on the FlatBuffer type.
/// It can be either implemented manually or by using the `flatbuffer_owned!` macro.
///
/// # Example trait usage
/// ```
/// # use flatbuffers_owned::OwnedFlatBuffer;
///
/// fn process_fbs(flatbuffers: &[impl OwnedFlatBufferTrait]) {
///    for item in flatbuffers {
///         let bytes: &[u8] = &*item;
///         // ... do something with the raw bytes
///    }
/// }
/// ```
pub trait OwnedFlatBufferTrait: Deref<Target = [u8]> + Sized {
    type FlatBuffer: RelaxedFollow + Verifiable;

    /// Initializes a actual FlatBuffer struct that references the owned data.
    fn as_actual(&self) -> <<<Self as OwnedFlatBufferTrait>::FlatBuffer as RelaxedFollow>::Inner<'_> as Follow<'_>>::Inner;

    /// Create a new owned FlatBuffer from the provided data.
    /// This method calls the verifier of the FlatBuffer and returns an error result if the data is invalid.
    fn new(data: Box<[u8]>) -> Result<Self, InvalidFlatbuffer>;
}

/// This is the FlatBuffer wrapper struct.
/// It is a newtype for a Box<[u8]> that accepts a FlatBuffer as the generic type.
/// The lifetime parameter of the FlatBuffer type is nowhere used and can be safely set to `'static`.
///
/// To access a actual FlatBuffer struct, use the `.as_actual()` method.
/// The returned FlatBuffer has the lifetime of the `OwnedFlatBuffer` struct.
///
/// # Example usage
/// ```
/// # use flatbuffers_owned::OwnedFlatBuffer;
///
/// let owned_message;
/// {
///     let message_bytes: &[u8] = receive_message_bytes();
///     let message_bytes: Box<[u8]> = Box::from(message_bytes);
///
///     owned_message = OwnedFlatBuffer::<Message<'static>>::new(message_bytes).expect("Failed to parse message");
/// }
///
/// let message = owned_message.as_actual();
///
/// assert_eq!(message.get_text().unwrap(), "Hello, world!");
/// ```
pub struct OwnedFlatBuffer<T>(Box<[u8]>, std::marker::PhantomData<T>)
    where T: RelaxedFollow + Verifiable;

/// I would really like to implement a deref to the actual FlatBuffer struct here, but the associated type does not allow lifetime parameters.
impl<T> Deref for OwnedFlatBuffer<T>
    where T: RelaxedFollow + Verifiable
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T> OwnedFlatBufferTrait for OwnedFlatBuffer<T>
    where T: RelaxedFollow + Verifiable
{
    type FlatBuffer = T;

    fn as_actual(&self) -> <<<Self as OwnedFlatBufferTrait>::FlatBuffer as RelaxedFollow>::Inner<'_> as Follow<'_>>::Inner {
        Self::FlatBuffer::follow(self, 0)
    }

    fn new(data: Box<[u8]>) -> Result<Self, InvalidFlatbuffer> {
        let opts = VerifierOptions::default();
        let mut v = Verifier::new(&opts, &data);

        <ForwardsUOffset<Self::FlatBuffer>>::run_verifier(&mut v, 0)?;

        Ok(Self(data, std::marker::PhantomData))
    }
}

/// This macro implements the [`RelaxedFollow`] trait for your FlatBuffer and creates a type alias for the corresponding [`OwnedFlatBuffer`] type.
/// This is the go-to macro for creating an owned FlatBuffer type.
///
///
/// # Example usage
/// ```
/// # use flatbuffers_owned::flatbuffers_owned;
///
/// flatbuffers_owned!(MyFirstFlatBuffer, MySecondFlatBuffer);
/// ```
///
/// The above macro call expands to:
/// ```
/// # use flatbuffers_owned::{RelaxedFollow, OwnedFlatBuffer};
///
/// impl RelaxedFollow for MyFirstFlatBuffer<'_> {
///   type Inner<'a> = MyFirstFlatBuffer<'a>;
/// }
///
/// pub type OwnedMyFirstFlatBuffer = OwnedFlatBuffer<MyFirstFlatBuffer<'static>>;
///
/// // ... and the same for MySecondFlatBuffer
///
#[macro_export]
macro_rules! flatbuffers_owned {
    ($struct_name:ident) => {
        $crate::paste! {
            impl $crate::RelaxedFollow for $struct_name<'_> {
                type Inner<'a> = $struct_name<'a>;
            }

            pub type [<Owned $struct_name>] = $crate::OwnedFlatBuffer<$struct_name<'static>>;
        }
    };

    ($($struct_name:ident),*) => {
        $(
            $crate::flatbuffers_owned!($struct_name);
        )*
    };
}