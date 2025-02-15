use scraper::{Html, Selector};
use std::fs;

// declare a struct called Viewpoint, containing all the fields that must be mapped from HTML.
#[derive(Debug)]
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
pub fn read_html(file: String) -> Vec<Viewpoint> {
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
    // TODO: to be used after
    //let comment_selector: Selector = Selector::parse("div.comment").unwrap();
    // status selector - select the status of each viewpoint
    // TODO: to be used after
    let status_selector: Selector = Selector::parse("span.namevaluepair > span.value").unwrap();
    // coords CANNOT BE selected by selector because the coords aren't within an HTML TAG.

    // starting the parse
    let viewpoints: Vec<Viewpoint> = document.select(&viewpoint_selector).map(|viewpoint| {
        let title: String = viewpoint.select(&title_selector)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_default();
        let imgurl: String = viewpoint.select(&img_selector).next()
                    .and_then(|el| el.value().attr("src"))
                    .unwrap_or("")
                    .to_string();
        let coords: String = viewpoint.text()
                     .filter(|t| t.trim().chars().next().map(|c| c.is_numeric()).unwrap_or(false))
                     .collect::<Vec<_>>()
                     .join(", ")
                     .trim()
                     .to_string();
        let status:String = viewpoint.select(&status_selector).next().map(|txt| txt.text().collect::<String>()).unwrap_or_default();
        let comment: String = String::from("Prova");

        Viewpoint {
            title,
            imgurl,
            coords,
            comment,
            status
        }
    }).collect();
    viewpoints
}
