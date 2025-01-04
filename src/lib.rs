//! A Rust crate that enables a more flexible usage of FlatBuffers.
//!
//! Using the `flatbuffers_owned!` macro, you can generate wrapper structs for your flatc generated Rust FlatBuffers. \
//! The generated wrapper structs utilize more flexible lifetimes to access the actual underlying FlatBuffer structure. \
//! As the lifetimes are more relaxed, the raw FlatBuffer bytes can be owned and moved along, or be referenced with any lifetime available.
//!
//! ## Usage
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! flatbuffers-owned = "0.2"
//! ```
//!
//! ## Quickstart
//! Use the `flatbuffers_owned!` macro on your FlatBuffers to generate the wrapper structs.
//!
//! This generates a `RelaxedMessage` wrapper-struct and a `OwnedMessage` type alias for the `Message` FlatBuffer:
//! ```rust
//! use flatbuffers_owned::*;
//!
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
//! The `new()` constructor always verifies the raw FlatBuffer bytes using the FlatBuffer's built-in `run_verifier()` method.</br>
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
//!         }
//!     }
//! }
//! ```
//!
//! ## Approach
//! ### The wrapper struct
//! The `Relaxed{FLATBUFFER_NAME}` wrapper struct is a Newtype which can wrap any struct that can convert to a byte slice reference. (```where TBuffer: AsRef<[u8]>```) \
//! This struct can be used with buffers that fully own its memory, or only hold a shared-reference.
//!
//! The `Owned{FLATBUFFER_NAME}` type alias generated along the wrapper struct just predefines the `TBuffer` generic. \
//! For our `Message` example FlatBuffer, the generated type alias code would be the following:
//! ```rust
//! pub type OwnedMessage = RelaxedMessage<Box<[u8]>>;
//! ```
//!
//! ### Deref to &[u8]
//! The `RelaxedFlatBufferTrait` enforces a de-reference to the underlying [u8] byte slice. \
//! A de-reference to the actual FlatBuffer struct is sadly not possible, since the associated type of the `Deref` trait can not carry a lifetime.
//!
//! ## Open to Feedback
//! If you have any ideas for improvements or would like to contribute to this project, please feel free to open an issue or pull request.
//!
//! I will also be happy for any general tips or suggestions given that this is my first (published) library ever. :)

use std::ops::Deref;
use flatbuffers::{Follow, ForwardsUOffset, InvalidFlatbuffer, Verifiable, Verifier, VerifierOptions};

#[doc(hidden)]
pub use paste::paste;

/// This trait allows a `.follow()` method that returns a FlatBuffer with the lifetime of the provided byte slice.
///
/// # Example trait implementation
/// ```
/// use flatbuffers_owned::RelaxedFollowTrait;
///
/// impl RelaxedFollowTrait for MyFlatBuffer<'_> {
///    type Inner<'a> = MyFlatBuffer<'a>;
/// }
pub trait RelaxedFollowTrait {
    type Inner<'a>: Follow<'a>;

    /// # Safety
    ///
    /// This function lacks verification. This method could yield a FlatBuffer with undefined behavior on field reads when corrupted FlatBuffer bytes are passed.
    #[inline(always)]
    unsafe fn follow(buf: &[u8], loc: usize) -> <<Self as RelaxedFollowTrait>::Inner<'_> as Follow<'_>>::Inner {
        <ForwardsUOffset<Self::Inner<'_>>>::follow(buf, loc)
    }
}

