# flatbuffers-owned &emsp; [![Build Status]][actions] [![License]][License File] [![Latest Version]][crates.io]

[Build Status]: https://github.com/florian-g2/flatbuffers-owned/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/florian-g2/flatbuffers-owned/actions/workflows/rust.yml
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[License File]: LICENSE
[Latest Version]: https://img.shields.io/crates/v/flatbuffers-owned.svg
[crates.io]: https://crates.io/crates/flatbuffers-owned

This small Rust crate provides a wrapper struct for generated Rust FlatBuffers that allows them to be used as owned types.</br></br>
A owned FlatBuffer does not reference its source data and can therefore be easily moved into another thread.

[Documentation](https://docs.rs/flatbuffers-owned)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
flatbuffers-owned = "0.1"
```

## Quickstart
Use the `flatbuffers_owned!` convenience macro on your FlatBuffers to implement the required trait and introduce a type alias for each owned FlatBuffer.

Generate the `OwnedMessage` type alias for the `Message` FlatBuffer:
```rust
use flatbuffers_owned::*;

flatbuffers_owned!(Message);
```

Receive a byte slice, create a boxed slice, and initialize the owned flatbuffer:
```rust
let message_bytes: &[u8] = receive_message_bytes();
let message_bytes: Box<[u8]> = Box::from(message_bytes);

let owned_message = OwnedMessage::new(message_bytes).unwrap();
```

Access the actual FlatBuffer:
```rust
let message: Message = owned_message.as_actual();

assert_eq!(message.get_text().unwrap(), "Hello, world!");
```

## Error-Handling
The new() constructor always verifies the raw FlatBuffer bytes using the FlatBuffer's built-in run_verifier() method.</br>
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
        },
    }
} 
```

## Approach
### The wrapper struct
The wrapper struct is a newtype for a Box<[u8]> that accepts a FlatBuffer as the generic type.</br>
With the `flatbuffers_owned!` convenience macro we get a type alias that just masks this wrapper struct.

```rust
pub type OwnedMessage = OwnedFlatBuffer<Message<'static>>;
```

So instead of `OwnedMessage`, we can just as well use `OwnedFlatBuffer<Message<'static>>`.

```rust
let owned_message = OwnedFlatBuffer::<Message<'static>>::new(message_bytes).unwrap();
```

As you may have noticed, the `'static` lifetime is then always present when working with the OwnedFlatBuffer.</br>
However, this can be misleading, because the OwnedFlatBuffer does not actually reference anything in the `'static` lifetime.</br>
The lifetime is only required by the FlatBuffer struct.</br>
So to make the code more readable, we have the type alias.</br>

### Deref to &[u8]
The OwnedFlatBuffer struct de-references itself to its underlying bytes slice.</br>
A Deref to the actual FlatBuffer struct is sadly not possible, since the associated type of the Deref trait can not carry a lifetime.

## Open to Feedback
If you have any ideas for improvements or would like to contribute to this project, please feel free to open an issue or pull request.</br>
</br>
I will also be happy for any general tips or suggestions given that this is my first (published) library ever. :)

## License

This project is released under the [MIT License](LICENSE), which allows for commercial use, modification, distribution, and private use.
See the [LICENSE](LICENSE) file for the full text of the license.