use scraper::{Html, Selector};
use std::fs;

// declare a struct called Viewpoint, containing all the fields that must be mapped from HTML.
pub struct Viewpoint {
    pub title: String,
    pub imgurl: String,
    pub coords: String,
    pub comment: String,
    pub status: String,
}

// function to read HTML and transform in JSON
// @args: file: String filename that must be in CWD.
// @return Result<String>: generic that manage JSON
pub fn read_html(file: String) -> () {
    let html_content: String = fs::read_to_string(&file)
        .expect("Invalid path or file not exists. Check your path and retry.\n");
    // Initialize all the selector we'll need on each iteration in the document file
    // document - parse all the html by the user selected file
    let document = Html::parse_document(&html_content);
    // viewpoint selector - select all the viewpoints - Each viewpoint will be a page in PDF
    // generation.
    let viewpoint_selector: Selector = Selector::parse("div.viewpoint").unwrap();
    // title selector - select the title of each viewpoint.
    let title_selector: Selector = Selector::parse("h2").unwrap();
    // img selector - select the image of each viewpoint
    let img_selector: Selector = Selector::parse("img").unwrap();
    // comment selector - select all the comments of each viewpoint
    let comment_selector: Selector = Selector::parse("div.comment").unwrap();
    // status selector - select the status of each viewpoint
    let status: Selector = Selector::parse("span.namevaluepair").unwrap();
    // coords CANNOT BE selected by selector because the coords aren't within an HTML TAG.

    // starting the parse
}
