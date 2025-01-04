#[allow(dead_code, unused_imports)]
pub mod generated_fbs {
    pub mod foo;
}

use flatbuffers::{FlatBufferBuilder};
use generated_fbs::foo::{Foo, FooArgs};
use flatbuffers_owned::{flatbuffers_owned, RelaxedFlatBufferTrait};

// Create OwnedFoo type alias
flatbuffers_owned!(Foo);

fn get_foo_bytes() -> Box<[u8]> {
    let mut builder = FlatBufferBuilder::new();
    let b = builder.create_string("Hello, world!");

    let offset = Foo::create(&mut builder, &FooArgs {
        a: 42,
        b: Some(b),
    });

    builder.finish(offset, None);

    builder.finished_data().into()
}

#[test]
fn init_foo() {
    let foo_bytes = get_foo_bytes();
    let foo = flatbuffers::root::<Foo>(&foo_bytes).expect("Failed to parse Foo");

    assert_eq!(foo.a(), 42);
    assert_eq!(foo.b().unwrap(), "Hello, world!");
}

#[test]
fn create_owned_foo() {
    let owned_foo;
    {
        let foo_bytes = get_foo_bytes();

        owned_foo = OwnedFoo::new(foo_bytes).expect("Failed to parse Foo");
    }

    let foo = owned_foo.as_actual();

    assert_eq!(foo.a(), 42);
    assert_eq!(foo.b().unwrap(), "Hello, world!");
}

#[test]
fn fail_invalid_foo_bytes() {
    let mut foo_bytes = get_foo_bytes();
    foo_bytes[0] = 1; // corrupt the flatbuffer

    assert!(OwnedFoo::new(foo_bytes).is_err());
}

// This is more a compile- than a runtime-time test.
#[test]
fn working_generic_function() {
    fn test<TBuffer, TFlatBuffer>(flatbuffer: TFlatBuffer)
        where TFlatBuffer: RelaxedFlatBufferTrait<TBuffer>
    {
        assert!(TFlatBuffer::verify(flatbuffer.deref()).is_ok());
    }

    let foo = OwnedFoo::new(get_foo_bytes()).unwrap();
    test(foo);
}