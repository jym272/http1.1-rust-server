use std::collections::HashMap;

#[derive(Debug)]
pub struct Query<'buffer> {
    data: HashMap<&'buffer str, Value<'buffer>>,
}
#[derive(Debug)]
enum Value<'buffer> { //the space allocated in the heap for the value is the most weighted variant
    Single(&'buffer str),
    Multiple(Vec<&'buffer str>), //heap dynamically allocated
}

impl<'buffer> Query<'buffer> {
    fn get(&self, key: &'buffer str) -> Option<&Value<'buffer>> {
        self.data.get(key)
    }
}

impl<'buffer> From<&'buffer str> for Query<'buffer> {
    // a=1&b=2&c&d=&e===&d=7&d=abc
    fn from(query: &'buffer str) -> Query {
        let mut data = HashMap::new();

        query.split('&').for_each(|pair| {
            if let Some(index) = pair.find('=') {
                let key = &pair[..index];
                let value = &pair[index + 1..];


                data.entry(key)
                    .and_modify(|existing: &mut Value|
                        match existing {
                            Value::Single(prevValue) => {
                                *existing = Value::Multiple(vec![prevValue, value]);
                            }
                            Value::Multiple(values) => {
                                values.push(value);
                            }


                        })
                    .or_insert(Value::Single(value));

                // if data.contains_key(key) {
                //
                //     match data.get_mut(key) {
                //
                //         Some(Value::Single(v)) => {
                //             let mut multiple: Vec<&str> = vec![v, value];
                //             data.insert(key, Value::Multiple(multiple));
                //         }
                //         Some(Value::Multiple(v)) => {
                //             v.push(value);
                //         }
                //         None => {
                //             data.insert(key, Value::Single(value));
                //         }
                //
                //     }
                // } else {
                //     data.insert(key, Value::Single(value));
                // }
            }

        });
        Query { data }
    }
}