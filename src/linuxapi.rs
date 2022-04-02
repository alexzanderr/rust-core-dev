use notify_rust::Notification;
use notify_rust::Hint;
use notify_rust::Timeout;


pub fn linux_notification<'a>(
    title: &'a str,
    message: &'a str,
    icon_path: &'a str,
    timeout: u32,
) {
    Notification::new()
        .icon(
            std::path::Path::canonicalize(std::path::Path::new(icon_path))
                .unwrap()
                .to_str()
                .unwrap(),
        )
        .summary(title)
        .body(message)
        .hint(Hint::Resident(true))
        .timeout(Timeout::Milliseconds(timeout * 1000))
        .show()
        .unwrap();
}

// fn send_desktop_notification<'a>(
//     title: &'a str,
//     message: &'a str,
//     icon_path: &'a str,
//     url_to_open: &'a str,
// ) {
//     Notification::new()
//         .icon(icon_path)
//         .summary(title)
//         .body(message)
//         .action("default", "nothing")
//         .hint(Hint::Resident(true))
//         .timeout(Timeout::Milliseconds(8000))
//         .show()
//         .unwrap()
//         .wait_for_action(|action| match action {
//             "default" => {
//                 // open::that(saints_url);
//                 let default_browser =
//                     query_default_app("text/html").unwrap();
//                 println!("{:?}", default_browser);
//                 let url_open_command =
//                     default_browser.replace("%u", url_to_open);
//                 println!("{:?}", url_open_command);
//                 run_shell_command(&url_open_command);
//             },
//             _ => (),
//         });
// }