/// This trait serves as the foundation for this crate. \
/// It allows access to the underlying FlatBuffer using the [as_actual()](RelaxedFlatBufferTrait::as_actual) method. \
/// The lifetimes used here are more relaxed than those in the flatc generated code. \
/// For example, it could be implemented for a fully owned `Box<\[u8\]>` new-type or even a reference to memory that has been temporarily pinned following a DB query.
///
/// This trait requires the [RelaxedFollowTrait] trait bound on the FlatBuffer type. \
/// It can be implemented either manually or using the [flatbuffers_owned!](flatbuffers_owned) macro.
///
/// # Safety
/// The default implementation of the [as_actual()](RelaxedFlatBufferTrait::as_actual) method uses the unsafe [.follow()](RelaxedFollowTrait::follow) method do return the actual FlatBuffer. \
/// [as_actual()](RelaxedFlatBufferTrait::as_actual) does not verify the FlatBuffer. \
/// If you choose to implement this trait manually, you must ensure that the underlying byte slice is verified and the buffer remains immutable.
///
/// # Example trait usage
/// ```
/// # use flatbuffers_owned::RelaxedFlatBufferTrait;
///
/// fn store_fbs<TBuffer>(flatbuffers: &[impl RelaxedFlatBufferTrait<TBuffer>]) {
///    for item in flatbuffers {
///         let bytes: &[u8] = item.deref();
///         // ... store the raw bytes somewhere.
///    }
/// }
/// ```
pub unsafe trait RelaxedFlatBufferTrait<TBuffer>
    where Self: Deref<Target = [u8]> + Sized
{
    type FlatBuffer: RelaxedFollowTrait + Verifiable;

    /// Initializes a actual FlatBuffer struct from the byte slice returned by the Self::deref() method.
    #[inline(always)]
    fn as_actual(&self) -> <<<Self as RelaxedFlatBufferTrait<TBuffer>>::FlatBuffer as RelaxedFollowTrait>::Inner<'_> as Follow<'_>>::Inner {
        unsafe { Self::FlatBuffer::follow(self, 0) }
    }

    /// Verifies the FlatBuffer data.
    fn verify(data: &[u8]) -> Result<(), InvalidFlatbuffer> {
        let opts = VerifierOptions::default();
        let mut v = Verifier::new(&opts, data);

        <ForwardsUOffset<Self::FlatBuffer>>::run_verifier(&mut v, 0)
    }

    fn new(data: TBuffer) -> Result<Self, InvalidFlatbuffer>;
}

/// Use this macro on your FlatBuffers to generate the required code to start using this crate.
///
/// After invoking the macro, you have two generated types for each of your passed FlatBuffers: \
/// 1. A generic new-type struct named `Relaxed{FLATBUFFER_NAME}`, which implements [RelaxedFlatBufferTrait] and takes the generic `TBuffer: AsRef<[u8]>`. \
/// 2. A type alias named `Owned{FLATBUFFER_NAME}, which aliases the `Relaxed{FLATBUFFER_NAME}` struct and sets `TBuffer` to `Box<[u8]>`.
///
/// # Usage
/// ```
/// use flatbuffers_owned::flatbuffers_owned;
///
/// flatbuffers_owned!(MyFirstFlatBuffer, MySecondFlatBuffer);
/// ```
#[macro_export]
macro_rules! flatbuffers_owned {
    ($struct_name:ident) => {
        $crate::paste! {
            impl $crate::RelaxedFollowTrait for $struct_name<'_> {
                type Inner<'a> = $struct_name<'a>;
            }

            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            pub struct [<Relaxed $struct_name>]<TBuffer: AsRef<[u8]>>(TBuffer);

            unsafe impl <TBuffer: AsRef<[u8]>> RelaxedFlatBufferTrait<TBuffer> for [<Relaxed $struct_name>]<TBuffer> {
                type FlatBuffer = $struct_name<'static>;

                fn new(data: TBuffer) -> Result<Self, flatbuffers::InvalidFlatbuffer> {
                    Self::verify(data.as_ref())?;

                    Ok(Self(data))
                }
            }
            
            impl <TBuffer: AsRef<[u8]>> std::ops::Deref for [<Relaxed $struct_name>]<TBuffer> {
                type Target = [u8];

                fn deref(&self) -> &Self::Target {
                    self.0.as_ref()
                }
            }

            pub type [<Owned $struct_name>] = [<Relaxed $struct_name>]<Box<[u8]>>;
        }
    };

    ($($struct_name:ident),*) => {
        $(
            $crate::flatbuffers_owned!($struct_name);
        )*
    };
}