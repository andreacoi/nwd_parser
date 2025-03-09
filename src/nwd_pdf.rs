use image::io::Reader as ImageReader;
use image::GenericImageView;
use printpdf::{ColorBits, Image, ImageTransform, ImageXObject, Mm, PdfDocument, Px};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use crate::domparser::Viewpoint;

// function to generate the pdf. Accepts as arguments the file title and the nwd_data
// nwd_data is a vector of Viewpoint, which is a struct that contains the data of the viewpoint

pub fn gen_nwdpdf(file_title: String, nwd_data: Vec<Viewpoint>) -> () {
    // create the empty document
    let doc = PdfDocument::empty(&file_title);
    // use include bytes to allocate a space in the heap in order to save the font on runtime, without recall it on the go.
    let font_regular = include_bytes!("../fonts/calibri-regular.ttf");
    // add the italic style to the font family
    let font_italic = include_bytes!("../fonts/calibri-italic.ttf");
    // add the bold style to the font family
    let font_bold = include_bytes!("../fonts/calibri-bold.ttf");
    // add the bold italic style to the font family
    let font_bold_italic = include_bytes!("../fonts/calibri-bold-italic.ttf");
    // use doc.add_external_font to allocate stream in the heap - regular font - TODO: embed all others styles, to complete the entire family
    let font_regular = doc
        .add_external_font(&font_regular[..])
        .expect("Failed to load Regular font");
    // use doc.add_external_font to allocate stream in the heap - italic font
    let _font_italic = doc
        .add_external_font(&font_italic[..])
        .expect("Failed to load Italic font");
    // use doc.add_external_font to allocate stream in the heap - bold font
    let font_bold = doc
        .add_external_font(&font_bold[..])
        .expect("Failed to load Bold font");
    // use doc.add_external_font to allocate stream in the heap - bold italic font
    let font_bold_italic = doc
        .add_external_font(&font_bold_italic[..])
        .expect("Failed to load Bold Italic font");
    // get the current working directory
    let current_working_dir =
        env::current_dir().expect("Unable to retrieve current working directory.");
    // iterate over the nwd_data vector
    for (index, viewpoint) in nwd_data.iter().enumerate() {
        let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0),&format!("layer {}", index + 1));
        let current_layer = doc.get_page(page).get_layer(layer);
        // insert here the title of the issue - the title is the name of the viewpoint
        current_layer.use_text(&viewpoint.title, 22.0, Mm(10.0), Mm(280.0), &font_bold);
        // get the image filename - to be joined with the current working directory
    let image_path = current_working_dir.join(&viewpoint.imgurl);
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
    // create the status layer - useful to set here the status of the issue - Open, Closed, In Progress, etc.
    let status_layer = doc.get_page(page).add_layer("status_layer");
    // set the status of the issue - get it from the viewpoint
    status_layer.use_text(format!("Status: {}", &viewpoint.status), 16.0, Mm(10.0), Mm(73.0), &font_bold);
    // create the coords layer - useful to set here the coordinates of the issue
    let coords_layer = doc.get_page(page).add_layer("coords_layer");
    // set the coordinates of the issue - get it from the viewpoint
    coords_layer.use_text(format!("Coords: {}", &viewpoint.coords), 12.0, Mm(10.0), Mm(65.0), &font_bold_italic);
    // create the comment layer - the comment is the description of the issue and explains why the issue is open, closed, etc.
    let comment_layer = doc.get_page(page).add_layer("comment_layer");
    // set the comment of the issue - get it from the viewpoint
    comment_layer.use_text(&viewpoint.comment, 14.0, Mm(10.0), Mm(53.0), &font_regular);
    }
    
    // create the real file in the runtime path - not using folders and complex paths
    let file = File::create(format!("{}.pdf", &file_title)).expect("failed to create file");
    // create bufwriter, doc.save accepts as argument only bufwriter<file>
    let mut writer = BufWriter::new(file);
    // save pdf in the created file
    doc.save(&mut writer).expect("failed to save pdf");
}
