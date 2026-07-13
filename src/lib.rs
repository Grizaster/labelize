pub mod assets;
pub mod barcodes;
pub mod drawers;
pub mod elements;
pub mod encodings;
pub mod error;
pub mod hex;
pub mod images;
pub mod parsers;
#[cfg(feature = "serve")]
pub mod playground;

#[cfg(feature = "skill")]
pub mod skill;

pub use drawers::renderer::Renderer;
pub use elements::drawer_options::DrawerOptions;
pub use elements::label_info::LabelInfo;
pub use error::LabelizeError;
pub use images::monochrome::encode_png;
pub use images::pdf::encode_pdf;
pub use parsers::epl_parser::EplParser;
pub use parsers::zpl_parser::ZplParser;

/// Scale all coordinate and size fields in a [`LabelInfo`] by `scale`.
///
/// This is used for supersampled preview rendering: the label is rendered at
/// a higher resolution by scaling every ZPL dot coordinate and element size,
/// then the resulting image is downsampled to the target resolution with a
/// smooth filter for anti-aliased edges.
pub fn scale_label(label: &LabelInfo, scale: f64) -> LabelInfo {
    use elements::label_element::LabelElement;

    let mut scaled = LabelInfo {
        print_width: ((label.print_width as f64) * scale).round() as i32,
        inverted: label.inverted,
        elements: Vec::with_capacity(label.elements.len()),
    };

    for element in &label.elements {
        let scaled_element = match element {
            LabelElement::Text(t) => LabelElement::Text(scale_text_field(t, scale)),
            LabelElement::GraphicBox(gb) => LabelElement::GraphicBox(scale_graphic_box(gb, scale)),
            LabelElement::GraphicCircle(gc) => {
                LabelElement::GraphicCircle(scale_graphic_circle(gc, scale))
            }
            LabelElement::DiagonalLine(dl) => {
                LabelElement::DiagonalLine(scale_diagonal_line(dl, scale))
            }
            LabelElement::GraphicField(gf) => {
                LabelElement::GraphicField(scale_graphic_field(gf, scale))
            }
            LabelElement::Barcode128(bc) => LabelElement::Barcode128(scale_barcode128(bc, scale)),
            LabelElement::BarcodeEan13(bc) => {
                LabelElement::BarcodeEan13(scale_barcode_ean13(bc, scale))
            }
            LabelElement::Barcode2of5(bc) => {
                LabelElement::Barcode2of5(scale_barcode_2of5(bc, scale))
            }
            LabelElement::Barcode39(bc) => LabelElement::Barcode39(scale_barcode_39(bc, scale)),
            LabelElement::BarcodePdf417(bc) => {
                LabelElement::BarcodePdf417(scale_barcode_pdf417(bc, scale))
            }
            LabelElement::BarcodeAztec(bc) => {
                LabelElement::BarcodeAztec(scale_barcode_aztec(bc, scale))
            }
            LabelElement::BarcodeDatamatrix(bc) => {
                LabelElement::BarcodeDatamatrix(scale_barcode_datamatrix(bc, scale))
            }
            LabelElement::BarcodeQr(bc) => LabelElement::BarcodeQr(scale_barcode_qr(bc, scale)),
            LabelElement::Maxicode(mc) => {
                LabelElement::Maxicode(scale_maxicode(mc, scale))
            }
            // Config/template elements — pass through unchanged
            _ => element.clone(),
        };
        scaled.elements.push(scaled_element);
    }

    scaled
}

fn scale_pos(pos: &elements::label_position::LabelPosition, scale: f64) -> elements::label_position::LabelPosition {
    elements::label_position::LabelPosition {
        x: ((pos.x as f64) * scale).round() as i32,
        y: ((pos.y as f64) * scale).round() as i32,
        calculate_from_bottom: pos.calculate_from_bottom,
        automatic_position: pos.automatic_position,
    }
}

fn scale_text_field(t: &elements::text_field::TextField, scale: f64) -> elements::text_field::TextField {
    let mut scaled = t.clone();
    scaled.position = scale_pos(&t.position, scale);
    scaled.font.width = (t.font.width * scale).round();
    scaled.font.height = (t.font.height * scale).round();
    if let Some(block) = &t.block {
        scaled.block = Some(elements::field_block::FieldBlock {
            max_width: ((block.max_width as f64) * scale).round() as i32,
            max_lines: block.max_lines,
            line_spacing: ((block.line_spacing as f64) * scale).round() as i32,
            alignment: block.alignment,
            hanging_indent: ((block.hanging_indent as f64) * scale).round() as i32,
        });
    }
    scaled
}

fn scale_graphic_box(gb: &elements::graphic_box::GraphicBox, scale: f64) -> elements::graphic_box::GraphicBox {
    elements::graphic_box::GraphicBox {
        reverse_print: gb.reverse_print.clone(),
        position: scale_pos(&gb.position, scale),
        width: ((gb.width as f64) * scale).round() as i32,
        height: ((gb.height as f64) * scale).round() as i32,
        border_thickness: ((gb.border_thickness as f64) * scale).round() as i32,
        corner_rounding: ((gb.corner_rounding as f64) * scale).round() as i32,
        line_color: gb.line_color,
    }
}

