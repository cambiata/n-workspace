// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-score1"

use core::context::CoreContext;

use parse::parse::parse_sysitems;
use utils::build_score;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cx = CoreContext::new();
    // let _ = parse_sysitems(cx, "|clef G | D4. -2,-3 D8 -4 % D16 2 3 4 5 D8 3 4 / D2. 0  |bl | 0 / 1").unwrap();
    let _ = parse_sysitems(cx, "|bl |0 / 1 % 2 3 ").unwrap();

    build_score(&cx)?;

    Ok(())
}
