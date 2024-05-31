use crate::error::{CapyError, ErrorCode};
use std::fmt::{self, Display, Formatter};
use std::io::Read;

pub struct Font {
    // Required tables
    font_directory_table: FontDirectoryTable,
    cmap_table: CmapTable,
    glyf_table: GlyfTable,
    head_table: HeadTable,
    hhea_table: HheaTable,
    hmtx_table: HmtxTable,
    loca_table: LocaTable,
    maxp_table: MaxpTable,
    name_table: NameTable,
    post_table: PostTable,

    // Optional tables
    cvt_table: CvtTable,
    fpgm_table: FpgmTable,
    hdmx_table: HdmxTable,
    kern_table: KernTable,
    os2_table: Os2Table,
    prep_table: PrepTable,
}

struct OffsetSubtable {
    scalar_type: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

struct TableDirectorySubtable {
    tag: u32,
    check_sum: u32,
    offset: u32,
    length: u32,
}

struct FontDirectoryTable {
    offset_subtable: OffsetSubtable,
    table_directory_subtables: Vec<TableDirectorySubtable>,
}

struct CmapTable {}

struct GlyfTable {}

struct HeadTable {}

struct HheaTable {}

struct HmtxTable {}

struct LocaTable {}

struct MaxpTable {}

struct NameTable {}

struct PostTable {}

struct CvtTable {}

struct FpgmTable {}

struct HdmxTable {}

struct KernTable {}

struct Os2Table {}

struct PrepTable {}

pub fn parse_from_file(filepath: &str) -> Result<Font, CapyError> {
    let buffer = read_file_to_byte_buffer(filepath)?;
    let mut parser = ByteParser::new(&buffer);

    // Parse required tables
    let font_directory_table = parse_font_directory_table(&mut parser)?;

    Ok(Font {
        // Required tables
        font_directory_table,
        cmap_table: CmapTable {},
        glyf_table: GlyfTable {},
        head_table: HeadTable {},
        hhea_table: HheaTable {},
        hmtx_table: HmtxTable {},
        loca_table: LocaTable {},
        maxp_table: MaxpTable {},
        name_table: NameTable {},
        post_table: PostTable {},

        // Optional tables
        cvt_table: CvtTable {},
        fpgm_table: FpgmTable {},
        hdmx_table: HdmxTable {},
        kern_table: KernTable {},
        os2_table: Os2Table {},
        prep_table: PrepTable {},
    })
}

struct ByteParser<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> ByteParser<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, offset: 0 }
    }

    fn read_be_u32(&mut self) -> Result<u32, CapyError> {
        match self.buffer[self.offset..self.offset + 4].try_into() {
            Ok(bytes) => {
                let value = u32::from_be_bytes(bytes);
                self.offset += 4;
                Ok(value)
            }
            Err(_) => Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u32",
            )),
        }
    }

    fn read_be_u16(&mut self) -> Result<u16, CapyError> {
        match self.buffer[self.offset..self.offset + 2].try_into() {
            Ok(bytes) => {
                let value = u16::from_be_bytes(bytes);
                self.offset += 2;
                Ok(value)
            }
            Err(_) => Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u32",
            )),
        }
    }
}

fn parse_font_directory_table(parser: &mut ByteParser) -> Result<FontDirectoryTable, CapyError> {
    let offset_subtable = parse_offset_table(parser)?;
    let table_directory_subtables =
        parse_table_directory_subtables(parser, offset_subtable.num_tables)?;
    Ok(FontDirectoryTable {
        offset_subtable,
        table_directory_subtables,
    })
}

fn parse_offset_table(parser: &mut ByteParser) -> Result<OffsetSubtable, CapyError> {
    let scalar_type = parser.read_be_u32()?;
    let num_tables = parser.read_be_u16()?;
    let search_range = parser.read_be_u16()?;
    let entry_selector = parser.read_be_u16()?;
    let range_shift = parser.read_be_u16()?;

    Ok(OffsetSubtable {
        scalar_type,
        num_tables,
        search_range,
        entry_selector,
        range_shift,
    })
}

fn parse_table_directory_subtables(
    parser: &mut ByteParser,
    num_tables: u16,
) -> Result<Vec<TableDirectorySubtable>, CapyError> {
    let mut subtables = Vec::new();
    for _ in 0..num_tables {
        let tag = parser.read_be_u32()?;
        let check_sum = parser.read_be_u32()?;
        let offset = parser.read_be_u32()?;
        let length = parser.read_be_u32()?;
        subtables.push(TableDirectorySubtable {
            tag,
            check_sum,
            offset,
            length,
        });
    }
    Ok(subtables)
}

fn read_file_to_byte_buffer(filepath: &str) -> Result<Vec<u8>, CapyError> {
    let mut file = std::fs::File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "font parsing is unimplemented")
    }
}
