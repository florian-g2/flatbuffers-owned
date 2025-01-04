// This file is manually copied here, so we do not need flatc to run our tests.
// The generated Rust code is based on the following flatbuffer schema:
// table Foo {
//     a: uint32;
//     b: string;
// }
//
// Used flatc version: 24.12.23

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum FooOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Foo<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Foo<'a> {
    type Inner = Foo<'a>;
    #[inline]
    unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table::new(buf, loc) }
    }
}

impl<'a> Foo<'a> {
    pub const VT_A: flatbuffers::VOffsetT = 4;
    pub const VT_B: flatbuffers::VOffsetT = 6;

    #[inline]
    pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Foo { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        args: &'args FooArgs<'args>
    ) -> flatbuffers::WIPOffset<Foo<'bldr>> {
        let mut builder = FooBuilder::new(_fbb);
        if let Some(x) = args.b { builder.add_b(x); }
        builder.add_a(args.a);
        builder.finish()
    }


    #[inline]
    pub fn a(&self) -> u32 {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<u32>(Foo::VT_A, Some(0)).unwrap()}
    }
    #[inline]
    pub fn b(&self) -> Option<&'a str> {
        // Safety:
        // Created from valid Table for this object
        // which contains a valid value in this slot
        unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Foo::VT_B, None)}
    }
}

impl flatbuffers::Verifiable for Foo<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier, pos: usize
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u32>("a", Self::VT_A, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>("b", Self::VT_B, false)?
            .finish();
        Ok(())
    }
}
pub struct FooArgs<'a> {
    pub a: u32,
    pub b: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for FooArgs<'a> {
    #[inline]
    fn default() -> Self {
        FooArgs {
            a: 0,
            b: None,
        }
    }
}

pub struct FooBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FooBuilder<'a, 'b, A> {
    #[inline]
    pub fn add_a(&mut self, a: u32) {
        self.fbb_.push_slot::<u32>(Foo::VT_A, a, 0);
    }
    #[inline]
    pub fn add_b(&mut self, b: flatbuffers::WIPOffset<&'b  str>) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Foo::VT_B, b);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FooBuilder<'a, 'b, A> {
        let start = _fbb.start_table();
        FooBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Foo<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl core::fmt::Debug for Foo<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("Foo");
        ds.field("a", &self.a());
        ds.field("b", &self.b());
        ds.finish()
    }
}