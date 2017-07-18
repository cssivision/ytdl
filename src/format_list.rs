use format::Format;

pub type FormatList = Vec<Format>;
trait Filter {
    fn extremes(self, &str, bool) -> Self;
}

impl Filter for FormatList {
    fn extremes(self, key: &str, best: bool) -> Self {
        if self.len() > 1 {

        }
        self 
    }
}