use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryValue<'buf>>
}

#[derive(Debug)]
pub enum QueryValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf > QueryString<'buf > {
    pub fn get(&self, key: &str) -> Option<&QueryValue> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    QueryValue::Single(prev_val) => {
                        *existing = QueryValue::Multiple(vec![prev_val,val]);
                    },
                    QueryValue::Multiple(vec) => {vec.push(val)},
                })
                .or_insert(QueryValue::Single(val));

        }

        QueryString { data }
    }
}