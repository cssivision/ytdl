use format::Format;
use std::cmp::Ordering;
use format::FormatValue;

pub type FormatList = Vec<Format>;
pub trait Filter {
    fn extremes(&self, &str, bool) -> Self;
    fn subtract(&self, &Self) -> Self;
    fn filter(&self, key: &str, &Vec<&str>) -> Self;
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

    fn subtract(&self, other: &Self) -> Self {
        let mut dst: FormatList = vec![];
        for f1 in self {
            let mut include = true;
            for f2 in other {
                if f1.itag == f2.itag {
                    include = false;
                    break;
                }
            }
            if include {
                dst.push(f1.clone());
            }
        }
        dst 
    }

    fn filter(&self, key: &str, values: &Vec<&str>) -> Self {
        let mut dst: FormatList = vec![];
        for v in values {
            for f in self {
                match f.get_value(key) {
                    FormatValue::Integer(i) => {

                    },
                    FormatValue::String(s) => {

                    },
                    FormatValue::Default => {

                    },
                }
            }
        }
        dst 
    }
}
