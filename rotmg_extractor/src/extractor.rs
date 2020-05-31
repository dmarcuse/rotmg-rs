use crate::avm2::abcfile::AbcFile;
use crate::avm2::class::LinkedClass;
use crate::avm2::traits::TraitSlotValue;
use crate::avm2::{Parse, ParseError, Reader};
use rotmg_packets::structured::packets::PacketType;
use rotmg_packets::{BasicParameters, PacketMappings, Parameters};
use std::collections::HashMap;
use swf_parser::swf_types::Tag;
use swf_parser::{parse_swf, SwfParseError};

#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("SWF parse error: {0:?}")]
    SwfParseError(SwfParseError),

    #[error("No bytecode found (is the input correct?)")]
    NoBytecodeFound,

    #[error("Error parsing AVM2 bytecode: {0}")]
    Avm2Error(#[from] ParseError),

    #[error("An expected class wasn't found: {0}")]
    ClassNotFound(&'static str),

    #[error("The RC4 keys couldn't be found")]
    NoRC4Found,

    #[error("Required parameter not found: {0}")]
    ParameterNotFound(&'static str),
}

pub struct ParsedClient {
    abc: AbcFile,
}

impl ParsedClient {
    /// Parse the given flash ROTMG client.
    pub fn new(client: &[u8]) -> Result<Self, ExtractionError> {
        let parsed = parse_swf(client).map_err(ExtractionError::SwfParseError)?;

        let abc_tag = parsed
            .tags
            .into_iter()
            .filter_map(|t| match t {
                Tag::DoAbc(abc) => Some(abc),
                _ => None,
            })
            .next()
            .ok_or(ExtractionError::NoBytecodeFound)?;

        let abc = AbcFile::parse_avm2(&mut Reader::new(&abc_tag.data))?;

        Ok(Self { abc })
    }

    /// Get a class with a given name.
    ///
    /// Package is ignored, only the name of the class itself is checked.
    fn class(&self, name: &'static str) -> Result<LinkedClass, ExtractionError> {
        self.abc
            .classes()
            .find(|c| c.name.1 == name)
            .ok_or(ExtractionError::ClassNotFound(name))
    }

    /// Extract RC4 key from this client, in hexadecimal.
    pub fn extract_rc4(&self) -> Result<&str, ExtractionError> {
        self.abc
            .constants()
            .all_strings()
            .iter()
            .skip_while(|&s| s != "rc4")
            .nth(1)
            .ok_or(ExtractionError::NoRC4Found)
            .map(|s| s.as_ref())
    }

    /// Extract packet ID mappings from this client.
    ///
    /// Note that this operation will succeed even if not all IDs were found.
    /// `PacketMappings::get_unmapped` should be used to detect any missing IDs.
    pub fn extract_packets(&self) -> Result<PacketMappings, ExtractionError> {
        let gsc = self.class("GameServerConnection")?;

        // construct map of currently-unmapped packet types associated by name
        let mut names: HashMap<String, PacketType> = PacketType::VALUES
            .iter()
            .copied()
            .map(|t| (t.name().to_lowercase(), t))
            .collect();

        // construct packet table
        Ok(PacketMappings::new(
            gsc.consts
                .into_iter()
                .filter_map(|t| match t.value {
                    TraitSlotValue::Int(i) => Some((t.name.1.to_lowercase().replace('_', ""), i)),
                    _ => None,
                })
                .filter_map(|(name, id)| names.remove(&name).map(|typ| (typ, id as u8))),
        ))
    }

    /// Extract basic game client parameters.
    pub fn extract_basic_parameters(&self) -> Result<BasicParameters, ExtractionError> {
        let params = self.class("Parameters")?;

        // construct map of constants
        let map: HashMap<&str, TraitSlotValue> = params
            .consts
            .into_iter()
            .map(|t| (t.name.1, t.value))
            .collect();

        let get_param = |n| map.get(n).ok_or(ExtractionError::ParameterNotFound(n));

        let version = {
            let build_version = get_param("BUILD_VERSION")?.as_str()?;
            let minor_version = get_param("MINOR_VERSION")?.as_str()?;
            format!("{}.{}", build_version, minor_version)
        };

        let port = get_param("PORT")?.as_int()? as u16;
        let tutorial_game_id = get_param("TUTORIAL_GAMEID")?.as_int()?;
        let nexus_game_id = get_param("NEXUS_GAMEID")?.as_int()?;
        let random_game_id = get_param("RANDOM_REALM_GAMEID")?.as_int()?;

        Ok(BasicParameters {
            version,
            port,
            tutorial_game_id,
            nexus_game_id,
            random_game_id,
        })
    }

    /// Extract parameters, RC4 keys, and parameters.
    pub fn extract_all(&self) -> Result<Parameters, ExtractionError> {
        let packets = self.extract_packets()?;
        let rc4 = self.extract_rc4()?.to_string();
        let basic = self.extract_basic_parameters()?;
        Ok(Parameters {
            packets,
            rc4,
            basic,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_extraction() {
        let started = Instant::now();

        let client = reqwest::blocking::get("https://realmofthemadgodhrd.appspot.com/client")
            .unwrap()
            .bytes()
            .unwrap();
        let download_time = started.elapsed();

        let parsed = ParsedClient::new(&client).unwrap();
        let params = parsed.extract_all().unwrap();
        let extract_time = started.elapsed() - download_time;

        let missing = params.packets.get_unmapped();
        assert!(missing.is_empty(), "missing packet mappings: {:?}", missing);

        println!("Parameters: {:#?}", params);
        println!(
            "Extraction successful (dl {}s + extract {}s)",
            download_time.as_secs_f32(),
            extract_time.as_secs_f32()
        )
    }
}
