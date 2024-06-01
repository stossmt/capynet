use crate::error::{CapyError, ErrorCode};
use core::num;
use std::io::Read;

#[derive(Debug)]
pub struct Font {
    font_directory_table: FontDirectoryTable,
    cmap_table: CmapTable,
    // Other required tables can be added here as needed
}

#[derive(Debug)]
struct OffsetSubtable {
    scalar_type: u32,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
}

#[derive(Debug)]
struct TableDirectorySubtable {
    tag: u32,
    check_sum: u32,
    offset: u32,
    length: u32,
}

#[derive(Debug)]
struct FontDirectoryTable {
    offset_subtable: OffsetSubtable,
    table_directory_subtables: Vec<TableDirectorySubtable>,
}

#[derive(Debug)]
struct CmapFormatZeroTable {
    format: u16,
    length: u16,
    language: u16,
    glyph_index_array: [u8; 256],
}

#[derive(Debug)]
struct CmapFormatFourTable {
    format: u16,
    length: u16,
    language: u16,
    seg_count_x2: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    end_code: Vec<u16>,
    reserved_pad: u16,
    start_code: Vec<u16>,
    id_delta: Vec<u16>,
    id_range_offset: Vec<u16>,
    glyph_id_array: Vec<u16>,
}

#[derive(Debug)]
struct CmapEncodingSubtable {
    platform_id: u16,
    platform_specific_id: u16,
    offset: u32,
}

#[derive(Debug)]
struct CmapTable {
    version: u16,
    num_subtables: u16,
    encoding_subtables: Vec<CmapEncodingSubtable>,
    format_zero_table: Option<CmapFormatZeroTable>,
    format_four_table: Option<CmapFormatFourTable>,
}

enum TableTag {
    Dsig = 1146308935,
    Gdef = 1195656518,
    Gpos = 1196445523,
    Gsub = 1196643650,
    Jstf = 1246975046,
    Ltsh = 1280594760,
    Os2 = 1330851634,
    Pclt = 1346587732,
    Vdmx = 1447316824,
    Cmap = 1668112752,
    Cvt = 1668707360,
    Fpgm = 1718642541,
    Gasp = 1734439792,
    Glyf = 1735162214,
    Hdmx = 1751412088,
    Head = 1751474532,
    Hhea = 1751672161,
    Hmtx = 1752003704,
    Kern = 1801810542,
    Loca = 1819239265,
    Maxp = 1835104368,
    Meta = 1835365473,
    Name = 1851878757,
    Post = 1886352244,
    Prep = 1886545264,
}

pub fn parse_from_file(filepath: &str) -> Result<Font, CapyError> {
    let buffer = read_file_to_byte_buffer(filepath)?;

    let mut parser = ByteParser::new(&buffer);

    let font_directory_table = parse_font_directory_table(&mut parser)?;
    let cmap_table = parse_cmap_table(&mut parser, &font_directory_table)?;

    Ok(Font {
        font_directory_table,
        cmap_table,
    })
}

struct ByteParser<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> ByteParser<'a> {
    const U32_SIZE: usize = 4;
    const U16_SIZE: usize = 2;
    const U8_256_ARRAY_SIZE: usize = 256;

    fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, offset: 0 }
    }

    fn set_offset(&mut self, offset: usize) -> Result<(), CapyError> {
        if offset > self.buffer.len() {
            return Err(CapyError::new(
                ErrorCode::OutOfRange,
                "failed to slice buffer for tag",
            ));
        }

        self.offset = offset;
        Ok(())
    }

    fn read_u8_array_256(&mut self) -> Result<[u8; 256], CapyError> {
        if self.offset + Self::U8_256_ARRAY_SIZE <= self.buffer.len() {
            let bytes = &self.buffer[self.offset..self.offset + Self::U8_256_ARRAY_SIZE];
            self.offset += Self::U8_256_ARRAY_SIZE;
            Ok(bytes.try_into().unwrap())
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u8 array",
            ))
        }
    }

    fn read_be_u32(&mut self) -> Result<u32, CapyError> {
        if self.offset + Self::U32_SIZE <= self.buffer.len() {
            let bytes = &self.buffer[self.offset..self.offset + Self::U32_SIZE];
            self.offset += Self::U32_SIZE;
            Ok(u32::from_be_bytes(bytes.try_into().unwrap()))
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u32",
            ))
        }
    }

    fn read_be_u16(&mut self) -> Result<u16, CapyError> {
        if self.offset + Self::U16_SIZE <= self.buffer.len() {
            let bytes = &self.buffer[self.offset..self.offset + Self::U16_SIZE];
            self.offset += Self::U16_SIZE;
            Ok(u16::from_be_bytes(bytes.try_into().unwrap()))
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u16",
            ))
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
    Ok(OffsetSubtable {
        scalar_type: parser.read_be_u32()?,
        num_tables: parser.read_be_u16()?,
        search_range: parser.read_be_u16()?,
        entry_selector: parser.read_be_u16()?,
        range_shift: parser.read_be_u16()?,
    })
}

