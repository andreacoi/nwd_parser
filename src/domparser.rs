use html_parser::{Dom, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

// declare a struct called Viewport, containing all the fields that must be mapped from HTML.
pub struct Viewport {
    pub title: String,
    pub imgurl: String,
    pub coords: String,
    pub comment: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HtmlNode {
    name: String,
    variant: String,
    #[serde(default)]
    classes: Vec<String>,
    #[serde(default)]
    attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    children: Vec<Value>, // Può contenere sia stringhe che altri nodi
}

// function to read HTML and transform in JSON
// @args: file: String filename that must be in CWD.
// @return Result<String>: generic that manage JSON
pub fn read_html(file: String) -> Vec<HtmlNode> {
    let html_content: String = fs::read_to_string(&file)
        .expect("Invalid path or file not exists. Check your path and retry.\n");
    let json = Dom::parse(&html_content).unwrap().to_json_pretty().unwrap();
    let nodes = serde_json::from_str(json.as_str()).expect("JSON non valido");
    nodes
}

// function to extract_viewports from JSON
// @args: file: String filename that must be in CWD.
// @return Vec<Viewport>: a vector containing Viewport type elements.
pub fn extract_viewports(nodes: &[HtmlNode]) -> Vec<Viewport> {
    // initialize viewports as empty Vec.
    let mut viewports: Vec<Viewport> = Vec::new();
    for node in nodes {
        if node.name == "div" && node.classes.contains(&"viewpoint".to_string()) {
            let mut title: String = String::new();
            let mut imgurl: String = String::new();
            let mut coords: String = String::new();
            let mut comment: String = String::new();
            let mut status: String = String::new();

            for child in &node.children {
                if let Some(obj) = child.as_object() {
                    if let Ok(child_node) = serde_json::from_value::<HtmlNode>(child.clone()) {
                        match child_node.name.as_str() {
                            "h2" => {
                                if let Some(first_child) = child_node.children.first() {
                                    if let Some(text) = first_child.as_str() {
                                        title = text.to_string();
                                    }
                                }
                            }
                            "a" => {
                                if let Some(attrs) = &child_node.attributes {
                                    if let Some(href) = attrs.get("href") {
                                        imgurl = href.clone();
                                    }
                                }
                            }
                            "div" => {
                                if child_node.classes.contains(&"comments".to_string()) {
                                    for comment_child in child_node.children {
                                        if let Ok(comment_node) = serde_json::from_value::<HtmlNode>(
                                            comment_child.clone(),
                                        ) {
                                            if comment_node.classes.contains(&"comment".to_string())
                                            {
                                                for sub_child in &comment_node.children {
                                                    if let Some(text) = sub_child.as_str() {
                                                        comment = text.to_string();
                                                    } else if let Ok(sub_node) =
                                                        serde_json::from_value::<HtmlNode>(
                                                            sub_child.clone(),
                                                        )
                                                    {
                                                        if sub_node
                                                            .classes
                                                            .contains(&"value".to_string())
                                                        {
                                                            if let Some(value) = sub_node
                                                                .children
                                                                .first()
                                                                .and_then(|v| v.as_str())
                                                            {
                                                                status = value.to_string();
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                } else if let Some(text) = child.as_str() {
                    // if the child is a text with no tags let's suppose could be coordinates
                    coords = text
                        .trim()
                        .replace("\r\n", "")
                        .replace("\t", "")
                        .to_string();
                }
            }
            viewports.push(Viewport {
                title,
                imgurl,
                coords,
                comment,
                status,
            });
        }
    }

    viewports
}

// TODO: INSERT COMMENTS FOR EACH AREA
