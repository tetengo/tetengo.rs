/*!
 * A string input.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use std::any::Any;
use std::hash::Hash;

use crate::input::Input;

/**
 * A string input.
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    fn length(&self) -> usize {
        self.value.len()
    }

    fn clone_box(&self) -> Box<dyn Input> {
        Box::new(self.clone())
    }

    /*
        std::unique_ptr<input> clone_impl() const
        {
            return std::make_unique<string_input>(m_value);
        }
    */
    fn create_subrange(&self, _offset: usize, _lengthh: usize) -> anyhow::Result<Box<dyn Input>> {
        todo!()
    }
    /*
        std::unique_ptr<input> create_subrange_impl(const std::size_t offset, const std::size_t length) const
        {
            if (offset + length > m_value.length())
            {
                throw std::out_of_range{ "offset and/or length are out of the range." };
            }

            return std::make_unique<string_input>(m_value.substr(offset, length));
        }
    */
    fn append(&mut self, _another: Box<dyn Input>) -> anyhow::Result<()> {
        todo!()
    }
    /*
    void append_impl(std::unique_ptr<input>&& p_another)
    {
        if (!p_another)
        {
            throw std::invalid_argument{ "p_another is nullptr." };
        }
        if (!p_another->is<string_input>())
        {
            throw std::invalid_argument{ "Mismatch type of p_another." };
        }

        m_value += std::move(p_another->as<string_input>().value());
    }
    */
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    use super::*;

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
    fn eq() {
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("hoge"));

            assert_eq!(input1, input2);
            assert_eq!(input2, input1);
        }
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("fuga"));

            assert_ne!(input1, input2);
            assert_ne!(input2, input1);
        }
    }

    fn hash_value(input: &StringInput) -> u64 {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash() {
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("hoge"));

            assert_eq!(hash_value(&input1), hash_value(&input2));
        }
        {
            let input1 = StringInput::new(String::from("hoge"));
            let input2 = StringInput::new(String::from("fuga"));

            assert_ne!(hash_value(&input1), hash_value(&input2));
        }
    }

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

    /*
    BOOST_AUTO_TEST_CASE(create_subrange)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input{ "hoge" };

            const auto p_subrange = input.create_subrange(0, 4);
            BOOST_REQUIRE(p_subrange);
            BOOST_TEST_REQUIRE(p_subrange->is<tetengo::lattice::string_input>());
            BOOST_TEST(p_subrange->as<tetengo::lattice::string_input>().value() == "hoge");
        }
        {
            const tetengo::lattice::string_input input{ "hoge" };

            const auto p_subrange = input.create_subrange(1, 2);
            BOOST_REQUIRE(p_subrange);
            BOOST_TEST_REQUIRE(p_subrange->is<tetengo::lattice::string_input>());
            BOOST_TEST(p_subrange->as<tetengo::lattice::string_input>().value() == "og");
        }
        {
            const tetengo::lattice::string_input input{ "hoge" };

            const auto p_subrange = input.create_subrange(4, 0);
            BOOST_REQUIRE(p_subrange);
            BOOST_TEST_REQUIRE(p_subrange->is<tetengo::lattice::string_input>());
            BOOST_TEST(p_subrange->as<tetengo::lattice::string_input>().value() == "");
        }
        {
            const tetengo::lattice::string_input input{ "hoge" };

            BOOST_CHECK_THROW(const auto p_subrange = input.create_subrange(0, 5), std::out_of_range);
        }
        {
            const tetengo::lattice::string_input input{ "hoge" };

            BOOST_CHECK_THROW(const auto p_subrange = input.create_subrange(5, 0), std::out_of_range);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(append)
    {
        BOOST_TEST_PASSPOINT();

        {
            tetengo::lattice::string_input input{ "hoge" };

            input.append(std::make_unique<tetengo::lattice::string_input>("fuga"));

            BOOST_TEST(input.value() == "hogefuga");
        }
        {
            tetengo::lattice::string_input input{ "hoge" };

            BOOST_CHECK_THROW(input.append(nullptr), std::invalid_argument);
            BOOST_CHECK_THROW(input.append(std::make_unique<another_input>()), std::invalid_argument);
        }
    }
    */
}
