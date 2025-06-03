pub mod complex;
pub mod constants;
pub mod glyphitem;
pub mod scorecontext;

// fn build_sysitem_barline(scx: &ScoreContext, sysitem_id: usize, btype: &BarlineType, expected_parts_count: usize) -> Result<(), Box<dyn std::error::Error>> {
//     let mut scx_grid_columns = scx.grid_columns.borrow_mut();
//     let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();

//     let rect = match btype {
//         BarlineType::Double => (0.0, -SPACE2, BARLINE_DOUBLE_WIDTH, SPACE4), // Placeholder rectangle for double barline
//         BarlineType::Final => (0.0, -SPACE2, BARLINE_FINAL_WIDTH, SPACE4),   // Placeholder rectangle for final barline
//         _ => (0.0, -SPACE2, BARLINE_WIDTH, SPACE4),                          // Placeholder rectangle for single barline
//     };
//     let glyph: GlyphItem = GlyphItem::Barline(btype.clone());
//     let item: GlyphRectangle = (rect, glyph.clone());
//     column_griditems.push(GridItemType::Rectangles(vec![item.clone()]));

//     // add missing barlines
//     while column_griditems.len() < expected_parts_count {
//         let glyph: GlyphItem = glyph.clone();
//         let rect = (0.0, -SPACE2, 1.0, SPACE4); // Placeholder rectangle
//         column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
//     }

//     scx.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
//     scx_grid_columns.push(column_griditems);

//     Ok(())
// }

// fn build_sysitem_clefs(scx: &ScoreContext, sysitem_id: usize, _clefs: &[ClefSignature], expected_parts_count: usize) -> Result<(), Box<dyn std::error::Error>> {
//     let mut scx_grid_columns = scx.grid_columns.borrow_mut();
//     let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();

//     for clef in _clefs {
//         println!("Clef: {:?}", clef);
//         let glyph: GlyphItem = GlyphItem::Clef(clef.clone());
//         let rect = (0.0, -SPACE2, 1.0, SPACE4); // Placeholder rectangle

//         column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
//     }

//     // add missing clefs
//     while column_griditems.len() < expected_parts_count {
//         let glyph = match column_griditems.len() {
//             0 => GlyphItem::Clef(ClefSignature::Treble),
//             1 => GlyphItem::Clef(ClefSignature::Bass),
//             _ => GlyphItem::XBlue,
//         };

//         let rect = (0.0, -SPACE2, 1.0, SPACE4); // Placeholder rectangle

//         column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
//     }

//     scx.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
//     scx_grid_columns.push(column_griditems);
//     Ok(())
// }

// pub fn build_sysitem_parts(
//     scx: &ScoreContext,
//     complexes: &[Complex],
//     sysitem_id: usize,
//     _parts_ids: &Vec<ItemId>,
//     _sum_duration: &SumDuration,
//     complexes_infos: &Vec<BTreeMap<usize, ComplexInfo>>,
//     positions_durations: &BTreeMap<usize, usize>,
//     expected_parts_count: usize,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     //-----------------------------
//     for (pos, dur) in positions_durations.iter() {
//         let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();
//         for (partidx, _part_complexes) in complexes_infos.iter().enumerate() {
//             println!("- - Position: {}, Duration: {}", pos, dur);
//             if let Some(complex_info) = _part_complexes.get(pos) {
//                 println!("- - - Part {}: Complex Info: {:?}", partidx, complex_info);
//                 let complex_id = complex_info.0;
//                 let complex = &complexes[complex_id];
//                 let complex_rectangles: ComplexGlyphsRectangles = create_glyphsrectangles_complex(partidx, complex);
//                 // dbg!(&complex_rectangles);
//                 column_griditems.push(GridItemType::Rectangles(complex_rectangles));
//             } else {
//                 println!("- - - Part {}: No complex info at position {}", partidx, pos);
//                 column_griditems.push(GridItemType::Empty);
//             }
//         }
//         while column_griditems.len() < expected_parts_count {
//             column_griditems.push(GridItemType::Empty);
//         }
//         scx.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
//         scx.grid_columns.borrow_mut().push(column_griditems);
//     }

//     Ok(())
// }
