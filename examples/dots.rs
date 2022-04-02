use core_dev::spinners::SpinnerDotsThread;

use std::thread::sleep;
use std::time::Duration;

use std::io::stdout;
use std::io::Write;

use color_backtrace::install as _color_backtrace_install;


fn worker(iterations: i32) -> i32 {
    let mut counter = 0;
    for _iter in 1..iterations {
        counter += 1;
        // println!("asd");
        sleep(Duration::from_millis(100));
    }
    iterations
}

fn main() {
    _color_backtrace_install();

    let message = "random message ...".to_string();
    let termination_message = "done here".to_string();

    let iterations = 32;
    let closure = Box::new(move || {
        {
            worker(iterations);
            String::from("its working");
            Err("salutare")
        }
        .unwrap()
    });


    let result = SpinnerDotsThread::<String>::run_default(closure.clone());
    if let Err(result) = result {
        println!("{}", result);
    }
    // let result = SpinnerDotsThread::<String>::run_with_args(
    //     message,
    //     closure,
    //     termination_message,
    // );
    // println!("{:?}", result.err());

    // let spinner = SpinnerDotsThread::new(
    //     "📦 loading ... ".to_string(),
    //     move || {
    //         worker(iterations);
    //     },
    //     "📦 terminated successfully".to_string(),
    // );

    // spinner.run();

    // fn worker2(iterations: i32) -> i32 {
    //     let mut counter = 0;
    //     for _iter in 1..iterations {
    //         counter += 1;
    //         // println!("asd");
    //         sleep(Duration::from_millis(100));
    //     }
    //     iterations
    // }

    // let spinner = SpinnerDotsThread::new(
    //     "📦 loading ... ".to_string(),
    //     move || {
    //         worker2(iterations);
    //     },
    //     "📦 terminated successfully".to_string(),
    // );

    // spinner.run();

    // its working because i32 has Copy trait implemented
    // with String it wouldnt work
    // println!("{}", iterations);
}
