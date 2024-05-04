//! Generation of dat files.

use anyhow::{anyhow, Error};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::reader::Reader;
use quick_xml::Writer;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, Cursor, ErrorKind, Write};
use std::path::Path;
use std::thread::{Scope, ScopedJoinHandle};
use std::{fs, thread, vec};
use zip::read::ZipFile;
use zip::ZipArchive;

use crate::files::{ALL_NON_ZIPPED_CONTENT, ARTWORK, SAMPLES};
use crate::Config;

/// Custom result with any context error.
type Result<T> = anyhow::Result<T>;

/// Game configuration for a specific input dat.
struct GameConfig<'a> {
    /// Optional root dir (for artwork and samples)
    root_dir: Option<&'a str>,
    /// Optional directories (for dats and folders)
    dirs: Vec<&'a str>,
    /// Dat file name
    dat: &'a str,
    /// Input Zip file path
    zip: &'a Path,
}

/// Dat content result from output generation.
struct DatContent {
    /// Content generated for all roms
    all: String,
    /// Content generated for artwork
    artwork: String,
    /// Content generated for samples
    samples: String,
}

/// Generate output file using dats from input Zip file.
///
/// # Errors
///
/// Will return `Err` if an error occured during XML read or XML write.
pub fn generate_output(config: &Config) -> Result<()> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // Declaration
    add_declaration(&mut writer)?;

    // Doctype
    add_doctype(&mut writer)?;

    // Add start tag for datafile
    writer.write_event(Event::Start(BytesStart::new("datafile")))?;

    // Add headers
    add_headers(&mut writer, config.version)?;

    let result = thread::scope(|scope| -> Result<DatContent> {
        // Spawn thread to compute all content
        let config_all = Box::new(GameConfig {
            root_dir: None,
            dirs: vec!["dats", "folders"],
            dat: ALL_NON_ZIPPED_CONTENT,
            zip: &config.input_file_path,
        });
        let all = build_handle(scope, config_all);

        // Spawn thread to compute artwork
        let config_artwork = Box::new(GameConfig {
            root_dir: Some("artwork"),
            dirs: Vec::new(),
            dat: ARTWORK,
            zip: &config.input_file_path,
        });
        let artwork = build_handle(scope, config_artwork);

        // Spawn thread to compute samples
        let config_samples = Box::new(GameConfig {
            root_dir: Some("samples"),
            dirs: Vec::new(),
            dat: SAMPLES,
            zip: &config.input_file_path,
        });
        let samples = build_handle(scope, config_samples);

        let Ok(join_all_result) = all.join() else {
            return Err(Error::msg("Failed to generate all content"));
        };
        let all = join_all_result.unwrap_or_default();

        let Ok(join_artwork_result) = artwork.join() else {
            return Err(Error::msg("Failed to generate artwork content"));
        };
        let artwork = join_artwork_result.unwrap_or_default();

        let Ok(join_samples_result) = samples.join() else {
            return Err(Error::msg("Failed to generate samples content"));
        };
        let samples = join_samples_result.unwrap_or_default();

        Ok(DatContent {
            all,
            artwork,
            samples,
        })
    });

    // Write threads results to main writer
    let content = match result {
        Ok(content) => content,
        Err(err) => return Err(err),
    };
    writer.write_event(Event::Text(BytesText::from_escaped(content.all)))?;
    writer.write_event(Event::Text(BytesText::from_escaped(content.artwork)))?;
    writer.write_event(Event::Text(BytesText::from_escaped(content.samples)))?;

    // Add end tag for datafile
    writer.write_event(Event::End(BytesEnd::new("datafile")))?;

    write_to_file(writer, &config.output_file_path)?;

    Ok(())
}

/// Build thread handle in order to generate output for specified config in another thread.
fn build_handle<'a>(
    scope: &'a Scope<'a, '_>,
    config: Box<GameConfig<'a>>,
) -> ScopedJoinHandle<'a, Result<String>> {
    let handle = scope.spawn(move || -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        let zip_file = File::open(config.zip)?;
        let mut zip = ZipArchive::new(&zip_file)?;
        let entry = zip.by_name(config.dat)?;
        let mut reader = Reader::from_reader(BufReader::new(entry));

        if add_games(&mut writer, &config, &mut reader).is_err() {
            return Ok(String::new());
        }

        let result = writer.into_inner().into_inner();
        Ok(String::from_utf8(result).unwrap_or_default())
    });

    handle
}

