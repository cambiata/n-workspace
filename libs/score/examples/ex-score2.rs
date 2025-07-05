// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-score2"

use core::context::CoreContext;
use parse::parse2::Parse2;
use score::{scorecontext::ScoreContext, scoreutils2::ScoreUtils2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, " 0 1 2  % D8 0 0 0", false).unwrap();

    let scx = ScoreContext::new();
    // scx.build_sysitems(&cx.sysitems.borrow(), &cx.complexes.borrow())?;

    ScoreUtils2::build(&scx, &cx);

    Ok(())
}
