use std::{collections::HashSet, fmt::Write};

use syn::Ident;

#[derive(Default)]
pub(super) struct SafeNameCast {
    buffer: String,
    registered_safe_name: HashSet<String>,
}

impl SafeNameCast {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to_safe_name(&mut self, name: &str) -> Ident {
        self.to_safe_name_string(name);
        self.register_name()
    }

    pub fn clear(&mut self) {
        self.registered_safe_name.clear();
    }

    fn to_safe_name_string(&mut self, name: &str) {
        let buffer = &mut self.buffer;
        buffer.clear();

        let mut previous_is_unsound_char = true;
        let mut is_first = true;
        for ch in name.chars() {
            if is_first {
                is_first = false;
                if let '0'..'9' = ch {
                    buffer.push('_');
                }
            }

            if is_sound_char(ch) {
                buffer.push(ch);
            } else if !previous_is_unsound_char {
                buffer.push('_');
                previous_is_unsound_char = true;
            }
        }

        if let "" | "_" = buffer.as_str() {
            buffer.clear();
            // fallback to this name
            buffer.push_str("__Unnamed");
        }
    }

    fn register_name(&mut self) -> Ident {
        if let Some(ident) = self.try_register() {
            return ident;
        }

        if !self.buffer.ends_with('_') {
            self.buffer.push('_');
        }
        let truncate_len = self.buffer.len();

        let mut suffix = 2;
        loop {
            write!(&mut self.buffer, "{suffix}").unwrap();
            if let Some(ident) = self.try_register() {
                return ident;
            }
            self.buffer.truncate(truncate_len);
            suffix += 1;
        }
    }

    fn try_register(&mut self) -> Option<Ident> {
        if self.registered_safe_name.contains(&self.buffer) {
            return None;
        }
        self.registered_safe_name.insert(self.buffer.clone());
        syn::parse_str::<Ident>(&self.buffer).ok()
    }
}

fn is_sound_char(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}
