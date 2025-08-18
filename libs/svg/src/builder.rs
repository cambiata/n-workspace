use std::collections::HashSet;

use graphics::fill::Fill;
use graphics::graphicitem::{items_bounding_box, items_move, GraphicItem, GraphicItems, PathCache};

use graphics::path::PathUtils;
use graphics::stroke::Stroke;

#[derive(Debug)]
pub struct SvgBuilder {
    path_cache: HashSet<String>,
}
const SVG_BORDER_MARGIN: f32 = 3.0;

impl SvgBuilder {
    pub fn new() -> SvgBuilder {
        SvgBuilder { path_cache: HashSet::new() }
    }

    pub fn build(&mut self, mut items: GraphicItems, code: Option<String>) -> String {
        let bbox = items_bounding_box(&items);
        items = items_move(items, SVG_BORDER_MARGIN + -bbox.0, SVG_BORDER_MARGIN + -bbox.1);

        let svg_width_value = bbox.2 + (-bbox.0) + (2.0 * SVG_BORDER_MARGIN);
        let svg_height_value = bbox.3 + (-bbox.1) + (2.0 * SVG_BORDER_MARGIN);
        let mut svg = xmlwriter::XmlWriter::new(xmlwriter::Options::default());
        svg.start_element("svg");
        svg.write_attribute("xmlns", "http://www.w3.org/2000/svg");
        svg.write_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink");
        svg.write_attribute("width", svg_width_value.to_string().as_str());
        svg.write_attribute("height", svg_height_value.to_string().as_str());
        svg.write_attribute_fmt("viewBox", format_args!("{} {} {} {}", 0, 0, svg_width_value, svg_height_value));
        if let Some(code) = code {
            svg.write_attribute("code", code.as_str());
        }
        // white background
        svg.start_element("rect");
        svg.write_attribute("x", "0");
        svg.write_attribute("y", "0");
        svg.write_attribute("width", svg_width_value.to_string().as_str());
        svg.write_attribute("height", svg_height_value.to_string().as_str());
        svg.write_attribute("fill", "white");
        svg.end_element();

        for item in items.iter() {
            match item {
                GraphicItem::Line(x1, y1, x2, y2, stroke, _id) => {
                    //
                    svg.start_element("line");
                    svg.write_attribute("x1", x1);
                    svg.write_attribute("y1", y1);
                    svg.write_attribute("x2", x2);
                    svg.write_attribute("y2", y2);
                    if let Stroke::Solid(w, color) = stroke {
                        svg.write_attribute("stroke", color);
                        svg.write_attribute("stroke-width", w);
                    }
                    svg.end_element();
                }

                GraphicItem::Rect(x, y, w, h, stroke, fill, _id) => {
                    svg.start_element("rect");
                    svg.write_attribute("x", x);
                    svg.write_attribute("y", y);
                    svg.write_attribute("width", w);
                    svg.write_attribute("height", h);
                    if let Stroke::Solid(w, color) = stroke {
                        svg.write_attribute("stroke", color);
                        svg.write_attribute("stroke-width", w);
                    }
                    if let Fill::Solid(color) = fill {
                        svg.write_attribute("fill", color);
                    } else {
                        svg.write_attribute("fill", "none");
                    }
                    svg.end_element();
                }

                GraphicItem::Path(segments, x, y, stroke, fill, cache) => {
                    match &cache {
                        Some(PathCache::UseCache) => {
                            let s = format!("{:?}", segments);
                            let md5: String = format!("{:?}", md5::compute(s));
                            if self.path_cache.contains(&md5) {
                                // just add a use element
                                svg.start_element("use");
                                svg.write_attribute("href", format!("#{}", md5.as_str()).as_str());
                                svg.write_attribute("x", format!("{}", x).as_str());
                                svg.write_attribute("y", format!("{}", y).as_str());

                                if let Stroke::Solid(w, color) = stroke {
                                    svg.write_attribute("stroke", color);
                                    svg.write_attribute("stroke-width", w);
                                }
                                if let Fill::Solid(color) = fill {
                                    svg.write_attribute("fill", color);
                                } else {
                                    svg.write_attribute("fill", "none");
                                }

                                svg.end_element();
                            } else {
                                // store the hashed path in a g wrapper element
                                self.path_cache.insert(md5.clone());

                                svg.start_element("g");
                                svg.write_attribute("transform", format!("translate({}, {})", x, y).as_str());
                                svg.write_attribute("visibility", "hidden".to_string().as_str());

                                svg.start_element("path");
                                svg.write_attribute("id", md5.as_str());
                                svg.write_attribute("d", PathUtils::path_to_string(segments.to_vec()).as_str());

                                svg.end_element();
                                svg.end_element();

                                // Store the use data
                                svg.start_element("use");
                                svg.write_attribute("href", format!("#{}", md5.as_str()).as_str());
                                svg.write_attribute("x", format!("{}", x).as_str());
                                svg.write_attribute("y", format!("{}", y).as_str());
                                svg.end_element();
                            }
                        }

                        None => {
                            svg.start_element("path");
                            svg.write_attribute("d", PathUtils::path_to_string_move(segments.to_vec(), *x, *y).as_str());
                            if let Stroke::Solid(w, color) = stroke {
                                svg.write_attribute("stroke", color);
                                svg.write_attribute("stroke-width", w);
                            }
                            if let Fill::Solid(color) = fill {
                                svg.write_attribute("fill", color);
                            } else {
                                svg.write_attribute("fill", "none");
                            }
                            svg.end_element();
                        }
                    }
                }

                GraphicItem::Text(x, y, text, _xtra) => {
                    svg.start_element("text");
                    svg.write_attribute("x", x);
                    svg.write_attribute("y", y);
                    svg.write_text(text);
                    svg.end_element();
                }

                _ => todo!("Unimplemented item type"),
            }
        }

        let str = svg.end_document();
        str
    }
}
