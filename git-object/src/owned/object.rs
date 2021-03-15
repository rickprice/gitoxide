use crate::owned;
use std::io;

/// A mutable object representing [`Trees`][owned::Tree], [`Blobs`][owned::Blob], [`Commits`][owned::Commit] or [`Tags`][owned::Tag].
#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant, missing_docs)]
pub enum Object {
    Tree(owned::Tree),
    Blob(owned::Blob),
    Commit(owned::Commit),
    Tag(owned::Tag),
}

/// Convenient extraction of typed object.
impl Object {
    /// Returns a [`Blob`][owned::Blob] if it is one.
    #[must_use]
    pub fn as_blob(&self) -> Option<&owned::Blob> {
        match self {
            Self::Blob(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Commit`][owned::Commit] if it is one.
    #[must_use]
    pub fn as_commit(&self) -> Option<&owned::Commit> {
        match self {
            Self::Commit(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tree`][owned::Tree] if it is one.
    #[must_use]
    pub fn as_tree(&self) -> Option<&owned::Tree> {
        match self {
            Self::Tree(v) => Some(v),
            _ => None,
        }
    }
    /// Returns a [`Tag`][owned::Tag] if it is one.
    #[must_use]
    pub fn as_tag(&self) -> Option<&owned::Tag> {
        match self {
            Self::Tag(v) => Some(v),
            _ => None,
        }
    }
    /// Returns the kind of object stored in this instance.
    #[must_use]
    pub fn kind(&self) -> crate::Kind {
        match self {
            Self::Tree(_) => crate::Kind::Tree,
            Self::Blob(_) => crate::Kind::Blob,
            Self::Commit(_) => crate::Kind::Commit,
            Self::Tag(_) => crate::Kind::Tag,
        }
    }
}

/// Serialization
impl Object {
    /// Write the contained object to `out` in the git serialization format.
    pub fn write_to(&self, out: impl io::Write) -> io::Result<()> {
        use Object::{Blob, Commit, Tag, Tree};
        match self {
            Tree(v) => v.write_to(out),
            Blob(v) => v.write_to(out),
            Commit(v) => v.write_to(out),
            Tag(v) => v.write_to(out),
        }
    }
}

mod convert {
    use crate::owned::{Blob, Commit, Object, Tag, Tree};
    use std::convert::TryFrom;

    impl From<Tag> for Object {
        fn from(v: Tag) -> Self {
            Self::Tag(v)
        }
    }

    impl From<Commit> for Object {
        fn from(v: Commit) -> Self {
            Self::Commit(v)
        }
    }

    impl From<Tree> for Object {
        fn from(v: Tree) -> Self {
            Self::Tree(v)
        }
    }

    impl From<Blob> for Object {
        fn from(v: Blob) -> Self {
            Self::Blob(v)
        }
    }

    impl TryFrom<Object> for Tag {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tag(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Commit {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Commit(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Tree {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Tree(v) => v,
                _ => return Err(value),
            })
        }
    }

    impl TryFrom<Object> for Blob {
        type Error = Object;

        fn try_from(value: Object) -> Result<Self, Self::Error> {
            Ok(match value {
                Object::Blob(v) => v,
                _ => return Err(value),
            })
        }
    }
}
