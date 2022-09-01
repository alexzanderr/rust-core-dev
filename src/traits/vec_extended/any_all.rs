pub trait All_Any_String {
    fn all<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&String) -> bool;
    fn any<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&String) -> bool;
}

impl All_Any_String for Vec<String> {
    fn all<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&String) -> bool {
        for item in self.iter() {
            if !closure(item) {
                return false;
            }
        }
        true
    }

    fn any<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&String) -> bool {
        for item in self.iter() {
            if closure(item) {
                return true;
            }
        }
        false
    }
}

pub trait All_Any_Str {
    fn all<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&str) -> bool;
    fn any<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&str) -> bool;
}

impl All_Any_Str for Vec<&str> {
    fn all<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&str) -> bool {
        for item in self.iter() {
            if !closure(item) {
                return false;
            }
        }
        true
    }

    fn any<F>(
        &self,
        closure: F
    ) -> bool
    where
        F: Fn(&str) -> bool {
        for item in self.iter() {
            if closure(item) {
                return true;
            }
        }
        false
    }
}
