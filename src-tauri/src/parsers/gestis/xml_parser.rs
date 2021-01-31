use std::collections::HashMap;

use lazy_static::lazy_static;
use roxmltree::{Document, Node, NodeId};

use super::error::{ParseError, Result};
use super::types::{Data, GestisResponse, Image};

// maybe needed for later
// pub const PARTS: [(&str, &str, &str); 8] = [
//   ("0400", "0400", "sf"),
//   ("0600", "0602", "smp"),
//   ("0600", "0603", "sdp"),
//   ("0500", "0501", "ld50"),
//   ("1100", "1303", "h_p_signal_symbols"),
//   ("1100", "1106", "wgk"),
//   ("1100", "1201", "mak1"),
//   ("1100", "1203", "mak2"),
// ];

lazy_static! {
  pub static ref CHAPTER_MAPPING: HashMap<&'static str, (&'static str, &'static str)> = [
    ("melting_point", ("0600", "0602")),
    ("boiling_point", ("0600", "0603")),
    ("molecular_formula", ("0400", "0400")),
    ("water_hazard_class", ("1100", "1106")),
    ("h_p_signal_symbols", ("1100", "1303")),
    ("lethal_dose", ("0500", "0501")),
  ]
  .iter()
  .cloned()
  .collect();
}

pub fn parse_response(json: GestisResponse) -> Result<Data> {
  println!("extracting data for: {} ...", json.name);

  let h_p_signal_symbols_error;
  let (h_phrases, p_phrases, signal_word, symbols) = match get_h_p_signal_symbols(&json) {
    Ok(inner) => {
      h_p_signal_symbols_error = None;
      inner
    }
    Err(e) => {
      h_p_signal_symbols_error = Some(e);
      (
        // this error type should not be used like this but here it is
        // only a sentinel value that is ignored further down anyways
        Err(ParseError::Empty),
        Err(ParseError::Empty),
        Err(ParseError::Empty),
        Err(ParseError::Empty),
      )
    }
  };

  Ok(Data {
    molecular_formula: match get_molecular_formula(&json) {
      Ok(inner) => inner,
      Err(e) => {
        // should never occur
        eprintln!("[molecular_formula] error: {:#?}", e);
        return Err(e);
      }
    },
    melting_point: match get_melting_point(&json) {
      Ok(inner) => Some(inner),
      Err(e) => {
        eprintln!("[melting_point] error: {:#?}", e);
        None
      }
    },
    boiling_point: match get_boiling_point(&json) {
      Ok(inner) => Some(inner),
      Err(e) => {
        eprintln!("[boiling_point] error: {:#?}", e);
        None
      }
    },
    water_hazard_class: match get_whc(&json) {
      Ok(inner) => Some(inner),
      Err(e) => {
        eprintln!("[water_hazard_class] error: {:#?}", e);
        None
      }
    },
    h_phrases: match h_phrases {
      Ok(inner) => Some(inner),
      Err(e) => {
        let e = match &h_p_signal_symbols_error {
          Some(e_new) => e_new,
          None => &e,
        };
        eprintln!("[h_phrases] error: {:#?}", e);
        None
      }
    },
    p_phrases: match p_phrases {
      Ok(inner) => Some(inner),
      Err(e) => {
        let e = match &h_p_signal_symbols_error {
          Some(e_new) => e_new,
          None => &e,
        };
        eprintln!("[p_phrases] error: {:#?}", e);
        None
      }
    },
    signal_word: match signal_word {
      Ok(inner) => Some(inner),
      Err(e) => {
        let e = match &h_p_signal_symbols_error {
          Some(e_new) => e_new,
          None => &e,
        };
        eprintln!("[signal_word] error: {:#?}", e);
        None
      }
    },
    symbols: match symbols {
      Ok(inner) => Some(inner),
      Err(e) => {
        let e = match &h_p_signal_symbols_error {
          Some(e_new) => e_new,
          None => &e,
        };
        eprintln!("[symbols] error: {:#?}", e);
        None
      }
    },
    lethal_dose: match get_lethal_dose(&json) {
      Ok(inner) => inner,
      Err(e) => {
        eprintln!("[lethal_dose] error: {:#?}", e);
        match e {
          ParseError::Multiple(inner) => Some(inner),
          _ => None,
        }
      }
    },
  })
}

/* #region  helpers */

pub fn get_xml(json: &GestisResponse, chapter: &str, subchapter: &str) -> Result<String> {
  if let Some(subchapters) = json.chapters.iter().find(|c| c.dr_number == chapter) {
    if let Some(sub) = subchapters
      .subchapters
      .iter()
      .find(|s| s.dr_number == subchapter)
    {
      return Ok(format!("<div>\n{}</div>\n", sub.text.as_ref().unwrap()));
    }
  }
  Err(ParseError::NoXML)
}

