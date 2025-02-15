use std::fs::File;
use std::io::BufWriter;
use printpdf::{PdfDocument, Mm};

pub fn gen_nwdpdf() {
    // use include bytes to allocate a space in the heap in order to save the font on runtime, without recall it on the go.
    let font_regular = include_bytes!("../fonts/calibri-regular.ttf");
    // create the document - blueprint
    let (doc, page1, layer1): (printpdf::PdfDocumentReference, printpdf::PdfPageIndex, printpdf::PdfLayerIndex) = PdfDocument::new("Font Variants Example", Mm(210.0), Mm(297.0), "Layer 1");
    // use doc.add_external_font to allocate stream in the heap - regular font - TODO: embed all others styles, to complete the entire family
    let font_regular = doc.add_external_font(&font_regular[..]).expect("Failed to load Regular font");
    // initialize the current layer to have a place to write my content.
    let current_layer = doc.get_page(page1).get_layer(layer1);
    // insert a dummy text to test this function
    current_layer.use_text("This is Regular", 12.0, Mm(10.0), Mm(280.0), &font_regular);
    // create the real file in the runtime path - not using folders and complex paths
    let file = File::create("font_variants_example.pdf").expect("Failed to create file");
    // create Bufwriter, doc.save accepts as argument only BufWriter<File>
    let mut writer = BufWriter::new(file);
    // Save PDF in the created file
    doc.save(&mut writer).expect("Failed to save PDF");
    // TODO: manage images
}