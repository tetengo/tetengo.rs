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
}
