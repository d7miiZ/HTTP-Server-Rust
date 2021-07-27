use std::collections::HashMap;
use std::convert::From;

pub struct Query<'buf_life> {
    map: HashMap<&'buf_life str, Value<'buf_life>>,
}

impl<'buf_life> Query<'buf_life> {
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.map.get(key)
    }
}

pub enum Value<'buf_life> {
    Single(&'buf_life str),
    Many(Vec<&'buf_life str>),
}

impl<'buf_life> From<&'buf_life str> for Query<'buf_life> {
    fn from(query: &'buf_life str) -> Self {
        let mut hash_map = HashMap::new();
        for sub_query in query.split('&') {
            let mut key = sub_query;
            let mut value = "";
            if let Some(index) = sub_query.find('=') {
                key = &sub_query[..index];
                value = &sub_query[index + 1..];
            }

            hash_map
                .entry(key)
                .and_modify(|element| match element {
                    Value::Single(prev_element) => {
                        *element = Value::Many(vec![prev_element, value]);
                    }
                    Value::Many(prev_vec) => prev_vec.push(value),
                })
                .or_insert(Value::Single(value));
        }

        Query { map: hash_map }
    }
}
