use core::{
    barline::BarlineType,
    clef::ClefSignature,
    complex::{Complex, ComplexInfo, ComplexType},
    duration::SumDuration,
    stems::stemitems::{StemHeadPosition, StemItem},
    sysitem::{SysItem, SysItemType},
    ItemId,
};
use std::{cell::RefCell, collections::BTreeMap};

use graphics::{color::Color, rectangle::Rectangle};
use grid::griditem::GridItemType;

use crate::{
    complex::{collect_accidentals, create_glyphsrectangles_accidentals, create_glyphsrectangles_note, sort_accidentals},
    constants::{BARLINE_DOUBLE_WIDTH, BARLINE_FINAL_WIDTH, BARLINE_WIDTH, CLEF_WIDTH, SPACE2, SPACE4},
    glyphitem::{ComplexGlyphsRectangles, GlyphItem, GlyphRectangle, PartGlyphsRectangles, SysitemGlyphsRectangles},
    headpositions::calculate_head_positions,
};

#[derive(Debug)]
pub struct ScoreContext {
    pub grid_columns: RefCell<Vec<Vec<GridItemType<GlyphItem>>>>,
    pub grid_column_sysitem_ids: RefCell<Vec<usize>>,
    pub map_head_position: RefCell<BTreeMap<usize, StemHeadPosition>>,
}

