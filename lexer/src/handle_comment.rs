use slk_tokenstream::TokenStream;

pub(crate) fn handle_comment(ts: &mut TokenStream<'_, char>) -> bool {
    let start = ts.mark();
    if !(ts.peek() == Some(&'/')) {
        return false;
    }

    ts.skip();
    match ts.peek() {
        Some('/') => {
            ts.skip_while(|c| *c != '\n');
            true
        }
        Some('*') => {
            loop {
                if let (Some('*'), Some('/')) = (ts.peek(), ts.peek_offset(1)) {
                    ts.advance(2);
                    break true;
                }
                ts.skip();
            }
        }
        None => {
            ts.reset(&start);
            true
        },
        Some(_) => {
            ts.reset(&start);
            false
        }
    }
}