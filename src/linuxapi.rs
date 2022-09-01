use notify_rust::Notification;
use notify_rust::Hint;
use notify_rust::Timeout;
use notify_rust::Image;

pub fn linux_notification<'a>(
    title: &'a str,
    message: &'a str,
    icon_path: &'a str,
    timeout: u32,
) {
    Notification::new()
        .icon(
            std::path::Path::canonicalize(std::path::Path::new(
                icon_path,
            ))
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

use image::ImageBuffer;
use image::Rgba;

/// how to call this function
/// static image_bytes: &[u8] = include_bytes!("../../static/icons/rule-202020.png")
/// let dynamic_image =
///     image::load_from_memory(NOTIFICATION_ICON).unwrap();
/// let image_buffer = dynamic_image.to_rgba8();
/// linux_notification_from_image_buffer(
///     "its working",
///     "i've sent a notification from image loaded into the binary file of the app",
///     image_buffer,
///     10);
pub fn linux_notification_from_image_buffer_async<'a>(
    title: &'a str,
    message: &'a str,
    image_buffer: Option<&ImageBuffer<Rgba<u8>, Vec<u8>>>,
    timeout: u32,
) {
    let mut notif_handler = Notification::new();
    let mut notif_handler = notif_handler.summary(title);

    // if let Some(actions) = actions {
    //     for (identifier, label) in actions.iter() {
    //         notif_handler = notif_handler.action(identifier, label);
    //     }
    // }

    if let Some(image_buffer) = image_buffer {
        let image_data = image_buffer.to_vec();
        let height = image_buffer.height() as i32;
        let width = image_buffer.width() as i32;
        let image =
            Image::from_rgba(width, height, image_data).unwrap();
        notif_handler = notif_handler.image_data(image);
    }
    // .action("restart", "restart")
    notif_handler
        .body(message)
        .hint(Hint::Resident(true))
        .timeout(Timeout::Milliseconds(timeout))
        .show()
        .unwrap();
}

pub fn linux_notification_from_image_buffer_and_wait<'a, F>(
    title: &'a str,
    message: &'a str,
    image_buffer: Option<&ImageBuffer<Rgba<u8>, Vec<u8>>>,
    timeout: u32,
    actions: Option<&[(&str, &str)]>,
    invocation_closure: F,
) where
    F: FnOnce(&str),
{
    let mut notif_handler = Notification::new();
    let mut notif_handler = notif_handler.summary(title);

    if let Some(actions) = actions {
        for (identifier, label) in actions.iter() {
            notif_handler = notif_handler.action(identifier, label);
        }
    }

    if let Some(image_buffer) = image_buffer {
        let image_data = image_buffer.to_vec();
        let height = image_buffer.height() as i32;
        let width = image_buffer.width() as i32;
        let image =
            Image::from_rgba(width, height, image_data).unwrap();
        notif_handler = notif_handler.image_data(image);
    }
    // .action("restart", "restart")
    notif_handler
        .body(message)
        .hint(Hint::Resident(true))
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .wait_for_action(invocation_closure)
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

#[derive(Debug)]
pub struct NotificationHandler {
    title: String,
    image_buffer: Option<ImageBufferType>,
    timeout: u64,
}

#[cfg(feature = "image")]
use crate::imagelib::create_image_buffer_from_bytes;
#[cfg(feature = "image")]
use crate::imagelib::ImageBufferType;

impl NotificationHandler {
    pub fn new(
        title: &str,
        image_bytes: Option<&[u8]>,
        timeout: u64,
    ) -> Self {
        let title = title.to_string();
        match image_bytes {
            Some(image_bytes) => {
                let image_buffer =
                    create_image_buffer_from_bytes(image_bytes);
                Self {
                    title,
                    image_buffer: Some(image_buffer),
                    timeout,
                }
            },
            None => Self {
                title,
                image_buffer: None,
                timeout,
            },
        }
    }

    pub fn timeout(
        &mut self,
        timeout: u64,
    ) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn send_notification_async(
        &self,
        message: &str,
    ) {
        linux_notification_from_image_buffer_async(
            &self.title,
            message,
            self.image_buffer.as_ref(),
            self.timeout as u32,
        )
    }

    pub fn send_notification_and_wait(
        &self,
        message: &str,
        actions: Option<&[(&str, &str)]>,
        invocation_closure: impl FnOnce(&str),
    ) {
        linux_notification_from_image_buffer_and_wait(
            &self.title,
            message,
            self.image_buffer.as_ref(),
            self.timeout as u32,
            actions,
            invocation_closure,
        )
    }
}
