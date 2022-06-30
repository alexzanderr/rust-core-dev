use crate::traits::StringExtended;

#[derive(Default, Debug, Copy, Clone)]
pub struct TimeAttributes {
    pub millennials: usize,
    pub centuries:   usize,
    pub decades:     usize,
    pub years:       usize,
    pub weeks:       usize,
    pub days:        usize,
    pub hours:       usize,
    pub minutes:     usize,
    pub seconds:     usize,
}


impl TimeAttributes {
    pub fn all_zeros(&self) -> bool {
        self.millennials == 0
            && self.centuries == 0
            && self.decades == 0
            && self.years == 0
            && self.weeks == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
    }


    /// this will format like this
    /// 10d 10h 5m 30s
    pub fn format_with_letters(&self) -> String {
        let mut display = String::new();
        if self.millennials != 0 {
            display.push_str(&format!("{}mil", self.millennials));
        }
        if self.centuries != 0 {
            display.push_str(&format!(" {}cen", self.centuries));
        }
        if self.decades != 0 {
            display.push_str(&format!(" {}dec", self.decades));
        }
        if self.years != 0 {
            display.push_str(&format!(" {}y", self.years));
        }
        if self.weeks != 0 {
            display.push_str(&format!(" {}w", self.weeks));
        }
        if self.days != 0 {
            display.push_str(&format!(" {}d", self.days));
        }
        if self.hours != 0 {
            display.push_str(&format!(" {}h", self.hours));
        }
        if self.minutes != 0 {
            display.push_str(&format!(" {}m", self.minutes));
        }
        if self.seconds != 0 {
            display.push_str(&format!(" {}s", self.seconds));
        } else {
            display.push_str(" 0s");
        }
        display.trim().to_string()
    }

    pub fn format_as_clock_with_level(&self, level: usize) -> String {
        let display = self.format_as_clock();
        let items = display.split_to_vec_str(":");
        // let mut level = 0;
        // for item in items.iter() {
        //     let mut time_to_break = false;
        //     for c in item.chars() {
        //         if c != '0' {
        //             time_to_break = true;
        //             break;
        //         }
        //     }
        //     if time_to_break {
        //         break
        //     }
        //     level += 1;
        // }

        items[items.len() - level..].join(":")
    }

    pub fn format_as_clock(&self) -> String {
        let mut display = String::new();
        if self.millennials != 0 {
            if self.millennials < 10 {
                display.push_str(&format!("000{}:", self.millennials))
            } else if self.millennials < 100 {
                display.push_str(&format!("00{}:", self.millennials))
            } else if self.millennials < 1000 {
                display.push_str(&format!("0{}:", self.millennials))
            } else {
                display.push_str(&format!("{}:", self.millennials))
            }
        } else {
            display.push_str("0000:")
        }
        if self.centuries != 0 {
            if self.centuries < 10 {
                display.push_str(&format!("00{}:", self.centuries))
            } else if self.centuries < 100 {
                display.push_str(&format!("0{}:", self.centuries))
            } else {
                display.push_str(&format!("{}:", self.centuries))
            }
        } else {
            display.push_str("000:")
        }
        if self.decades != 0 {
            if self.decades < 10 {
                dbg!("here");
                display.push_str(&format!("0{}:", self.decades))
            } else {
                display.push_str(&format!("{}:", self.decades))
            }
        } else {
            display.push_str("00:")
        }
        if self.years != 0 {
            if self.years < 10 {
                display.push_str(&format!("0{}:", self.years));
            } else {
                display.push_str(&format!("{}:", self.years));
            }
        } else {
            display.push_str("00:")
        }
        if self.weeks != 0 {
            if self.weeks < 10 {
                display.push_str(&format!("0{}:", self.weeks));
            } else {
                display.push_str(&format!("{}:", self.weeks));
            }
        } else {
            display.push_str("00:")
        }
        if self.days != 0 {
            if self.days < 10 {
                display.push_str(&format!("0{}:", self.days));
            } else {
                display.push_str(&format!("{}:", self.days));
            }
        } else {
            display.push_str("0:")
        }
        if self.hours != 0 {
            if self.hours < 10 {
                display.push_str(&format!("0{}:", self.hours));
            } else {
                display.push_str(&format!("{}:", self.hours));
            }
        } else {
            display.push_str("00:")
        }
        if self.minutes != 0 {
            if self.minutes < 10 {
                display.push_str(&format!("0{}:", self.minutes));
            } else {
                display.push_str(&format!("{}:", self.minutes));
            }
        } else {
            display.push_str("00:")
        }
        if self.seconds != 0 {
            if self.seconds < 10 {
                display.push_str(&format!("0{}", self.seconds));
            } else {
                display.push_str(&format!("{}", self.seconds));
            }
        } else {
            display.push_str("00")
        }

        display
    }

