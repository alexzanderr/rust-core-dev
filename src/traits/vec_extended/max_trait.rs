pub trait MaxVector<T> {
    fn find_max(&self) -> Option<T>;
}

impl MaxVector<i8> for Vec<i8> {
    fn find_max(&self) -> Option<i8> {
        if self.is_empty() {
            return None;
        }

        let mut maximum = self[0];
        for integer in self.iter().skip(1) {
            if *integer > maximum {
                maximum = *integer;
            }
        }
        Some(maximum)
    }
}

impl MaxVector<i16> for Vec<i16> {
    fn find_max(&self) -> Option<i16> {
        if self.is_empty() {
            return None;
        }

        let mut maximum = self[0];
        for integer in self.iter().skip(1) {
            if *integer > maximum {
                maximum = *integer;
            }
        }
        Some(maximum)
    }
}

impl MaxVector<i32> for Vec<i32> {
    fn find_max(&self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut maximum = self[0];
        for integer in self.iter().skip(1) {
            if *integer > maximum {
                maximum = *integer;
            }
        }
        Some(maximum)
    }
}

impl MaxVector<i64> for Vec<i64> {
    fn find_max(&self) -> Option<i64> {
        if self.is_empty() {
            return None;
        }

        let mut maximum = self[0];
        for integer in self.iter().skip(1) {
            if *integer > maximum {
                maximum = *integer;
            }
        }
        Some(maximum)
    }
}

impl MaxVector<i128> for Vec<i128> {
    fn find_max(&self) -> Option<i128> {
        if self.is_empty() {
            return None;
        }

        let mut maximum = self[0];
        for integer in self.iter().skip(1) {
            if *integer > maximum {
                maximum = *integer;
            }
        }
        Some(maximum)
    }
}