/// Add games for the specified configuration.
fn add_games(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    config: &GameConfig,
    reader: &mut Reader<BufReader<ZipFile>>,
) -> Result<()> {
    /// Helper state to parse input dat
    enum State {
        /// Datafile section state.
        Datafile,
        /// Machine section state.
        Machine,
        /// Description section state.
        Description,
    }
    let mut buf = Vec::new();
    let mut state = State::Datafile;
    let dirs = &config.dirs;
    let mut close_dir = false;
    let root_dir = config.root_dir;
    let dir = "dir";

    if root_dir.is_some() {
        let mut dir = BytesStart::new(dir);
        let attr = Attribute {
            key: QName(b"name"),
            value: Cow::from(root_dir.unwrap().as_bytes()),
        };
        dir.push_attribute(attr);
        writer.write_event(Event::Start(dir))?;
    }

    loop {
        match (&state, reader.read_event_into(&mut buf)) {
            (State::Datafile, Ok(Event::Start(tag))) => {
                if tag.name().as_ref() == b"machine" {
                    state = State::Machine;
                    let name_attribute = tag.try_get_attribute("name")?.unwrap();
                    if !dirs.is_empty() {
                        let value = String::from_utf8(name_attribute.value.to_vec())?;

                        if dirs.contains(&value.as_str()) {
                            let mut dir = BytesStart::new(dir);
                            dir.push_attribute(name_attribute.clone());
                            writer.write_event(Event::Start(dir))?;
                            close_dir = true;
                        }
                    }

                    let mut game = BytesStart::new("game");
                    game.push_attribute(name_attribute);
                    writer.write_event(Event::Start(game))?;
                }
            }
            (State::Machine, Ok(Event::Start(e))) => {
                if e.name().as_ref() == b"description" {
                    state = State::Description;
                    writer.write_event(Event::Start(e))?;
                }
            }
            (State::Description, Ok(Event::Text(e))) => {
                writer.write_event(Event::Text(e))?;
            }
            (State::Machine, Ok(Event::Empty(e))) => {
                if e.name().as_ref() == b"rom" {
                    writer.write_event(Event::Empty(e))?;
                }
            }
            (State::Description, Ok(Event::End(e))) => {
                if e.name().as_ref() == b"description" {
                    state = State::Machine;
                    writer.write_event(Event::End(e))?;
                }
            }
            (State::Machine, Ok(Event::End(e))) => {
                if e.name().as_ref() == b"machine" {
                    state = State::Datafile;
                    let game = BytesEnd::new("game");
                    writer.write_event(Event::End(game))?;
                    if close_dir {
                        close_dir = false;
                        let dir = BytesEnd::new(dir);
                        writer.write_event(Event::End(dir))?;
                    }
                }
            }
            (_, Ok(Event::Eof)) => break,
            (_, Err(err)) => panic!("Error at position {}: {:?}", reader.buffer_position(), err),
            _ => (),
        }
        buf.clear();
    }

    if root_dir.is_some() {
        let dir = BytesEnd::new(dir);
        writer.write_event(Event::End(dir))?;
    }

    Ok(())
}

/// Add XML declaration to writer
fn add_declaration(writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
    let declaration = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(declaration))?;

    Ok(())
}

/// Add XML doctype to writer
fn add_doctype(writer: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
    // TODO Update logiqx original DTD which doesn't support dir tags
    /// Doctype to write to XML output
    const DOCTYPE: &str = "datafile PUBLIC \"-//Logiqx//DTD ROM Management Datafile//EN\" \"http://www.logiqx.com/Dats/datafile.dtd\"";
    let doctype = BytesText::from_escaped(DOCTYPE);
    writer.write_event(Event::DocType(doctype))?;

    Ok(())
}

/// Add all expected headers to writer
fn add_headers(writer: &mut Writer<Cursor<Vec<u8>>>, version: f32) -> Result<()> {
    let name = "header";
    writer.write_event(Event::Start(BytesStart::new(name)))?;
    add_header(writer, "name", "Extras")?;
    add_header(
        writer,
        "description",
        &format!("MAME {version} Extras (all content)"),
    )?;
    add_header(writer, "category", "Standard DatFile")?;
    add_header(writer, "version", &version.to_string())?;
    add_header(writer, "author", "Pleasuredome")?;
    add_header(
        writer,
        "homepage",
        "https://github.com/fragoulin/convert-mame-extras-romvault",
    )?;
    add_header(
        writer,
        "url",
        "https://pleasuredome.miraheze.org/wiki/MAME_EXTRAs",
    )?;
    add_header(writer, "comment", "Compatible with RomVault")?;
    writer.write_event(Event::End(BytesEnd::new(name)))?;

    Ok(())
}

/// Add header to writer.
fn add_header(writer: &mut Writer<Cursor<Vec<u8>>>, name: &str, value: &str) -> Result<()> {
    writer.write_event(Event::Start(BytesStart::new(name)))?;
    writer.write_event(Event::Text(BytesText::new(value)))?;
    writer.write_event(Event::End(BytesEnd::new(name)))?;

    Ok(())
}

/// Write generated dat to specified output file.
fn write_to_file(writer: Writer<Cursor<Vec<u8>>>, output_file_path: &Path) -> Result<()> {
    let result = writer.into_inner().into_inner();
    let file_result = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(output_file_path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => {
                return Err(anyhow!(
                    "file {} already exists",
                    output_file_path.display()
                ))
            }
            _ => return Err(err.into()),
        },
    };

    file.write_all(&result)?;

    Ok(())
}
