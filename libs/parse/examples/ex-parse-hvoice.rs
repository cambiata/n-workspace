// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-parse-hvoice"

use core::context::CoreContext;
use parse::parse2::Parse2;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, "D8 0 -1", false);
    // dbg!(&cx.columns.borrow());
    // dbg!(&cx.hparts.borrow());
    dbg!(&cx.stemitems.borrow());
    dbg!(&cx.rows.borrow());
    Ok(())
}
