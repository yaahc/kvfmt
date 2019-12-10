#[macro_export]
macro_rules! kvfmt_inner {
    // base case
    ([$fmt:expr] [$($args:tt)*]) => {
        format_args!($fmt, $( $args )*)
    };

    // Entry case(s)
    ([""] [$($args:tt)*] ?$name:expr $(, $($t:tt)*)?) => {
        $crate::kvfmt_inner!([concat!(stringify!($name), "={:?}")] [$( $args )* $name] $($($t)*)?);
    };
    ([""] [$($args:tt)*] $name:expr $(, $($t:tt)*)?) => {
        $crate::kvfmt_inner!([concat!(stringify!($name), "={}")] [$( $args )* $name ] $($($t)*)?);
    };

    // Munching case(s)
    ([$fmt:expr] [$($args:tt)*] ?$name:expr $(, $($t:tt)*)?) => {
        $crate::kvfmt_inner!([concat!($fmt, " ", stringify!($name), "={:?}")] [$( $args )*, $name] $($($t)*)?);
    };
    ([$fmt:expr] [$($args:tt)*] $name:expr $(, $($t:tt)*)?) => {
        $crate::kvfmt_inner!([concat!($fmt, " ", stringify!($name), "={}")] [$( $args )*, $name ] $($($t)*)?);
    };

}

#[macro_export]
macro_rules! kvfmt {
    ($($t:tt)*) => {
        std::fmt::format($crate::kvfmt_inner!([""] [] $($t)*));
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn kv() {
        let hi = "henlo!";
        assert!(!kvfmt!(?hi, ?hi).is_empty());
        assert!(!kvfmt!(hi, hi).is_empty());
        assert!(!kvfmt!(?hi, hi).is_empty());
        assert!(!kvfmt!(hi, hi, hi).is_empty());
        assert!(!kvfmt!(hi, ?hi, hi).is_empty());
        assert_eq!("hi=henlo! hi=\"henlo!\"", kvfmt!(hi, ?hi));
    }

    // #[test]
    // fn it_works() {
    //     let msg = "Failed to get root of the current git repository";
    //     let note = "gsync can only be run from within git repositories";
    //     let dbg = || format!("local={:?}", local);

    //     let err = UserErr::msg(msg).note(note).with_debug(dbg);

    //     let expected = format!("{}\nnote: {}\ndebug: {}", msg, note, dbg);

    //     assert_eq!(expected, err.to_string());
    // }
}
