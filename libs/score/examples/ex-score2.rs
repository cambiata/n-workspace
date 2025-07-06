// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-score2"

use core::context::CoreContext;
use parse::parse2::Parse2;
use score::{build::ScoreUtils2, scorecontext::ScoreContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    let _ = Parse2::sysitemlist2(cx, "clef G x | bl", false).unwrap();

    let scx = ScoreContext::new();
    // scx.build_sysitems(&cx.sysitems.borrow(), &cx.complexes.borrow())?;

    ScoreUtils2::build(&scx, &cx)?;
    dbg!(&scx.grid_columns.borrow());
    Ok(())
}
