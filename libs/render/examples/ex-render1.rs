// cargo watch -q -c --ignore '**/*.svg' -x "run -q --example ex-render1"

use render::output::Generate;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = "clef G | 0,#1";

    let svg_string = Generate::svg_string(code)?;
    fs::write("libs/render/examples/ex-render1.svg", svg_string)?;
    Ok(())
}
