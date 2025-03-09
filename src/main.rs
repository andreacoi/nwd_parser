use nwd_parser::domparser::parse_html;
use nwd_parser::nwd_pdf::gen_nwdpdf;
use nwd_parser::help::help;
use std::env;
use std::path::PathBuf;
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
     if args.len() != 3 {
        help();
    } else {
        let path: PathBuf = env::current_dir()?;
        let complete_path: String = format!("{}/{}", path.display(), &args[2]);
        match &args[1].as_str() {
            &"-f" => {
                let nwd_data = parse_html(complete_path);
                let filename: String = args[2].clone().replace(".html", "");
                gen_nwdpdf(filename, nwd_data);
            },
            _ => help(),
        }
    }

    Ok(())
}
