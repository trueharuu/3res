use std::fmt::Write;
use std::str::FromStr;
use rustc_hash::FxHashMap;

pub struct Refv<'a> {
    scope: FxHashMap<&'a str, Value<'a>>,
}

pub enum Value<'a> {
    Content(&'a str),
    Ref(&'a str),
}

pub enum RefvParseErr {
    Invalid(usize),
}

impl<'a> Refv<'a> {
    pub fn new(raw: &'a str) -> Result<Self, RefvParseErr> {
        let mut scope = FxHashMap::default();

        for (idx, line) in raw.lines().map(str::trim_start).enumerate() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let eq_idx = line.find('=');
            if let Some(i) = eq_idx {
                let key = &line[0..i];
                let raw_value = &line[i + 1..];
                let value = if let Some(val) = raw_value.strip_prefix('&') {
                    Value::Ref(val)
                } else {
                    Value::Content(raw_value)
                };

                scope.insert(key, value);
            } else {
                return Err(RefvParseErr::Invalid(idx));
            }
        }

        Ok(Self { scope })
    }

    #[must_use]
    pub fn len(&'a self) -> usize {
        self.scope.len()
    }

    #[must_use]
    pub fn is_empty(&'a self) -> bool {
        self.scope.is_empty()
    }

    #[must_use]
    pub fn get_raw(&'a self, mut label: &'a str) -> Option<&'a str> {
        loop {
            let x = self.scope.get(label);
            if let Some(value) = x {
                match value {
                    Value::Ref(t) => {
                        label = *t;
                        continue;
                    }

                    Value::Content(t) => {
                        return Some(*t);
                    }
                }
            }
            return None;
        }
    }

    #[must_use]
    pub fn get<T>(&'a self, label: &'a str) -> Option<Result<T, <T as FromStr>::Err>>
    where
        T: FromStr,
    {
        self.get_raw(label).map(str::parse)
    }

    #[must_use]
    pub fn deserialize(self) -> Option<String> {
        let mut f = String::new();
        for k in self.scope.keys() {
            let vv = self.get_raw(k)?;

            writeln!(f, "{k}={vv}").ok()?;
        }

        Some(f)
    }
}
