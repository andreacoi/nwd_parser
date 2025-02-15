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
pub fn parse_html(file: String) -> Vec<Viewpoint> {
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
    let status_selector: Selector = Selector::parse("span.namevaluepair > span.value").unwrap();
    // coords CANNOT BE selected by selector because the coords aren't within an HTML TAG.
    
    // Selectors used in the comment section - used for filter the REAL comment in the comment section.
    // initialize span selector
    let span_selector: Selector = Selector::parse("span").unwrap();
    // initialize h4 selector
    let h4_selector: Selector = Selector::parse("h4").unwrap();

    // starting the parse
    let viewpoints: Vec<Viewpoint> = document.select(&viewpoint_selector).map(|viewpoint| {
        // get the title of each page
        let title: String = viewpoint.select(&title_selector)
                    .next()
                    .map(|el| el.text().collect::<String>())
                    .unwrap_or_default();
        // get the imgurl - TODO: verify how to render images in its maximum width and height. Standard: A4 297x210mm???
        let imgurl: String = viewpoint.select(&img_selector).next()
                    .and_then(|el| el.value().attr("src"))
                    .unwrap_or("")
                    .to_string();
        // get the coords
        let coords: String = viewpoint.text()
                     .filter(|t| t.trim().chars().next().map(|c| c.is_numeric()).unwrap_or(false))
                     .collect::<Vec<_>>()
                     .join(", ")
                     .trim()
                     .to_string()
                     .replace("\u{a0}", " ")
                     .replace("\n", "")
                     .replace("\t", "");
        // get a clean comment filtering status span and other DOM stuff
        let clean_comment: String = viewpoint.select(&comment_selector)
                        .next()
                        .map(|comment| {
                            // get all the text in the div.comment tag
                            let mut full_text: String = comment.text().collect::<String>();
                            // get all span text in the div.comment tag, remove span text with a for cycle, w/o trimming or other further passages.
                            for span_text in comment.select(&span_selector).map(|s| s.text().collect::<String>()) {
                                full_text = full_text.replace(&span_text, "");
                            }
                            // get all h4 text in the div.comment tag, remove span text with a for cycle, w/o trimming or other further passages.
                            for h4_text in comment.select(&h4_selector).map(|h| h.text().collect::<String>()) {
                                full_text = full_text.replace(&h4_text, "");
                            }
                            full_text.trim().to_string()
                        }).filter(|text| !text.is_empty()).unwrap_or_default();
        // get the status of each issue
        let status:String = viewpoint.select(&status_selector)
                    .next()
                    .map(|txt| txt.text().collect::<String>())
                    .unwrap_or_default();

        Viewpoint {
            title,
            imgurl,
            coords,
            comment:clean_comment,
            status
        }
    }).collect();
    viewpoints
}