    pub fn decrement_seconds(&mut self, by: usize) {
        if self.seconds == 0 {
            self.normalize_decrement();
        }
        self.seconds -= by;
    }

    pub fn decrement_seconds_and_normalize(&mut self, by: usize) {
        self.seconds -= by;
        self.normalize_decrement();
    }

    pub fn print_clock(&self, level: usize) {
        let clock = self.format_as_clock_with_level(level);
        println!("{}", clock);
    }

    pub fn normalize_decrement(&mut self) {
        if self.seconds == 0 && self.minutes > 0 {
            // dbg!(&self.seconds);
            self.minutes -= 1;
            // dbg!(&self.minutes);
            self.seconds = 60;
            if self.minutes == 0 && self.hours > 0 {
                self.minutes = 60;
                self.hours -= 1;
                if self.hours == 0 && self.days > 0 {
                    self.days -= 1;
                    self.hours = 24;
                    if self.days == 0 && self.weeks > 0 {
                        self.weeks -= 1;
                        self.days = 7;
                        if self.weeks == 0 && self.years > 0 {
                            self.years -= 1;
                            self.weeks = 52;
                            if self.years == 0 && self.decades > 0 {
                                self.decades -= 1;
                                self.years = 10;
                                if self.decades == 0
                                    && self.centuries > 0
                                {
                                    self.centuries -= 1;
                                    self.decades = 10;
                                    if self.centuries == 0
                                        && self.millennials > 0
                                    {
                                        self.millennials -= 1;
                                        self.centuries = 10;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn normalize(&mut self) {
        // 1010 / 60 == 16.83
        // 1010 % 60 == 50
        while self.seconds > 60 {
            self.minutes += 1;
            self.seconds -= 60;
        }

        while self.minutes > 60 {
            self.hours += 1;
            self.minutes -= 60;
        }

        while self.hours > 24 {
            self.days += 1;
            self.hours -= 24;
        }

        while self.days > 7 {
            self.weeks += 1;
            self.days -= 7;
        }


        while self.weeks > 52 {
            self.years += 1;
            self.weeks -= 52;
        }

        while self.years > 10 {
            self.decades += 1;
            self.years -= 10;
        }

        while self.decades > 10 {
            self.centuries += 1;
            self.decades -= 10;
        }

        while self.centuries > 10 {
            self.millennials += 1;
            self.centuries -= 10;
        }
    }

    pub fn get_level(&self) -> usize {
        // pub millennials: usize,
        // pub centuries:   usize,
        // pub decades:     usize,
        // pub years:       usize,
        // pub weeks:       usize,
        // pub days:        usize,
        // pub hours:       usize,
        // pub minutes:     usize,
        // pub seconds:     usize,
        let mut max_level = 9usize;
        if self.millennials == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }

        if self.centuries == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }

        if self.decades == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.years == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.weeks == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.days == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.hours == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.minutes == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }


        if self.seconds == 0 {
            max_level -= 1;
        } else {
            return max_level;
        }

        max_level
    }
}

#[derive(Debug)]
pub struct TimeAttributesBuilder {
    pub time_attributes: TimeAttributes,
}


impl std::fmt::Display for TimeAttributes {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        unimplemented!()
    }
}

impl TimeAttributesBuilder {
    pub fn default() -> Self {
        TimeAttributesBuilder {
            time_attributes: TimeAttributes::default(),
        }
    }

    pub fn seconds(mut self, seconds: usize) -> Self {
        self.time_attributes.seconds += seconds;
        self
    }

    pub fn minutes(mut self, minutes: usize) -> Self {
        self.time_attributes.minutes += minutes;
        self
    }

    pub fn hours(mut self, hours: usize) -> Self {
        self.time_attributes.hours += hours;
        self
    }

    pub fn days(mut self, days: usize) -> Self {
        self.time_attributes.days += days;
        self
    }

    pub fn weeks(mut self, weeks: usize) -> Self {
        self.time_attributes.weeks += weeks;
        self
    }

    pub fn years(mut self, years: usize) -> Self {
        self.time_attributes.years += years;
        self
    }

    pub fn decades(mut self, decades: usize) -> Self {
        self.time_attributes.decades += decades;
        self
    }

    pub fn centuries(mut self, centuries: usize) -> Self {
        self.time_attributes.centuries += centuries;
        self
    }

    pub fn millennials(mut self, millennials: usize) -> Self {
        self.time_attributes.millennials += millennials;
        self
    }

    pub fn normalize(mut self) -> Self {
        self.time_attributes.normalize();
        self
    }

    pub fn build(mut self) -> TimeAttributes {
        self.time_attributes
    }
}
