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
    /*
    explicit impl(std::string value) : m_value{ std::move(value) } {}
    */
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
    #[test]
    fn value() {}

    /*
       BOOST_AUTO_TEST_CASE(construction)
       {
           BOOST_TEST_PASSPOINT();

           const concrete_input input_{};
       }
    */
    /*
    BOOST_AUTO_TEST_CASE(operator_equal)
    {
        BOOST_TEST_PASSPOINT();

        {
            const concrete_input input1{ 42 };
            const concrete_input input2{ 42 };

            BOOST_CHECK(input1 == input2);
            BOOST_CHECK(input2 == input1);
        }
        {
            const concrete_input input1{ 42 };
            const concrete_input input2{ 24 };

            BOOST_CHECK(input1 != input2);
            BOOST_CHECK(input2 != input1);
        }
        {
            const concrete_input  input1{ 42 };
            const concrete_input2 input2{};

            BOOST_CHECK(input1 != input2);
            BOOST_CHECK(input2 != input1);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(hash_value)
    {
        BOOST_TEST_PASSPOINT();

        const concrete_input input_{};

        [[maybe_unused]] const auto hash_value_ = input_.hash_value();
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(length)
    {
        BOOST_TEST_PASSPOINT();

        const concrete_input input_{};

        BOOST_TEST(input_.length() == 42U);
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(clone)
    {
        BOOST_TEST_PASSPOINT();

        const concrete_input input_{};

        const auto p_clone = input_.clone();
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(create_subrange)
    {
        BOOST_TEST_PASSPOINT();

        const concrete_input input_{};

        {
            const auto p_subrange = input_.create_subrange(0, 42);
        }
        {
            const auto p_subrange = input_.create_subrange(42, 0);
        }
        {
            BOOST_CHECK_THROW(const auto p_subrange = input_.create_subrange(0, 43), std::out_of_range);
        }
        {
            BOOST_CHECK_THROW(const auto p_subrange = input_.create_subrange(43, 0), std::out_of_range);
        }
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(append)
    {
        BOOST_TEST_PASSPOINT();

        concrete_input input_{};

        input_.append(std::make_unique<concrete_input>());
        BOOST_CHECK_THROW(input_.append(nullptr), std::invalid_argument);
        BOOST_CHECK_THROW(input_.append(std::make_unique<concrete_input2>()), std::invalid_argument);
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(is)
    {
        BOOST_TEST_PASSPOINT();

        const tetengo::lattice::input& input_ = concrete_input{};

        BOOST_TEST(input_.is<concrete_input>());
        BOOST_TEST(!input_.is<concrete_input2>());
    }
    */
    /*
    BOOST_AUTO_TEST_CASE(as)
    {
        BOOST_TEST_PASSPOINT();

        {
            const tetengo::lattice::input& input_ = concrete_input{};

            const auto& casted = input_.as<concrete_input>();
            BOOST_TEST(&casted == &input_);
        }
        {
            concrete_input           input_{};
            tetengo::lattice::input& input_ref = input_;

            const auto& casted = input_ref.as<concrete_input>();
            BOOST_TEST(&casted == &input_);
        }
    }
    */
}
