/*!
 * A value serializer.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::fmt;

/**
 * A value serializer.
 */
#[derive(Clone, Copy)]
pub struct ValueSerializer<T> {
    serialize: fn(value: &T) -> Vec<u8>,

    fixed_value_size: usize,
}

impl<T> fmt::Debug for ValueSerializer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValueSerializer")
            .field("serialize", &"<fn>")
            .field("fixed_value_size", &self.fixed_value_size)
            .finish()
    }
}

impl<T> ValueSerializer<T> {
    /**
     * Creates a value serializer.
     *
     * # Arguments
     * * `serialize`        - A serializing function.
     * * `fixed_value_size` - The value size if it is fixed. Or 0 if the size is variable.
     */
    pub fn new(serialize: fn(value: &T) -> Vec<u8>, fixed_value_size: usize) -> Self {
        Self {
            serialize,
            fixed_value_size,
        }
    }

    /**
     * Serializes a value.
     *
     * # Arguments
     * * `value` - A value.
     *
     * # Returns
     * The serialized value.
     */
    pub fn serialize(&self, value: &T) -> Vec<u8> {
        (self.serialize)(value)
    }

    /**
     * Returns the fixed value size.
     *
     * # Returns
     * The fixed value size.
     */
    pub fn fixed_value_size(&self) -> usize {
        self.fixed_value_size
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn new() {
        {
            let _ = ValueSerializer::new(|_value: &i32| return Vec::new(), size_of::<i32>());
        }
        {
            let _ = ValueSerializer::new(|_: &i32| return vec![3, 1, 4], 0);
        }
    }
}
