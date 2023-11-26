/*!
 * A string input.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::hash::Hash;

use anyhow::Result;

use crate::input::{Input, InputError};

/**
 * A string input.
 */
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StringInput {
    value: String,
}

impl StringInput {
    /**
     * Creates a string input key.
     *
     * # Arguments
     * * `value` - A value.
     */
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    /**
     * Returns the value.
     *
     * # Returns
     * The value.
     */
    pub fn value_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl Input for StringInput {
    fn equal_to(&self, other: &dyn Input) -> bool {
        let Some(other) = other.as_any().downcast_ref::<StringInput>() else {
            return false;
        };
        self == other
    }

    fn hash_value(&self) -> u64 {
        42
    }

    fn length(&self) -> usize {
        self.value.len()
    }

    fn clone_box(&self) -> Box<dyn Input> {
        Box::new(self.clone())
    }

    fn create_subrange(&self, offset: usize, length: usize) -> Result<Box<dyn Input>> {
        if offset + length > self.value.len() {
            return Err(InputError::RangeOutOfBounds.into());
        }

        Ok(Box::new(StringInput::new(
            self.value[offset..offset + length].to_string(),
        )))
    }

    fn append(&mut self, another: Box<dyn Input>) -> Result<()> {
        let Some(another) = another.as_any().downcast_ref::<StringInput>() else {
            return Err(InputError::MismatchConcreteType.into());
        };

        self.value += another.value();

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AnotherInput {}

    impl Input for AnotherInput {
        fn equal_to(&self, _: &dyn Input) -> bool {
            unimplemented!()
        }

        fn hash_value(&self) -> u64 {
            unimplemented!()
        }

        fn length(&self) -> usize {
            unimplemented!()
        }

        fn clone_box(&self) -> Box<dyn Input> {
            unimplemented!()
        }

        fn create_subrange(&self, _: usize, _: usize) -> Result<Box<dyn Input>> {
            unimplemented!()
        }

        fn append(&mut self, _: Box<dyn Input>) -> Result<()> {
            unimplemented!()
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn new() {
        let _input = StringInput::new(String::from("hoge"));
    }

    #[test]
    fn value() {
        let input = StringInput::new(String::from("hoge"));

        assert_eq!(input.value(), "hoge");
    }

    #[test]
    fn value_mut() {
        let mut input = StringInput::new(String::from("hoge"));

        *input.value_mut() = String::from("fuga");
        assert_eq!(input.value_mut(), "fuga");
    }

    #[test]
    fn equal_to() {
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("hoge"));

            assert_eq!(input1, input2);
            assert_eq!(input2, input1);
            assert!(input1.equal_to(&input2));
            assert!(input2.equal_to(&input1));
        }
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("fuga"));

            assert_ne!(input1, input2);
            assert_ne!(input2, input1);
            assert!(!input1.equal_to(&input2));
            assert!(!input2.equal_to(&input1));
        }
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = AnotherInput {};

            assert!(!input1.equal_to(&input2));
        }
    }

    // #[test]
    // fn hash_value() {
    //     {
    //         let input1 = StringInput::new(String::from("hoge"));
    //         let input2 = StringInput::new(String::from("hoge"));

    //         assert_eq!(hash_value(&input1), hash_value(&input2));
    //     }
    //     {
    //         let input1 = StringInput::new(String::from("hoge"));
    //         let input2 = StringInput::new(String::from("fuga"));

    //         assert_ne!(hash_value(&input1), hash_value(&input2));
    //     }
    // }

    #[test]
    fn length() {
        let input = StringInput::new(String::from("hoge"));

        assert_eq!(input.length(), 4);
    }

    #[test]
    fn clone_box() {
        let input = StringInput::new(String::from("hoge"));

        let clone = input.clone_box();
        assert!(clone.as_any().is::<StringInput>());
        assert_eq!(
            clone
                .as_any()
                .downcast_ref::<StringInput>()
                .unwrap()
                .value(),
            "hoge"
        );
    }

    #[test]
    fn create_subrange() {
        {
            let input = StringInput::new(String::from("hoge"));

            let subrange = input.create_subrange(0, 4).unwrap();
            assert!(subrange.as_any().is::<StringInput>());
            assert_eq!(
                subrange
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap()
                    .value(),
                "hoge"
            );
        }
        {
            let input = StringInput::new(String::from("hoge"));

            let subrange = input.create_subrange(1, 2).unwrap();
            assert!(subrange.as_any().is::<StringInput>());
            assert_eq!(
                subrange
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap()
                    .value(),
                "og"
            );
        }
        {
            let input = StringInput::new(String::from("hoge"));

            let subrange = input.create_subrange(4, 0).unwrap();
            assert!(subrange.as_any().is::<StringInput>());
            assert_eq!(
                subrange
                    .as_any()
                    .downcast_ref::<StringInput>()
                    .unwrap()
                    .value(),
                ""
            );
        }
        {
            let input = StringInput::new(String::from("hoge"));

            let subrange = input.create_subrange(0, 5);
            assert!(subrange.is_err());
        }
        {
            let input = StringInput::new(String::from("hoge"));

            let subrange = input.create_subrange(5, 0);
            assert!(subrange.is_err());
        }
    }

    #[test]
    fn appand() {
        {
            let mut input = StringInput::new(String::from("hoge"));

            input
                .append(Box::new(StringInput::new(String::from("fuga"))))
                .unwrap();

            assert_eq!(input.value(), "hogefuga");
        }
        {
            let mut input = StringInput::new(String::from("hoge"));

            let result = input.append(Box::new(AnotherInput {}));
            assert!(result.is_err());
        }
    }

    #[test]
    fn as_any() {
        let input = StringInput::new(String::from("hoge"));

        let _ = input.as_any();
    }

    #[test]
    fn as_any_mut() {
        let mut input = StringInput::new(String::from("hoge"));

        let _ = input.as_any_mut();
    }
}