fn parse_table_directory_subtables(
    parser: &mut ByteParser,
    num_tables: u16,
) -> Result<Vec<TableDirectorySubtable>, CapyError> {
    let mut subtables = Vec::new();
    for _ in 0..num_tables {
        subtables.push(TableDirectorySubtable {
            tag: parser.read_be_u32()?,
            check_sum: parser.read_be_u32()?,
            offset: parser.read_be_u32()?,
            length: parser.read_be_u32()?,
        });
    }
    Ok(subtables)
}

fn parse_cmap_table(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
) -> Result<CmapTable, CapyError> {
    let cmap_offset = lookup_offset_for_tag(TableTag::Cmap, font_directory_table)?;
    parser.set_offset(cmap_offset)?;
    let version = parser.read_be_u16()?;
    let num_subtables = parser.read_be_u16()?;
    let encoding_subtables = parse_cmap_encoding_subtables(parser, num_subtables)?;
    let mut format_zero_table = None;
    let mut format_four_table = None;
    for table in encoding_subtables.iter() {
        parser.set_offset(cmap_offset + table.offset as usize)?;
        let format = parser.read_be_u16()?;
        match format {
            0 => {
                let tmp_table = parse_cmap_format_zero(parser, format)?;
                format_zero_table = Some(tmp_table);
            }
            4 => {
                let tmp_table = parse_cmap_format_four(parser, format)?;
                format_four_table = Some(tmp_table);
            }
            _ => todo!("Implement other cmap formats"),
        }
    }
    Ok(CmapTable {
        version,
        num_subtables,
        encoding_subtables,
        format_zero_table,
        format_four_table,
    })
}

fn parse_cmap_format_zero(
    parser: &mut ByteParser,
    format: u16,
) -> Result<CmapFormatZeroTable, CapyError> {
    Ok(CmapFormatZeroTable {
        format,
        length: parser.read_be_u16()?,
        language: parser.read_be_u16()?,
        glyph_index_array: parser.read_u8_array_256()?,
    })
}

fn parse_cmap_format_four(
    parser: &mut ByteParser,
    format: u16,
) -> Result<CmapFormatFourTable, CapyError> {
    let seg_count_x2 = parser.read_be_u16()?;
    let seg_count = seg_count_x2 / 2;
    let search_range = parser.read_be_u16()?;
    let entry_selector = parser.read_be_u16()?;
    let range_shift = parser.read_be_u16()?;
    let mut end_code = Vec::new();
    for _ in 0..seg_count {
        end_code.push(parser.read_be_u16()?);
    }
    let reserved_pad = parser.read_be_u16()?;
    let mut start_code = Vec::new();
    for _ in 0..seg_count {
        start_code.push(parser.read_be_u16()?);
    }
    let mut id_delta = Vec::new();
    for _ in 0..seg_count {
        id_delta.push(parser.read_be_u16()?);
    }
    let mut id_range_offset = Vec::new();
    for _ in 0..seg_count {
        id_range_offset.push(parser.read_be_u16()?);
    }
    let mut glyph_id_array = Vec::new();
    for _ in 0..seg_count {
        glyph_id_array.push(parser.read_be_u16()?);
    }
    Ok(CmapFormatFourTable {
        format,
        length: parser.read_be_u16()?,
        language: parser.read_be_u16()?,
        seg_count_x2,
        search_range,
        entry_selector,
        range_shift,
        end_code,
        reserved_pad,
        start_code,
        id_delta,
        id_range_offset,
        glyph_id_array,
    })
}

fn parse_cmap_encoding_subtables(
    parser: &mut ByteParser,
    num_subtables: u16,
) -> Result<Vec<CmapEncodingSubtable>, CapyError> {
    let mut tables = Vec::new();
    for _ in 0..num_subtables {
        tables.push(CmapEncodingSubtable {
            platform_id: parser.read_be_u16()?,
            platform_specific_id: parser.read_be_u16()?,
            offset: parser.read_be_u32()?,
        });
    }
    Ok(tables)
}

fn read_file_to_byte_buffer(filepath: &str) -> Result<Vec<u8>, CapyError> {
    let mut file = std::fs::File::open(filepath)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn lookup_offset_for_tag(
    tag: TableTag,
    font_directory_table: &FontDirectoryTable,
) -> Result<usize, CapyError> {
    let desired_tag = tag as u32;
    let table_dir = font_directory_table
        .table_directory_subtables
        .iter()
        .find(|&dir| dir.tag == desired_tag)
        .ok_or_else(|| {
            CapyError::new(ErrorCode::NotFound, "table not found in FontDirectoryTable")
        })?;
    Ok(table_dir.offset as usize)
}
