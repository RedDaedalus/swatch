use cfg_if::cfg_if;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

pub fn compose_svg(color: &String) -> String {
    return format!(r##"
        <svg width="512" height="256" viewbox="0 0 512 256" xmlns="http://www.w3.org/2000/svg">
            <defs>
                <linearGradient id="g" x2="100%" y2="100%">
                    <stop offset="0%" stop-color="black" stop-opacity="0" />
                    <stop offset="100%" stop-color="black" stop-opacity="0.4" />
                </linearGradient>
            </defs>

            <rect rx="4" ry="4" width="512" height="256" fill="{}" />
            <rect rx="4" ry="4" width="512" height="256" fill="url(#g)" />
        </svg>
    "##, color);
}