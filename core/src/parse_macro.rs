#[macro_export]
macro_rules! get_or_ret {
    ($ts: ident, $start: ident, $mat: pat, $error: expr) => {
        {

            let cur_pos = $ts.mark();
            match $ts.consume_if_else_err(|t| matches!(t.kind(), $mat)) {
                Ok(t) => {
                    t
                }
                Err(e) => {

                    let (kind, mut span) = ::slk_c_core::parser_core::parser_errors::ParserErrorKind::expected_got_from_opt($error, &e, cur_pos, cur_pos);
                    $ts.reset(&$start);

                    return Err(::slk_c_core::parser_core::parser_errors::ParserError::new(span, kind));
                }
            }
        }
    };
}

#[macro_export]
macro_rules! replace_desired {
    ($ts: ident, $start: ident, $err: expr, $desired: expr) => {
        $err.map_err(|mut e| {
            e.replace_desired($desired);
            $ts.reset(&$start);
            e
        })
    };
}