#[inline]
fn tables(node: &Node, class: &str) -> Vec<Vec<Vec<NodeId>>> {
  node
    .children()
    .filter(|n| {
      n.has_tag_name("table")
                // && n.has_attribute("class")
                && n.attribute_node("class").unwrap().value() == class
    })
    .map(|n| {
      n.children()
        .filter(|c| c.has_tag_name("tr"))
        .map(|n| {
          n.children()
            .filter(|n| n.has_tag_name("td"))
            .map(|n| n.id())
            .collect()
        })
        .collect()
    })
    .collect()
}

/* #endregion */

/* #region  extractors */

fn get_molecular_formula(json: &GestisResponse) -> Result<String> {
  let (chapter, subchapter) = CHAPTER_MAPPING.get("molecular_formula").unwrap();
  let xml = get_xml(json, chapter, subchapter)?;
  let doc = Document::parse(&xml)?;

  let mut tables = tables(&doc.root().first_child().unwrap(), "block")
    // tables
    .into_iter()
    // rows
    .flatten()
    // ids
    .flatten();

  let mut data;

  // kill empty tables at beginning
  loop {
    let data_id = tables.next().unwrap();
    data = doc.get_node(data_id).unwrap();
    if data.children().count() > 0 {
      break;
    }
  }

  if let Some(mf) = data.children().find(|c| c.has_tag_name("summenformel")) {
    return Ok(mf.first_child().unwrap().text().unwrap().into());
  }

  Err(ParseError::MissingInfo("molecular formula".into()))
}

fn get_melting_point(json: &GestisResponse) -> Result<String> {
  get_mp_bp(json, "melting_point", "Schmelzpunkt:")
}

fn get_boiling_point(json: &GestisResponse) -> Result<String> {
  get_mp_bp(json, "boiling_point", "Siedepunkt:")
}

/// returns melting point or boiling point
fn get_mp_bp(json: &GestisResponse, name: &str, xml_check: &str) -> Result<String> {
  let (chapter, subchapter) = CHAPTER_MAPPING.get(name).unwrap();
  let xml = get_xml(json, chapter, subchapter)?;
  let doc = Document::parse(&xml)?;

  let mut mp_bp_point = None;

  for data_id in tables(&doc.root().first_child().unwrap(), "block")
    // tables
    .into_iter()
    // rows
    .flatten()
    // ids
    .flatten()
  {
    let data = doc.get_node(data_id).unwrap();
    let tables = tables(&data, "feldmitlabel");
    if tables.is_empty() {
      continue;
    }
    let mut inner_data_ids = tables
      // tables
      .into_iter()
      // rows
      .flatten()
      // ids
      .flatten();

    let mp_bp_text_node = doc.get_node(inner_data_ids.next().unwrap()).unwrap();
    if let Some(mp_bp_text) = mp_bp_text_node.text() {
      if mp_bp_text == xml_check {
        let mp_bp_node = doc.get_node(inner_data_ids.next().unwrap()).unwrap();
        mp_bp_point = mp_bp_node.text();
      }
    }
  }

  match mp_bp_point {
    Some(mp) => Ok(mp.trim().into()),
    None => Err(ParseError::MissingInfo(name.into())),
  }
}

fn get_whc(json: &GestisResponse) -> Result<String> {
  static KEYWORD: &str = "Nicht wassergefährdender Stoff";

  let (chapter, subchapter) = CHAPTER_MAPPING.get("water_hazard_class").unwrap();
  let xml = get_xml(json, chapter, subchapter)?;
  let doc = Document::parse(&xml)?;

  let mut tables = tables(&doc.root().first_child().unwrap(), "block")
    // tables
    .into_iter()
    // rows
    .flatten()
    // ids
    .flatten();

  let mut data;

  // kill empty tables at beginning
  loop {
    let data_id = tables.next().unwrap();
    data = doc.get_node(data_id).unwrap();
    if data.children().count() > 0 {
      break;
    }
  }

  if let Some(node_id) = tables.next() {
    data = doc.get_node(node_id).unwrap();
    let text = data.text().unwrap();
    if text == KEYWORD {
      return Err(ParseError::Empty);
    } else {
      return Ok(text.split('-').next().unwrap().trim().into());
    }
  }

  Err(ParseError::MissingInfo("water hazard class".into()))
}

type HPSignalSymbolsResult = Result<(
  Result<Vec<(std::string::String, std::string::String)>>,
  Result<Vec<(std::string::String, std::string::String)>>,
  Result<std::string::String>,
  Result<Vec<Image>>,
)>;

