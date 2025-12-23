#[derive(Debug)]
pub struct RFloat {
    pub float: f64,
}
pub const RESTRICT_DIGITS: usize = 2;
impl std::fmt::Display for RFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.float)
    }
}
#[allow(unused)]
impl RFloat {
    pub fn new(float: f64, restricted_number: usize) -> Option<Self> {
        if float.is_nan() {
            eprintln!("The Float you Provided is NaN!");
            return None;
        } else if float.is_infinite() {
            eprintln!("Can't Restrict infinity!");
            return None;
        }

        //This shouldn't contain any NaNs... nor infinities
        let mut stringed_float = float.to_string();
        // Away with thee E's!
        if stringed_float.contains('e') || stringed_float.contains('E') {
            // base is like 1.06942 and power would be a int i think
            stringed_float = format!("{:.50}", float)
        }
        if !stringed_float.contains(".") {
            stringed_float.push_str(".0");
        };
        // //chuck middling zeros if they're too rowady
        let (base, float_part) = stringed_float.split_once('.')?;

        let mut runs: Vec<(usize, usize)> = vec![];
        let mut count: usize = 0;
        let mut start: usize = 0;

        for (i, ch) in float_part.chars().enumerate() {
            if ch == '0' {
                if count == 0 {
                    start = i;
                }
                count += 1;
            } else {
                if count >= 7 {
                    runs.push((start, i - 1));
                }
                count = 0;
            }
        }

        // handle trailing zeros
        if count >= 7 {
            let last_index = float_part.chars().count() - 1;
            runs.push((start, last_index));
        }

        let mut seen: Vec<char> = float_part.chars().collect();

        for (start, end) in runs.into_iter().rev() {
            seen.drain(start..=end);
        }
        let cleaned_float_part: String = seen.into_iter().collect();

        stringed_float = format!(
            "{}.{}",
            base,
            cleaned_float_part
                .chars()
                .take(restricted_number)
                .collect::<String>()
        );

        let out = stringed_float.parse::<f64>().ok()?;
        Some(Self { float: out })
    }
    pub fn restrict_self(&mut self) -> &mut Self {
        self.float = RFloat::new(self.float, RESTRICT_DIGITS)
            .expect("Invaild Float Variant!")
            .float; //<---- this will bite my ankles
        self
    }
}
impl PartialEq for RFloat {
    fn eq(&self, other: &Self) -> bool {
        self.float == other.float
    }
}
impl Eq for RFloat {}
