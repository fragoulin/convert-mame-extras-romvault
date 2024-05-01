use anyhow::anyhow;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::io::{Cursor, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{fs, vec};

use crate::files::{ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES};

type Result<T> = anyhow::Result<T>;

struct GameConfig {
    path: PathBuf,
    root_dir: Option<String>,
    dirs: Option<Vec<String>>,
}

/// # Errors
///
/// Will return `Err` if an error occured during XML read or XML write.
pub fn generate_output(output_file_path: &Path, version: f32, temp_dir: &Path) -> Result<()> {
    let all_content_path = PathBuf::from(&temp_dir).join(ALL_NON_ZIPPED_CONTENT);
    let artwork_path = PathBuf::from(&temp_dir).join(ARTWORK);
    let samples_path = PathBuf::from(&temp_dir).join(SAMPLES);
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // Declaration
    add_declaration(&mut writer);

    // Doctype
    add_doctype(&mut writer);

    // Add start tag for datafile
    writer.write_event(Event::Start(BytesStart::new("datafile")))?;

    // Add headers
    add_headers(&mut writer, version);

    let config_all = GameConfig {
        path: all_content_path,
        root_dir: None,
        dirs: Some(vec![String::from("dats"), String::from("folders")]),
    };
    add_games(&mut writer, &config_all)?;

    let config_artwork = GameConfig {
        path: artwork_path,
        root_dir: Some(String::from("artwork")),
        dirs: None,
    };
    add_games(&mut writer, &config_artwork)?;

    let config_samples = GameConfig {
        path: samples_path,
        root_dir: Some(String::from("samples")),
        dirs: None,
    };
    add_games(&mut writer, &config_samples)?;

    // Add end tag for datafile
    writer.write_event(Event::End(BytesEnd::new("datafile")))?;

    write_to_file(writer, output_file_path)?;

    Ok(())
}

fn add_games(writer: &mut Writer<Cursor<Vec<u8>>>, config: &GameConfig) -> Result<()> {
    enum State {
        Datafile,
        Machine,
        Description,
    }
    let mut reader = Reader::from_file(&config.path)?;
    let mut buf = Vec::new();
    let mut state = State::Datafile;
    let dirs = config.dirs.as_ref();
    let mut close_dir = false;
    let root_dir = config.root_dir.as_ref();

    if root_dir.is_some() {
        let mut dir = BytesStart::new("dir");
        let attr = Attribute {
            key: QName(b"name"),
            value: Cow::from(root_dir.unwrap().as_bytes()),
        };
        dir.push_attribute(attr);
        assert!(writer.write_event(Event::Start(dir)).is_ok());
    }

    loop {
        match (&state, reader.read_event_into(&mut buf)) {
            (State::Datafile, Ok(Event::Start(e))) => {
                if e.name().as_ref() == b"machine" {
                    state = State::Machine;
                    let name_attribute = e.try_get_attribute("name")?.unwrap();
                    let value = String::from_utf8(name_attribute.value.to_vec())?;
                    let add_dir = dirs.is_some() && dirs.unwrap().contains(&value);

                    let mut game = BytesStart::new("game");
                    game.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                    if add_dir {
                        let mut dir = BytesStart::new("dir");
                        dir.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                        assert!(writer.write_event(Event::Start(dir)).is_ok());
                        close_dir = true;
                    }
                    assert!(writer.write_event(Event::Start(game)).is_ok());
                }
            }
            (State::Machine, Ok(Event::Start(e))) => {
                if e.name().as_ref() == b"description" {
                    state = State::Description;
                    assert!(writer.write_event(Event::Start(e)).is_ok());
                }
            }
            (State::Description, Ok(Event::Text(e))) => {
                assert!(writer.write_event(Event::Text(e)).is_ok());
            }
            (State::Machine, Ok(Event::Empty(e))) => {
                if e.name().as_ref() == b"rom" {
                    assert!(writer.write_event(Event::Empty(e)).is_ok());
                }
            }
            (State::Description, Ok(Event::End(e))) => {
                if e.name().as_ref() == b"description" {
                    state = State::Machine;
                    assert!(writer.write_event(Event::End(e)).is_ok());
                }
            }
            (State::Machine, Ok(Event::End(e))) => {
                if e.name().as_ref() == b"machine" {
                    state = State::Datafile;
                    let game = BytesEnd::new("game");
                    assert!(writer.write_event(Event::End(game)).is_ok());
                    if close_dir {
                        close_dir = false;
                        let dir = BytesEnd::new("dir");
                        assert!(writer.write_event(Event::End(dir)).is_ok());
                    }
                }
            }
            (_, Ok(Event::Eof)) => break,
            (_, Err(e)) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    if root_dir.is_some() {
        let dir = BytesEnd::new("dir");
        assert!(writer.write_event(Event::End(dir)).is_ok());
    }

    Ok(())
}

fn add_declaration(writer: &mut Writer<Cursor<Vec<u8>>>) {
    let declaration = BytesDecl::new("1.0", Some("UTF-8"), None);
    assert!(writer.write_event(Event::Decl(declaration)).is_ok());
}

fn add_doctype(writer: &mut Writer<Cursor<Vec<u8>>>) {
    // TODO Update logiqx original DTD which doesn't support dir tags
    const DOCTYPE: &str = "datafile PUBLIC \"-//Logiqx//DTD ROM Management Datafile//EN\" \"http://www.logiqx.com/Dats/datafile.dtd\"";
    let doctype = BytesText::from_escaped(DOCTYPE);
    assert!(writer.write_event(Event::DocType(doctype)).is_ok());
}

fn add_headers(writer: &mut Writer<Cursor<Vec<u8>>>, version: f32) {
    let name = "header";
    assert!(writer
        .write_event(Event::Start(BytesStart::new(name)))
        .is_ok());
    add_header(writer, "name", "Extras");
    add_header(
        writer,
        "description",
        &format!("MAME {version} Extras (all content)"),
    );
    add_header(writer, "category", "Standard DatFile");
    add_header(writer, "version", &version.to_string());
    add_header(writer, "author", "Pleasuredome");
    add_header(
        writer,
        "homepage",
        "https://github.com/fragoulin/convert-mame-extras-romvault",
    );
    add_header(
        writer,
        "url",
        "https://pleasuredome.miraheze.org/wiki/MAME_EXTRAs",
    );
    add_header(writer, "comment", "Compatible with RomVault");
    assert!(writer.write_event(Event::End(BytesEnd::new(name))).is_ok());
}

fn add_header(writer: &mut Writer<Cursor<Vec<u8>>>, name: &str, value: &str) {
    assert!(writer
        .write_event(Event::Start(BytesStart::new(name)))
        .is_ok());
    assert!(writer
        .write_event(Event::Text(BytesText::new(value)))
        .is_ok());
    assert!(writer.write_event(Event::End(BytesEnd::new(name))).is_ok());
}

fn write_to_file(writer: Writer<Cursor<Vec<u8>>>, output_file_path: &Path) -> Result<()> {
    let result = writer.into_inner().into_inner();
    let file_result = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output_file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => {
                return Err(anyhow!(
                    "file {} already exists",
                    output_file_path.display()
                ))
            }
            _ => return Err(e.into()),
        },
    };

    file.write_all(&result)?;

    Ok(())
}
