use {
  boilerplate::Boilerplate,
  clap::Parser as _,
  html_escaper::{Escape, Trusted},
  pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag},
  std::{error::Error, fs, path::PathBuf},
};

#[derive(Boilerplate)]
struct IndexHtml {
  title: Option<String>,
  slides: Vec<String>,
}

#[derive(clap::Parser)]
struct Args {
  input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();

  let input = fs::read_to_string(args.input)?;

  let mut markdown = Vec::new();

  for event in Parser::new_ext(&input, Options::all()) {
    if let Event::Rule = event {
      markdown.push(Vec::new());
      continue;
    }
    if markdown.is_empty() {
      markdown.push(Vec::new());
    }
    markdown.last_mut().unwrap().push(event);
  }

  let title = {
    if let Some(events) = markdown.first() {
      let title_events = events
        .iter()
        .skip_while(|event| !matches!(event, Event::Start(Tag::Heading(HeadingLevel::H1, ..))))
        .skip(1)
        .take_while(|event| !matches!(event, Event::End(Tag::Heading(HeadingLevel::H1, ..))))
        .cloned()
        .collect::<Vec<Event>>();

      if title_events.is_empty() {
        None
      } else {
        let mut title = String::new();
        pulldown_cmark::html::push_html(&mut title, title_events.into_iter());
        Some(title)
      }
    } else {
      None
    }
  };

  let slides = markdown
    .into_iter()
    .map(|events| {
      let mut html = String::new();
      pulldown_cmark::html::push_html(&mut html, events.into_iter());
      html.trim().into()
    })
    .collect();

  let index = IndexHtml { title, slides };

  println!("{}", index);

  Ok(())
}
