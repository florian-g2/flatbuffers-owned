use std::ops::Deref;
use flatbuffers::{Follow, ForwardsUOffset, InvalidFlatbuffer, Verifiable, Verifier, VerifierOptions};

pub use paste::paste;

/// This trait allows a `.follow()` method that returns a flatbuffer with the lifetime of the provided u8 slice.
pub trait RelaxedFollow {
    type Inner<'a>: Follow<'a>;

    fn follow(buf: &[u8], loc: usize) -> <<Self as RelaxedFollow>::Inner<'_> as Follow<'_>>::Inner {
        unsafe { <ForwardsUOffset<Self::Inner<'_>>>::follow(buf, loc) }
    }
}

/// The trait for owned flatbuffers.
/// Use the `.as_inner()` method to get a actual flatbuffer struct that references the owned data.
/// Use `&*` or `.deref()` to get the underlying bytes.
///
/// With the `owned_flatbuffer!` macro you can create a newtype that implements this trait for your flatbuffer.
///
/// # Example trait usage
/// ```
/// use flatbuffers_owned::OwnedFlatBuffer;
///
/// fn process(flatbuffers: &[impl OwnedFlatBuffer]) {
///    for item in flatbuffers {
///         let bytes: &[u8] = &*item;
///         // ... do something with the raw bytes
///    }
/// }
/// ```
pub trait OwnedFlatBuffer: Deref<Target = [u8]> + From<Box<[u8]>> + Sized {
    type FlatBuffer: RelaxedFollow + Verifiable;

    /// Initializes a actual flatbuffer struct that references the owned data.
    fn as_inner(&self) -> <<<Self as OwnedFlatBuffer>::FlatBuffer as RelaxedFollow>::Inner<'_> as Follow<'_>>::Inner {
        Self::FlatBuffer::follow(self, 0)
    }

    /// Create a new owned flatbuffer from the provided data.
    /// This method calls the verifier of the flatbuffer and returns an error result if the data is invalid.
    fn new(data: Box<[u8]>) -> Result<Self, InvalidFlatbuffer>
    {
        let opts = VerifierOptions::default();
        let mut v = Verifier::new(&opts, &data);

        <ForwardsUOffset<Self::FlatBuffer>>::run_verifier(&mut v, 0)?;

        Ok(Self::from(data))
    }
}

/// This macro creates a newtype that implements the `OwnedFlatBuffer` trait.
/// The generated newtype is named `Owned<struct_name>`.
/// Call this macro with the flatbuffer struct name as the argument.
///
/// Example:
/// ```
/// # use flatbuffers_owned::flatbuffers_owned;
///
/// flatbuffers_owned!(MyFirstFlatBuffer, MySecondFlatBuffer);
/// ```
#[macro_export]
macro_rules! flatbuffers_owned {
    ($struct_name:ident) => {
        $crate::paste! {
            impl $crate::RelaxedFollow for $struct_name<'_> {
                type Inner<'a> = $struct_name<'a>;
            }

            pub struct [<Owned $struct_name>] (Box<[u8]>);

            impl std::ops::Deref for [<Owned $struct_name>] {
                type Target = [u8];

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl From<Box<[u8]>> for [<Owned $struct_name>] {
                fn from(value: Box<[u8]>) -> Self {
                    Self(value)
                }
            }

            impl $crate::OwnedFlatBuffer for [<Owned $struct_name>] {
                type FlatBuffer = $struct_name<'static>;
            }
        }
    };

    ($($struct_name:ident),*) => {
        $(
            $crate::flatbuffers_owned!($struct_name);
        )*
    };
}