fn scale_graphic_circle(gc: &elements::graphic_circle::GraphicCircle, scale: f64) -> elements::graphic_circle::GraphicCircle {
    elements::graphic_circle::GraphicCircle {
        reverse_print: gc.reverse_print.clone(),
        position: scale_pos(&gc.position, scale),
        circle_diameter: ((gc.circle_diameter as f64) * scale).round() as i32,
        border_thickness: ((gc.border_thickness as f64) * scale).round() as i32,
        line_color: gc.line_color,
    }
}

fn scale_diagonal_line(dl: &elements::graphic_diagonal_line::GraphicDiagonalLine, scale: f64) -> elements::graphic_diagonal_line::GraphicDiagonalLine {
    elements::graphic_diagonal_line::GraphicDiagonalLine {
        reverse_print: dl.reverse_print.clone(),
        position: scale_pos(&dl.position, scale),
        width: ((dl.width as f64) * scale).round() as i32,
        height: ((dl.height as f64) * scale).round() as i32,
        border_thickness: ((dl.border_thickness as f64) * scale).round() as i32,
        line_color: dl.line_color,
        top_to_bottom: dl.top_to_bottom,
    }
}

fn scale_graphic_field(gf: &elements::graphic_field::GraphicField, scale: f64) -> elements::graphic_field::GraphicField {
    elements::graphic_field::GraphicField {
        reverse_print: gf.reverse_print.clone(),
        position: scale_pos(&gf.position, scale),
        format: gf.format,
        data_bytes: gf.data_bytes,
        total_bytes: gf.total_bytes,
        row_bytes: gf.row_bytes,
        data: gf.data.clone(),
        magnification_x: ((gf.magnification_x as f64) * scale).round() as i32,
        magnification_y: ((gf.magnification_y as f64) * scale).round() as i32,
    }
}

fn scale_barcode128(bc: &elements::barcode_128::Barcode128WithData, scale: f64) -> elements::barcode_128::Barcode128WithData {
    elements::barcode_128::Barcode128WithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_128::Barcode128 {
            height: ((bc.barcode.height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        width: ((bc.width as f64) * scale).round() as i32,
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_ean13(bc: &elements::barcode_ean13::BarcodeEan13WithData, scale: f64) -> elements::barcode_ean13::BarcodeEan13WithData {
    elements::barcode_ean13::BarcodeEan13WithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_ean13::BarcodeEan13 {
            height: ((bc.barcode.height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        width: ((bc.width as f64) * scale).round() as i32,
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_2of5(bc: &elements::barcode_2of5::Barcode2of5WithData, scale: f64) -> elements::barcode_2of5::Barcode2of5WithData {
    elements::barcode_2of5::Barcode2of5WithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_2of5::Barcode2of5 {
            height: ((bc.barcode.height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        width: ((bc.width as f64) * scale).round() as i32,
        width_ratio: bc.width_ratio,
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_39(bc: &elements::barcode_39::Barcode39WithData, scale: f64) -> elements::barcode_39::Barcode39WithData {
    elements::barcode_39::Barcode39WithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_39::Barcode39 {
            height: ((bc.barcode.height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        width: ((bc.width as f64) * scale).round() as i32,
        width_ratio: bc.width_ratio,
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_pdf417(bc: &elements::barcode_pdf417::BarcodePdf417WithData, scale: f64) -> elements::barcode_pdf417::BarcodePdf417WithData {
    elements::barcode_pdf417::BarcodePdf417WithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_pdf417::BarcodePdf417 {
            row_height: ((bc.barcode.row_height as f64) * scale).round() as i32,
            module_width: ((bc.barcode.module_width as f64) * scale).round() as i32,
            by_height: ((bc.barcode.by_height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_aztec(bc: &elements::barcode_aztec::BarcodeAztecWithData, scale: f64) -> elements::barcode_aztec::BarcodeAztecWithData {
    elements::barcode_aztec::BarcodeAztecWithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_aztec::BarcodeAztec {
            magnification: ((bc.barcode.magnification as f64) * scale).round() as i32,
            ..bc.barcode
        },
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_datamatrix(bc: &elements::barcode_datamatrix::BarcodeDatamatrixWithData, scale: f64) -> elements::barcode_datamatrix::BarcodeDatamatrixWithData {
    elements::barcode_datamatrix::BarcodeDatamatrixWithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_datamatrix::BarcodeDatamatrix {
            height: ((bc.barcode.height as f64) * scale).round() as i32,
            ..bc.barcode
        },
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_barcode_qr(bc: &elements::barcode_qr::BarcodeQrWithData, scale: f64) -> elements::barcode_qr::BarcodeQrWithData {
    elements::barcode_qr::BarcodeQrWithData {
        reverse_print: bc.reverse_print.clone(),
        barcode: elements::barcode_qr::BarcodeQr {
            magnification: ((bc.barcode.magnification as f64) * scale).round() as i32,
        },
        height: ((bc.height as f64) * scale).round() as i32,
        position: scale_pos(&bc.position, scale),
        data: bc.data.clone(),
    }
}

fn scale_maxicode(mc: &elements::maxicode::MaxicodeWithData, scale: f64) -> elements::maxicode::MaxicodeWithData {
    elements::maxicode::MaxicodeWithData {
        reverse_print: mc.reverse_print.clone(),
        code: mc.code.clone(),
        position: scale_pos(&mc.position, scale),
        data: mc.data.clone(),
    }
}
