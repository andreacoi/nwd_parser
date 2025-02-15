use std::fs::File;
use std::io::{BufWriter, Write};
use printpdf::{PdfDocument, Mm, BuiltinFont, IndirectFontRef};

pub fn gen_nwdpdf() {
    let font_regular = include_bytes!("../fonts/calibri-regular.ttf");
    let (mut doc, page1, layer1) = PdfDocument::new("Font Variants Example", Mm(210.0), Mm(297.0), "Layer 1");
    let font_regular = doc.add_external_font(&font_regular[..]).expect("Failed to load Regular font");
        // Aggiungi testo con font Regular
        let current_layer = doc.get_page(page1).get_layer(layer1);
        current_layer.use_text("This is Regular", 12.0, Mm(10.0), Mm(280.0), &font_regular);
    // Crea il file e un `BufWriter` per ottimizzare la scrittura
    let file = File::create("font_variants_example.pdf").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // Salva il PDF nel file
    doc.save(&mut writer).expect("Failed to save PDF");

}