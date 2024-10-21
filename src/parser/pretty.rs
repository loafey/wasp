use crate::hex::Hex;

pub trait Pretty {
    const INDENT: &'static str = "    ";
    fn pretty(&self) -> String {
        self.pretty_indent(0)
    }

    fn pretty_indent(&self, indent: usize) -> String;

    fn get_indent(&self, indent: usize) -> String {
        Self::INDENT.repeat(indent)
    }
}

impl Pretty for () {
    fn pretty_indent(&self, _: usize) -> String {
        "()".to_string()
    }
}

impl<T: Pretty> Pretty for Vec<T> {
    fn pretty_indent(&self, indent: usize) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut s = String::new();
        for i in 0..self.len() {
            s += &self.get_indent(indent);
            s += &self[i].pretty_indent(indent + 1);
            if i + 1 != self.len() {
                s += "\n";
            }
        }
        s
    }
}

impl<const N: usize> Pretty for Hex<N> {
    fn pretty_indent(&self, _: usize) -> String {
        format!("{self:?}")
    }
}
