// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-score1"

use core::context::CoreContext;
use parse::parse::parse_sysitems;
use score::scorecontext::ScoreContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    // let _ = parse_sysitems(cx, "|clef G | D4. -2,-3 D8 -4 % D16 2 3 4 5 D8 3 4 / D2. 0  |bl | 0 / 1").unwrap();
    // let _ = parse_sysitems(cx, "|bl |clef G F | d8 0 1 d4 2 2 / 0 d8 1 1 d4 0 |bl").unwrap();
    // let _ = parse_sysitems(cx, "|bl |clef G F | #3,-5n 1 1 % d2 b-3 d4 n3 |bl").unwrap();
    let _ = parse_sysitems(cx, "|bl |clef | 0 1 2 / d2 0 0 |bl").unwrap();

    let scx = ScoreContext::new();
    scx.build_sysitems(&cx.sysitems.borrow(), &cx.complexes.borrow())?;

    Ok(())
}