impl ScoreContext {
    pub fn new() -> &'static ScoreContext {
        let scx = ScoreContext {
            grid_columns: RefCell::new(Vec::new()),
            grid_column_sysitem_ids: RefCell::new(Vec::new()),
            map_head_position: RefCell::new(BTreeMap::new()),
        };
        Box::leak(Box::new(scx))
    }

    // Vec<Vec<GridItemType<GlyphItem>>>

    pub fn build_stemitems_headpositions(&self, stemitems: &[StemItem]) -> Result<(), Box<dyn std::error::Error>> {
        calculate_head_positions(stemitems, &mut self.map_head_position.borrow_mut());
        Ok(())
    }

    pub fn build_sysitems(&self, sysitems: &[SysItem], complexes: &[Complex]) -> Result<(), Box<dyn std::error::Error>> {
        //---------------------------------------------------------------
        let expected_parts_count = sysitems.iter().fold(0, |acc, sysitem| sysitem.parts_count.max(acc));

        for (sysidx, sysitem) in sysitems.iter().enumerate() {
            match &sysitem.stype {
                SysItemType::Parts(_part_ids, _sum_duration, _complexes_infos, _positions_durations) => {
                    self.build_sysitem_parts(complexes, sysitem.id, _part_ids, _sum_duration, _complexes_infos, _positions_durations, expected_parts_count)?;
                }
                SysItemType::Clefs(_clefs) => {
                    // println!("Clef found in sysitem {}", sysidx);
                    self.build_sysitem_clefs(sysitem.id, _clefs, expected_parts_count)?;
                }
                SysItemType::Barline(_barline) => {
                    // println!("Barline found in sysitem {}", sysidx);
                    self.build_sysitem_barline(sysitem.id, _barline, expected_parts_count)?;
                }
                SysItemType::Other => {
                    println!("Other item found in sysitem {}", sysidx);
                    todo!();
                }
            };
        }

        // Ok(self.grid_columns.borrow())
        Ok(())
    }

    fn build_sysitem_barline(&self, sysitem_id: usize, btype: &BarlineType, expected_parts_count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();

        let rect = match btype {
            BarlineType::Double => (0.0, -SPACE2, BARLINE_DOUBLE_WIDTH, SPACE4),
            BarlineType::Final => (0.0, -SPACE2, BARLINE_FINAL_WIDTH, SPACE4),
            _ => (0.0, -SPACE2, BARLINE_WIDTH, SPACE4),
        };
        let glyph: GlyphItem = GlyphItem::Barline(btype.clone());
        let item: GlyphRectangle = (rect, glyph.clone());
        column_griditems.push(GridItemType::Rectangles(vec![item.clone()]));

        // add missing barlines
        while column_griditems.len() < expected_parts_count {
            let glyph: GlyphItem = glyph.clone();
            let rect = (0.0, -SPACE2, 1.0, SPACE4); // Placeholder rectangle
            column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
        }

        self.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
        self.grid_columns.borrow_mut().push(column_griditems);

        Ok(())
    }

    fn build_sysitem_clefs(&self, sysitem_id: usize, _clefs: &[ClefSignature], expected_parts_count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();
        for clef in _clefs {
            // println!("Clef: {:?}", clef);
            let glyph: GlyphItem = GlyphItem::Clef(clef.clone());
            let rect = (0.0, -SPACE2, CLEF_WIDTH, SPACE4); // Placeholder rectangle
            column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
        }

        // add missing clefs
        while column_griditems.len() < expected_parts_count {
            let glyph = match column_griditems.len() {
                0 => GlyphItem::Clef(ClefSignature::Treble),
                1 => GlyphItem::Clef(ClefSignature::Bass),
                _ => GlyphItem::XRect(Color::Gray),
            };

            let rect = (0.0, -SPACE2, 1.0, SPACE4); // Placeholder rectangle

            column_griditems.push(GridItemType::Rectangles(vec![(rect, glyph.clone())]));
        }

        self.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
        self.grid_columns.borrow_mut().push(column_griditems);
        Ok(())
    }

    pub fn build_sysitem_parts(
        &self,
        complexes: &[Complex],
        sysitem_id: usize,
        _parts_ids: &Vec<ItemId>,
        _sum_duration: &SumDuration,
        complexes_infos: &Vec<BTreeMap<usize, ComplexInfo>>,
        positions_durations: &BTreeMap<usize, usize>,
        expected_parts_count: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //-----------------------------
        for (pos, _duration) in positions_durations.iter() {
            let mut column_griditems: Vec<GridItemType<GlyphItem>> = Vec::new();
            for (partidx, _part_complexes) in complexes_infos.iter().enumerate() {
                if let Some(complex_info) = _part_complexes.get(pos) {
                    let complex_id = complex_info.0;
                    let complex = &complexes[complex_id];
                    let complex_rectangles: ComplexGlyphsRectangles = self.create_glyphsrectangles_complex(partidx, complex);
                    column_griditems.push(GridItemType::Rectangles(complex_rectangles));
                } else {
                    column_griditems.push(GridItemType::Empty);
                }
            }
            while column_griditems.len() < expected_parts_count {
                column_griditems.push(GridItemType::Empty);
            }
            self.grid_column_sysitem_ids.borrow_mut().push(sysitem_id);
            self.grid_columns.borrow_mut().push(column_griditems);
        }
        Ok(())
    }

    pub fn create_glyphsrectangles_complex(&self, _partidx: usize, _complex: &Complex) -> ComplexGlyphsRectangles {
        let mut rectangles: ComplexGlyphsRectangles = Vec::new();

        match _complex.ctype {
            ComplexType::Upper(ref note) => {
                // note
                let mut note_rectangles: Vec<(Rectangle, GlyphItem)> = create_glyphsrectangles_note(note, &self.map_head_position.borrow());

                // accidentals
                let mut accidentals = collect_accidentals(note);
                sort_accidentals(&mut accidentals);
                create_glyphsrectangles_accidentals(&accidentals, &mut note_rectangles);

                rectangles.extend(note_rectangles);
                // rectangles.extend(acc_rectangles);
            }
            ComplexType::Lower(ref note) => {
                // note
                let mut note_rectangles = create_glyphsrectangles_note(note, &self.map_head_position.borrow());

                // accidentals
                let mut accidentals = collect_accidentals(note);
                sort_accidentals(&mut accidentals);
                create_glyphsrectangles_accidentals(&accidentals, &mut note_rectangles);

                rectangles.extend(note_rectangles);
                // rectangles.extend(acc_rectangles);
            }
            ComplexType::UpperAndLower(ref upper, ref lower, _diff) => {
                // note
                let mut note_rectangles = create_glyphsrectangles_note(upper, &self.map_head_position.borrow());
                note_rectangles.extend(create_glyphsrectangles_note(lower, &self.map_head_position.borrow()));

                // accidentals
                let mut accidentals = collect_accidentals(upper);
                accidentals.extend(collect_accidentals(lower));
                sort_accidentals(&mut accidentals);
                create_glyphsrectangles_accidentals(&accidentals, &mut note_rectangles);

                rectangles.extend(note_rectangles);
                // rectangles.extend(acc_rectangles);
            }
        }

        rectangles
    }
}

#[derive(Debug)]
pub enum SysItemGlyphsRectangles {
    OneColumn(PartGlyphsRectangles),
    ManyColumns(SysitemGlyphsRectangles),
}
