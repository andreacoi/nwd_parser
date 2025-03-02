use image::io::Reader as ImageReader;
use image::GenericImageView;
use printpdf::{ColorBits, Image, ImageTransform, ImageXObject, Mm, PdfDocument, Px};
use std::env;
use std::fs::File;
use std::io::BufWriter;

pub fn gen_nwdpdf() {
    // use include bytes to allocate a space in the heap in order to save the font on runtime, without recall it on the go.
    let font_regular = include_bytes!("../fonts/calibri-regular.ttf");
    // create the document - blueprint
    let (doc, page1, layer1): (
        printpdf::PdfDocumentReference,
        printpdf::PdfPageIndex,
        printpdf::PdfLayerIndex,
    ) = PdfDocument::new("test - TO BE CHANGED", Mm(210.0), Mm(297.0), "layer 1");
    // use doc.add_external_font to allocate stream in the heap - regular font - TODO: embed all others styles, to complete the entire family
    let font_regular = doc
        .add_external_font(&font_regular[..])
        .expect("Failed to load Regular font");
    // initialize the current layer to have a place to write my content.
    let current_layer = doc.get_page(page1).get_layer(layer1);
    // insert a dummy text to test this function
    current_layer.use_text("This is Regular", 12.0, Mm(10.0), Mm(280.0), &font_regular);
    // create the cwd path in order to get the images from this directory
    let current_working_dir =
        env::current_dir().expect("Unable to retrieve current working directory.");
    // set the image path joining the cwd path with an hardcoded name of the file.
    // todo: set filename dinamically
    let image_path = current_working_dir.join("vp0001.jpg");
    // load the image using the crate image
    let img = ImageReader::open(image_path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");
    // get the dimensions of the image
    let (width, height) = img.dimensions();

    // Convert the image to an array of pixels compatible with printpdf
    let rgba_img = img.into_rgb8();

    // get the buffer of pixels
    let img_vec = rgba_img.into_vec();

    // Create the ImageXObject with the image data
    let image_xobj = ImageXObject {
        width: Px(width as usize),
        height: Px(height as usize),
        color_space: printpdf::ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8,
        image_data: img_vec,
        interpolate: false,
        clipping_bbox: None,
        image_filter: None,
    };

    // Create a printpdf Image from the ImageXObject
    let image = Image::from(image_xobj);

    // Add the image to the current layer with a transform that quadruples the size of the image
    image.add_to_layer(
        current_layer,
        ImageTransform {
            translate_x: Some(Mm(17.0)),
            translate_y: Some(Mm(90.0)),
            scale_x: Some(4.0),
            scale_y: Some(4.0),
            rotate: None,
            dpi: None,
        },
    );
    // create the real file in the runtime path - not using folders and complex paths
    let file = File::create("font_variants_example.pdf").expect("failed to create file");
    // create bufwriter, doc.save accepts as argument only bufwriter<file>
    let mut writer = BufWriter::new(file);
    // save pdf in the created file
    doc.save(&mut writer).expect("failed to save pdf");
    // todo: manage images
}
