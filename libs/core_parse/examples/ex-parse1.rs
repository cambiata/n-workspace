// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-parse1"

use core_parse::parse::parse_sysitems;

use core::context::CoreContext;

pub fn main() {
    // let cx = Context::new();
    // let _ = parse_note(&cx, "b1,-3", 0, Duration::D8).unwrap();
    // dbg!(&cx);

    let cx = CoreContext::new();
    // let _ = parse_sysitems(cx, "|clef G | D4. -2,-3 D8 -4 % D16 2 3 4 5 D8 3 4 / D2. 0  |bl | 0 / 1").unwrap();
    // let _ = parse_sysitems(cx, "0 1 2").unwrap();
    let _ = parse_sysitems(cx, "0,1 0,-1");
    dbg!(&cx.heads.borrow());
    dbg!(&cx.map_head_position.borrow());

    // dbg!(&cx.stemitems);
}
