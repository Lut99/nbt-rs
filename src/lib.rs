//  LIB.rs
//    by Lut99
//
//  Created:
//    18 Oct 2024, 21:27:03
//  Last edited:
//    18 Oct 2024, 22:01:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Rust implementation of the NBT binary file format.
//

// Declare the modules

// Imports


/***** LIBRARY *****/
/// Defines the possible data types in the NBT specification.
///
/// Note that most of the documentation is taken from <https://wiki.vg/NBT> as well.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TagKind {
    /// Signifies the end of a TAG_Compound.
    ///
    /// It is only ever used inside a TAG_Compound, a TAG_List that has it's type id set to
    /// TAG_Compound or as the type for a TAG_List if the length is 0 or negative, and is not named
    /// even when in a TAG_Compound.
    End,
    /// A single signed byte.
    Byte,
    /// A single signed, big endian 16 bit integer.
    Short,
    /// A single signed, big endian 32 bit integer.
    Int,
    /// A single signed, big endian 64 bit integer.
    Long,
    /// A single, big endian [IEEE-754](http://en.wikipedia.org/wiki/IEEE_754-2008) single-
    /// precision floating point number ([NaN](http://en.wikipedia.org/wiki/NaN) possible).
    Float,
    /// A single, big endian [IEEE-754](http://en.wikipedia.org/wiki/IEEE_754-2008) double-
    /// precision floating point number ([NaN](http://en.wikipedia.org/wiki/NaN) possible).
    Double,
    /// A length-prefixed array of **signed** bytes. The prefix is a **signed** integer (thus 4 bytes).
    ByteArray,
    /// A length-prefixed
    /// [modified UTF-8](https://docs.oracle.com/javase/8/docs/api/java/io/DataInput.html#modified-utf-8)
    /// string. The prefix is an unsigned short (thus 2 bytes) signifying the length of the string
    /// in bytes.
    String,
    /// A list of **nameless** tags, all of the same type. The list is prefixed with the Type ID
    /// of the items it contains (thus 1 byte), and the length of the list as a **signed** integer
    /// (a further 4 bytes). If the length of the list is 0 or negative, the type may be 0
    /// ([TAG_End](TagKind::End)) but otherwise it must be any other type. (The notchian
    /// implementation uses [TAG_End](TagKind::End) in that situation, but another reference
    /// implementation by Mojang uses 1 instead; parsers should accept any type if the length is
    /// <= 0).
    List,
    /// Effectively a list of **named** tags. Order is not guaranteed.
    Compound,
    /// A length-prefixed array of **signed** integers. The prefix is a **signed** integer (thus 4
    /// bytes) and indicates the number of 4 byte integers.
    IntArray,
    /// A length-prefixed array of **signed** longs. The prefix is a **signed** integer (thus 4
    /// bytes) and indicates the number of 8 byte longs.
    LongArray,
}
impl TagKind {
    /// Returns the ID of this Tag.
    ///
    /// # Returns
    /// a [`u8`] that carries the ID, as defined by the spec.
    #[inline]
    pub const fn id(&self) -> u8 {
        match self {
            Self::End => 0,
            Self::Byte => 1,
            Self::Short => 2,
            Self::Int => 3,
            Self::Long => 4,
            Self::Float => 5,
            Self::Double => 6,
            Self::ByteArray => 7,
            Self::String => 8,
            Self::List => 9,
            Self::Compound => 10,
            Self::IntArray => 11,
            Self::LongArray => 12,
        }
    }

    /// Returns the number of bytes expected after the tag.
    ///
    /// If this is a [prefixed tag](TagKind::is_prefixed()), then this is the length of the prefix.
    /// Else, this is the length of the payload.
    ///
    /// # Returns
    /// A [`usize`] that carries the number of bytes to expect.
    #[inline]
    pub const fn len(&self) -> usize {
        match self {
            Self::End => 0,
            Self::Byte => 1,
            Self::Short => 2,
            Self::Int => 4,
            Self::Long => 8,
            Self::Float => 4,
            Self::Double => 8,
            Self::ByteArray => 4,
            Self::String => 2,
            Self::List => 1 + 4,
            Self::Compound => 0,
            Self::IntArray => 4,
            Self::LongArray => 4,
        }
    }

    /// Returns if this tag is prefixed or not.
    ///
    /// Non-prefixed tags have a statically-sized payload. Prefixed tags are dynamically sized, but
    /// have a static-size prefix indicating the length or other options.
    ///
    /// # Returns
    /// True if this tag is **prefixed**, false if it's not.
    #[inline]
    pub const fn is_prefixed(&self) -> bool {
        match self {
            Self::End | Self::Byte | Self::Short | Self::Int | Self::Long | Self::Float | Self::Double => false,
            Self::ByteArray | Self::String | Self::List | Self::Compound | Self::IntArray | Self::LongArray => true,
        }
    }
}
