// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-parse1"

use parse::parse2::Parse2;

use core::context::CoreContext;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cx = Context::new();
    // let _ = parse_note(&cx, "b1,-3", 0, Duration::D8).unwrap();
    // dbg!(&cx);

    let cx = CoreContext::new();
    // let _ = parse_sysitems(cx, "|clef G | D4. -2,-3 D8 -4 % D16 2 3 4 5 D8 3 4 / D2. 0  |bl | 0 / 1").unwrap();
    // let _ = parse_sysitems(cx, "0 1 2").unwrap();
    // let _ = parse_sysitemlist(cx, "0_,1_ 1_ | 1 % bp");
    let _ = Parse2::sysitemlist2(cx, "bp % 0 1 ", false);
    dbg!(&cx.map_noteid_configuration);

    // dbg!(cx.map_noteid_tiesto.borrow());
    // dbg!(cx.map_noteid_tiesfrom.borrow());
    // let sysitems = parse_sysitems(cx, "|clef G F | d8 0 1 2  |bl | 3 4 5 / 2").unwrap();
    // dbg!(cx.sysitems.borrow());
    // dbg!(&cx.heads.borrow());
    // dbg!(&cx.map_head_position.borrow());
    // dbg!(&cx.stemitems);
    Ok(())
}
