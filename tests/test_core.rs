

#[cfg(test)]
mod test_get_execution_time {

    use core_dev::core::get_execution_time;
    use std::time::Duration;
    use std::thread;
    fn some_long_function() {
        for i in 1..3 {
            let x = i + i;
            thread::sleep(Duration::from_secs(1));
        }
    }

    #[test]
    fn first() {
        get_execution_time(some_long_function);
    }
}

#[cfg(test)]
mod test_print_execution_time {

    use core_dev::core::print_execution_time;
    use std::time::Duration;
    use std::thread;
    fn some_long_function() {
        for i in 1..3 {
            let x = i + i;
            thread::sleep(Duration::from_secs(1));
        }
    }

    #[test]
    fn first() {
        print_execution_time(some_long_function);
    }
}