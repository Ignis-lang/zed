use zed_extension_api as zed;

struct IgnisExtension;

impl zed::Extension for IgnisExtension {
    fn new() -> Self {
        IgnisExtension
    }
}

zed::register_extension!(IgnisExtension);
