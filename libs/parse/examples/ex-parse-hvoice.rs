// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-parse-hvoice"

use core::context::CoreContext;
use parse::parse2::Parse2;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, "0,1 0,-1,-2", false);
    // dbg!(&cx.columns.borrow());
    // dbg!(&cx.hparts.borrow());
    // dbg!(&cx.rows.borrow());
    // dbg!(&cx.heads.borrow());
    dbg!(&cx.map_head_position.borrow());
    Ok(())
}
