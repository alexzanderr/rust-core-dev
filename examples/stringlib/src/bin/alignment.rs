use pad::{
    Alignment,
    PadStr
};
use core_dev::stringlib::AlignString;

fn main() {
    let mut builder = AlignString::new("hello");
    println!("{}", builder);

    let mut builder = builder.width(10).alignment(Alignment::Middle);
    println!("{}", builder);

    let result = builder.build();
    println!("'{}'", result);

    println!("'{}'", builder.left());
    println!("'{}'", builder.right());
    println!("'{}'", builder);
}
