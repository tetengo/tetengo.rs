/*!
 * A string input.
 *
 * Copyright 2023 kaoru  <https://www.tetengo.org/>
 */

use crate::input::Input;

/**
 * A string input.
 */
#[derive(Debug)]
pub struct StringInput {
    _value: String,
}

impl StringInput {
    /**
     * Creates a string input key.
     *
     * # Arguments
     * * `value` - A value.
     */
    pub fn new(value: String) -> Self {
        Self { _value: value }
    }

    /*
        const std::string& value() const
        {
            return m_value;
        }
    */
    /*
        std::string& value()
        {
            return m_value;
        }
    */
}

impl Input for StringInput {
    fn eq(&self, _another: &dyn Input) -> bool {
        todo!()
    }
    /*
        bool equal_to_impl(const input& another) const
        {
            return another.as<string_input>().value() == m_value;
        }
    */
    fn hash(&self) -> usize {
        todo!()
    }
    /*
        std::size_t hash_value_impl() const
        {
            return boost::hash_value(m_value);
        }
    */
    fn length(&self) -> usize {
        todo!()
    }
    /*
        std::size_t length_impl() const
        {
            return m_value.length();
        }
    */
    fn clone_box(&self) -> Box<dyn Input> {
        todo!()
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
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _input = StringInput::new("hoge".to_string());
    }

    /*
    BOOST_AUTO_TEST_CASE(construction)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input{ "hoge" };
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(value)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input{ "hoge" };

            BOOST_TEST(input.value() == "hoge");
        }
        {
            tetengo::lattice::string_input input{ "hoge" };

            input.value() = "fuga";
            BOOST_TEST(input.value() == "fuga");
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(operator_equal)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const tetengo::lattice::string_input input2{ "hoge" };

            BOOST_CHECK(input1 == input2);
            BOOST_CHECK(input2 == input1);
        }
        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const tetengo::lattice::string_input input2{ "fuga" };

            BOOST_CHECK(input1 != input2);
            BOOST_CHECK(input2 != input1);
        }
        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const another_input                  input2{};

            BOOST_CHECK(input1 != input2);
            BOOST_CHECK(input2 != input1);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(hash_value)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const tetengo::lattice::string_input input2{ "hoge" };

            BOOST_TEST(input1.hash_value() == input2.hash_value());
        }
        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const tetengo::lattice::string_input input2{ "fuga" };

            BOOST_TEST(input1.hash_value() != input2.hash_value());
        }
        {
            const tetengo::lattice::string_input input1{ "hoge" };
            const another_input                  input2{};

            BOOST_TEST(input1.hash_value() != input2.hash_value());
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(length)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input{ "hoge" };

            BOOST_TEST(input.length() == 4U);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(clone)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::string_input input{ "hoge" };

            const auto p_clone = input.clone();
            BOOST_REQUIRE(p_clone);
            BOOST_TEST_REQUIRE(p_clone->is<tetengo::lattice::string_input>());
            BOOST_TEST(p_clone->as<tetengo::lattice::string_input>().value() == "hoge");
        }
    }
    */
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
