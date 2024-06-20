/// gettext package
pub(crate) const GETTEXT_PACKAGE: &str = "udisks2";

/// Translate msgid to localized message from the specified domain (with context support).
///
/// For more information, see [`dpgettext2`](https://docs.gtk.org/glib/func.dpgettext2.html)
pub(crate) fn dpgettext<T, U>(msgctxt: T, msgid: U) -> String
where
    T: Into<String>,
    U: Into<String>,
{
    const MSG_SEPARATOR: char = '\u{004}';
    gettextrs::dgettext(
        GETTEXT_PACKAGE,
        format!("{}{MSG_SEPARATOR}{}", msgctxt.into(), msgid.into()),
    )
}

/// Similar to [`gettextrs::pgettext`], but with support for formatted strings.
///
/// Unlike the provided macro, this function is compatible with gettext string extraction tools.
///
/// # Example
///
/// ```rust
/// let formatted_string = pgettext_f("hello-world", "Hello, {}!", ["world"]);
/// assert_eq!(formatted_string, "Hello, world!");
/// ```
//TODO: add function name to gettext keywords for extraction
pub(crate) fn pgettext_f(
    msgctxt: &str,
    format: &str,
    args: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    // map Rust style string formatting to C style formatting
    let s = gettextrs::pgettext(msgctxt, format.replace("{}", "%s"));
    arg_replace(s, args)
}

/// Similar to [`gettextrs::gettext`], but with support for formatted strings.
///
/// Unlike the provided macro, this function is compatible with gettext string extraction tools.
///
/// # Example
///
/// ```rust
/// let formatted_string = gettext_f("Hello, {}!", ["world"]);
/// assert_eq!(formatted_string, "Hello, world!");
/// ```
//TODO: add function name to gettext keywords for extraction
pub(crate) fn gettext_f(format: &str, args: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    // map Rust style string formatting to C style formatting
    let s = gettextrs::gettext(format.replace("{}", "%s"));
    arg_replace(s, args)
}

fn arg_replace(mut s: String, args: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    for arg in args {
        s = s.replacen("%s", arg.as_ref(), 1);
    }
    s
}
