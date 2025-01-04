# flatbuffers-owned &emsp; [![Build Status]][actions] [![License]][License File] [![Latest Version]][crates.io] [![docs-badge]][docs.rs]

[Build Status]: https://github.com/florian-g2/flatbuffers-owned/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/florian-g2/flatbuffers-owned/actions/workflows/rust.yml
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[License File]: LICENSE
[Latest Version]: https://img.shields.io/crates/v/flatbuffers-owned.svg
[crates.io]: https://crates.io/crates/flatbuffers-owned
[docs-badge]: https://img.shields.io/docsrs/flatbuffers-owned
[docs.rs]: https://docs.rs/flatbuffers-owned

A Rust crate that enables a more flexible usage of FlatBuffers.

Using the `flatbuffers_owned!` macro, you can generate wrapper structs for your flatc generated Rust FlatBuffers. \
The generated wrapper structs utilize more flexible lifetimes to access the actual underlying FlatBuffer structure. \
As the lifetimes are more relaxed, the raw FlatBuffer bytes can be owned and moved along, or be referenced with any lifetime available.

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
flatbuffers-owned = "0.3"
```

## Quickstart
Use the `flatbuffers_owned!` macro on your FlatBuffers to generate the wrapper structs.

In this example it generates a `RelaxedMessage` wrapper-struct and a `OwnedMessage` type alias for the `Message` FlatBuffer:
```rust
use flatbuffers_owned::*;

flatbuffers_owned!(Message);
```

Owned flatbuffers can be created by calling the `new()` constructor on the generated `Owned{FLATBUFFER_NAME}` type alias.
```rust
// receive byte slice reference from somewhere
let message_bytes: &[u8] = receive_message_bytes();

// copy bytes into a Box to own the bytes
let message_bytes: Box<[u8]> = Box::from(message_bytes);

// create owned message from owned message_bytes box
let owned_message = OwnedMessage::new(message_bytes).unwrap();
```

Call `.as_actual()` on the owned message to get a reference to the actual FlatBuffer struct.
```rust
let message: Message = owned_message.as_actual();

assert_eq!(message.get_text().unwrap(), "Hello, world!");
```

## Error-Handling
The `new()` constructor always verifies the raw FlatBuffer bytes using the FlatBuffer's built-in `run_verifier()` method.</br>
Since there can always be a faulty byte-slice passed, you need to check the returned Result of the constructor:
```rust
for id in message_ids {
    let message_bytes = Box::from(receive_message_bytes());

    let owned_message = OwnedMessage::new(message_bytes);

    match owned_message {
        Ok(message) => {
            // ... process message
        },
        Err(e) => {
            println!("Failed to parse Message: {}", e);
            // ... handling logic
        }
    }
}
```

## Approach
### The wrapper struct
The `Relaxed{FLATBUFFER_NAME}` wrapper struct is a Newtype which can wrap any struct that can convert to a byte slice reference. (```where TBuffer: AsRef<[u8]>```) \
This struct can be used with buffers that fully own its memory, or only hold a shared-reference.

The `Owned{FLATBUFFER_NAME}` type alias generated along the wrapper struct just predefines the `TBuffer` generic. \
For our `Message` example FlatBuffer, the generated type-alias code would be the following:
```rust 
pub type OwnedMessage = RelaxedMessage<Box<[u8]>>;
```

### Deref to &[u8]
The `RelaxedFlatBufferTrait` enforces a de-reference to the underlying [u8] byte slice. \
A de-reference to the actual FlatBuffer struct is sadly not possible, since the associated type of the `Deref` trait can not carry a lifetime.

## Open to Feedback
If you have any ideas for improvements or would like to contribute to this project, please feel free to open an issue or pull request.

I will also be happy for any general tips or suggestions given that this is my first (published) library ever. :)

## License

This project is released under the [MIT License](LICENSE), which allows for commercial use, modification, distribution, and private use. \
See the [LICENSE](LICENSE) file for the full text of the license.