fn get_h_p_signal_symbols(json: &GestisResponse) -> HPSignalSymbolsResult {
  #[inline]
  fn extract_h_p(id: NodeId, doc: &Document) -> Vec<(String, String)> {
    let data = doc.get_node(id).unwrap();
    data
      .children()
      .filter(|n| n.is_text())
      .map(|n| n.text().unwrap())
      .map(|s| s.splitn(2, ':').map(|s| s.trim()).collect::<Vec<&str>>())
      .filter(|v| v.len() > 1) // TODO quickfix, remove
      .map(|v| (v[0].into(), v[1].into()))
      .collect()
  }

  let (chapter, subchapter) = CHAPTER_MAPPING.get("h_p_signal_symbols").unwrap();
  let xml = get_xml(json, chapter, subchapter)?;
  let doc = Document::parse(&xml)?;

  let mut h_phrases = Err(ParseError::MissingInfo("h phrases".into()));
  let mut p_phrases = Err(ParseError::MissingInfo("p phrases".into()));
  let mut signal_word = Err(ParseError::MissingInfo("signal word".into()));
  let mut symbols = Err(ParseError::MissingInfo("symbols".into()));

  for table in tables(&doc.root().first_child().unwrap(), "block").into_iter() {
    let mut row_iter = table.into_iter();

    if let Some(row) = row_iter.next() {
      let mut data_iter = row
        .into_iter()
        .map(|id| doc.get_node(id).unwrap())
        .peekable();

      if let Some(data) = data_iter.peek() {
        if let Some(inner) = data.first_element_child() {
          if inner.has_tag_name("b") {
            if inner.text() == Some("Gefahrenhinweise - H-Sätze:") {
              if let Some(row) = row_iter.next() {
                h_phrases = Ok(extract_h_p(row[0], &doc));
              }
            } else if inner.text() == Some("Sicherheitshinweise - P-Sätze:") {
              if let Some(row) = row_iter.next() {
                p_phrases = Ok(extract_h_p(row[0], &doc));
              }
            }
          } else if inner.has_tag_name("img") {
            symbols = Ok(
              data_iter
                .map(|n| n.first_element_child())
                .filter(|n| n.is_some())
                .map(|n| {
                  let data = n.unwrap();
                  Image {
                    url: data.attribute("src").unwrap().into(),
                    alt: data.attribute("alt").unwrap().into(),
                  }
                })
                .collect::<Vec<Image>>(),
            );
          } else if inner.has_tag_name("table") {
            let mut table_iter = tables(data, "feldmitlabel")
              .into_iter()
              .flatten()
              .flatten()
              .map(|id| doc.get_node(id).unwrap());
            if let Some(data) = table_iter.next() {
              if let Some(inner) = data.first_element_child() {
                if inner.has_tag_name("b") && inner.text() == Some("Signalwort:") {
                  signal_word = Ok(
                    table_iter
                      .next()
                      .unwrap()
                      .text()
                      .unwrap()
                      .trim_matches('"')
                      .to_string(),
                  );
                }
              }
            }
          }
        }
      }
    }
  }

  Ok((h_phrases, p_phrases, signal_word, symbols))
}

fn get_lethal_dose(json: &GestisResponse) -> Result<Option<String>> {
  let (chapter, subchapter) = CHAPTER_MAPPING.get("lethal_dose").unwrap();
  let xml = get_xml(json, chapter, subchapter)?;
  let doc = Document::parse(&xml)?;

  let mut ld50: Option<&str> = None;

  let mut table_iter = tables(&doc.root().first_child().unwrap(), "block").into_iter();

  while let Some(table) = table_iter.next() {
    let mut row_iter = table.into_iter();

    if let Some(row) = row_iter.next() {
      let mut data_iter = row
        .into_iter()
        .map(|id| doc.get_node(id).unwrap())
        .peekable();

      if let Some(data) = data_iter.peek() {
        if let Some(inner) = data.first_element_child() {
          if inner.has_tag_name("b") {
            if inner.text() == Some("LD50 oral Ratte") {
              if let Some(inner) = ld50 {
                return Err(ParseError::Multiple(inner.into()));
              } else if let Some(table) = table_iter.next() {
                let mut row_iter = table.into_iter();

                if let Some(row) = row_iter.next() {
                  let mut data_iter = row.into_iter().map(|id| doc.get_node(id).unwrap());

                  if let Some(data) = data_iter.next() {
                    if let Some(text) = data.first_child() {
                      if text.text() == Some("Wert:") {
                        if let Some(value) = data_iter.next() {
                          ld50 = value.text();
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  match ld50 {
    Some(inner) => Ok(Some(inner.into())),
    None => Err(ParseError::MissingInfo("lethal dose".into())),
  }
}

/* #endregion */