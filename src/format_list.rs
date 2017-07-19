use format::Format;
use std::cmp::Ordering;

pub type FormatList = Vec<Format>;
pub trait Filter {
    fn extremes(&self, &str, bool) -> Self;
}

impl Filter for FormatList {
    fn extremes(&self, key: &str, best: bool) -> Self {
        let mut formats = self.clone();
        if formats.len() > 1 {
            formats.sort_by(|a, b| a.compare_key(&b, key));
            if !best {
                formats.reverse();
            }

            let mut index = 0;
            {
                let first = &formats[0];
                for i in 0..(formats.len() - 1) {
                    index = i;
                    if first.compare_key(&formats[i + 1], key) != Ordering::Equal {
                        break;
                    }
                }
            }

            index += 1;
            formats.truncate(index);
        }

        formats
    }
}
