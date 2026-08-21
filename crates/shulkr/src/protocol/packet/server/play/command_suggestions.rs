use crate::{
    command::suggestion::Suggestions,
    protocol::{
        encode::{Encode, EncodeError, PacketWrite},
        packet::{Packet, ServerPacket},
    },
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub struct CommandSuggestionsPacket {
    pub transaction_id: i32,
    pub start: i32,
    pub length: i32,
    pub matches: Vec<SuggestionMatch>,
}

#[derive(Debug, Clone)]
pub struct SuggestionMatch {
    pub text: String,
    pub tooltip: Option<TextComponent>,
}

impl CommandSuggestionsPacket {
    pub fn from_suggestions(transaction_id: i32, input: &str, suggestions: &Suggestions) -> Self {
        let range = suggestions.range();
        let start = utf16_offset(input, range.start());
        let end = utf16_offset(input, range.end());

        Self {
            transaction_id,
            start: start as i32,
            length: (end - start) as i32,
            matches: suggestions
                .list()
                .iter()
                .map(|suggestion| SuggestionMatch {
                    text: suggestion.text().to_string(),
                    tooltip: suggestion.tooltip().cloned(),
                })
                .collect(),
        }
    }
}

fn utf16_offset(input: &str, byte_offset: usize) -> usize {
    let byte_offset = byte_offset.min(input.len());
    input[..byte_offset].chars().map(char::len_utf16).sum()
}

impl Packet for CommandSuggestionsPacket {}
impl ServerPacket for CommandSuggestionsPacket {}

impl Encode for CommandSuggestionsPacket {
    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.transaction_id)?;
        w.write_varint(this.start)?;
        w.write_varint(this.length)?;

        w.write_varint(this.matches.len() as i32)?;
        for entry in &this.matches {
            w.write_string(&entry.text)?;
            w.write_option(&entry.tooltip, |w, tooltip| w.write_component(tooltip))?;
        }

        Ok(())
    }
}
