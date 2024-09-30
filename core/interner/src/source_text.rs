use alloc::vec::Vec;

#[derive(Debug)]
pub struct SourceText {
    source_text: Vec<u16>,
    callable_parse: u32,
}

impl SourceText {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            source_text: Vec::with_capacity(capacity),
            callable_parse: 0,
        }
    }

    #[inline]
    fn is_callable_parse(&self) -> bool {
        // self.callable_parse != 0
        true
    }

    pub fn inc_callable_parse(&mut self) {
        self.callable_parse += 1;
    }
    pub fn dec_callable_parse(&mut self) {
        if !self.is_callable_parse() {
            panic!("TODO panic msg")
        }
        self.callable_parse -= 1;
        if !self.is_callable_parse() {
            self.source_text.clear();
        }
    }

    pub fn get_source_text_pos(&self) -> usize {
        self.source_text.len()
    }
    pub fn get_source_text_from_pos(&self, pos: usize) -> &[u16] {
        &self.source_text[pos..]
    }

    #[inline]
    pub fn remove_last_code_point(&mut self) {
        self.source_text.pop();
    }

    #[inline]
    pub fn collect_code_point(&mut self, cp: u32) {
        if self.is_callable_parse() {
            if let Ok(cu) = cp.try_into() {
                self.push(cu);
                return;
            }

            let cp = cp - 0x10000;
            let cu1 = (cp / 0x400 + 0xD800)
                .try_into()
                .expect("Invalid code point");
            let cu2 = (cp % 0x400 + 0xDC00)
                .try_into()
                .expect("Invalid code point");
            self.push(cu1);
            self.push(cu2);
        }
    }

    #[inline]
    fn push(&mut self, cp: u16) {
        self.source_text.push(cp);
    } 
}

const DEFAULT_CAPACITY: usize = 4 * 1024;

impl Default for SourceText {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}
