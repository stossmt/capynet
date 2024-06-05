use crate::error::{CapyError, ErrorCode};
use core::num;
use std::io::Read;

#[derive(Debug)]
pub struct Font {
    font_directory_table: FontDirectoryTable,
    cmap_table: CmapTable,
    head_table: HeadTable,
    hhea_table: HheaTable,
    maxp_table: MaxpTable,
    glyf_table: GlyfTable,
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

#[derive(Debug)]
struct HeadTable {
    version: u32,
    font_revision: u32,
    check_sum_adjustment: u32,
    magic_number: u32,
    flags: u16,
    units_per_em: u16,
    created: u32,
    modified: u32,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    mac_style: u16,
    lowest_rec_ppem: u16,
    font_direction_hint: i16,
    index_to_loc_format: i16,
    glyph_data_format: i16,
}

#[derive(Debug)]
struct HheaTable {
    version: u32,
    ascent: i16,
    descent: i16,
    line_gap: i16,
    advance_width_max: u16,
    min_left_side_bearing: i16,
    min_right_side_bearing: i16,
    x_max_extent: i16,
    caret_slope_rise: i16,
    caret_slope_run: i16,
    caret_offset: i16,
    reserved: [i16; 4],
    metric_data_format: i16,
    number_of_hmetrics: u16,
}

#[derive(Debug)]
struct MaxpTable {
    version: u32,
    num_glyphs: u16,
    max_points: u16,
    max_contours: u16,
    max_composite_points: u16,
    max_composite_contours: u16,
    max_zones: u16,
    max_twilight_points: u16,
    max_storage: u16,
    max_function_defs: u16,
    max_instruction_defs: u16,
    max_stack_elements: u16,
    max_size_of_instructions: u16,
    max_component_elements: u16,
    max_component_depth: u16,
}

#[derive(Debug)]
struct GlyfSubtable {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    end_pts_of_contours: Vec<u16>,
    instruction_length: u16,
    instructions: Vec<u8>,
    flags: Vec<u8>,
    x_coordinates: Vec<i16>,
    y_coordinates: Vec<i16>,
}

#[derive(Debug)]
struct GlyfTable {
    glyphs: Vec<GlyfSubtable>,
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
    let head_table = parse_head_table(&mut parser, &font_directory_table)?;
    let hhea_table = parse_hhea_table(&mut parser, &font_directory_table)?;
    let maxp_table = parse_maxp_table(&mut parser, &font_directory_table)?;
    let glyf_table = parse_glyf_table(&mut parser, &font_directory_table, maxp_table.num_glyphs)?;

    Ok(Font {
        font_directory_table,
        cmap_table,
        head_table,
        hhea_table,
        maxp_table,
        glyf_table,
    })
}

struct ByteParser<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> ByteParser<'a> {
    const U8_SIZE: usize = 1;
    const U32_SIZE: usize = 4;
    const U16_SIZE: usize = 2;
    const I16_SIZE: usize = 2;

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
        if self.offset + Self::U8_SIZE * 256 <= self.buffer.len() {
            let bytes = &self.buffer[self.offset..self.offset + Self::U8_SIZE * 256];
            self.offset += Self::U8_SIZE * 256;
            Ok(bytes.try_into().unwrap())
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u8 array",
            ))
        }
    }

    fn read_be_i16_array_4(&mut self) -> Result<[i16; 4], CapyError> {
        if self.offset + Self::I16_SIZE * 4 <= self.buffer.len() {
            let mut array = [0; 4];
            for i in 0..4 {
                let bytes = &self.buffer[self.offset..self.offset + Self::I16_SIZE];
                self.offset += Self::I16_SIZE;
                array[i] = i16::from_be_bytes(bytes.try_into().unwrap());
            }
            Ok(array)
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for i16 array",
            ))
        }
    }

    fn read_be_u8(&mut self) -> Result<u8, CapyError> {
        if self.offset + Self::U8_SIZE <= self.buffer.len() {
            let byte = self.buffer[self.offset];
            self.offset += Self::U8_SIZE;
            Ok(byte)
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for u8",
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

    fn read_be_i16(&mut self) -> Result<i16, CapyError> {
        if self.offset + Self::U16_SIZE <= self.buffer.len() {
            let bytes = &self.buffer[self.offset..self.offset + Self::U16_SIZE];
            self.offset += Self::U16_SIZE;
            Ok(i16::from_be_bytes(bytes.try_into().unwrap()))
        } else {
            Err(CapyError::new(
                ErrorCode::OutOfRange,
                "Buffer too small for i16",
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

fn parse_head_table(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
) -> Result<HeadTable, CapyError> {
    let head_offset = lookup_offset_for_tag(TableTag::Head, font_directory_table)?;
    parser.set_offset(head_offset)?;
    Ok(HeadTable {
        version: parser.read_be_u32()?,
        font_revision: parser.read_be_u32()?,
        check_sum_adjustment: parser.read_be_u32()?,
        magic_number: parser.read_be_u32()?,
        flags: parser.read_be_u16()?,
        units_per_em: parser.read_be_u16()?,
        created: parser.read_be_u32()?,
        modified: parser.read_be_u32()?,
        x_min: parser.read_be_i16()?,
        y_min: parser.read_be_i16()?,
        x_max: parser.read_be_i16()?,
        y_max: parser.read_be_i16()?,
        mac_style: parser.read_be_u16()?,
        lowest_rec_ppem: parser.read_be_u16()?,
        font_direction_hint: parser.read_be_i16()?,
        index_to_loc_format: parser.read_be_i16()?,
        glyph_data_format: parser.read_be_i16()?,
    })
}

fn parse_hhea_table(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
) -> Result<HheaTable, CapyError> {
    let hhea_offset = lookup_offset_for_tag(TableTag::Hhea, font_directory_table)?;
    parser.set_offset(hhea_offset)?;
    Ok(HheaTable {
        version: parser.read_be_u32()?,
        ascent: parser.read_be_i16()?,
        descent: parser.read_be_i16()?,
        line_gap: parser.read_be_i16()?,
        advance_width_max: parser.read_be_u16()?,
        min_left_side_bearing: parser.read_be_i16()?,
        min_right_side_bearing: parser.read_be_i16()?,
        x_max_extent: parser.read_be_i16()?,
        caret_slope_rise: parser.read_be_i16()?,
        caret_slope_run: parser.read_be_i16()?,
        caret_offset: parser.read_be_i16()?,
        reserved: parser.read_be_i16_array_4()?,
        metric_data_format: parser.read_be_i16()?,
        number_of_hmetrics: parser.read_be_u16()?,
    })
}

fn parse_maxp_table(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
) -> Result<MaxpTable, CapyError> {
    let maxp_offset = lookup_offset_for_tag(TableTag::Maxp, font_directory_table)?;
    parser.set_offset(maxp_offset)?;
    Ok(MaxpTable {
        version: parser.read_be_u32()?,
        num_glyphs: parser.read_be_u16()?,
        max_points: parser.read_be_u16()?,
        max_contours: parser.read_be_u16()?,
        max_composite_points: parser.read_be_u16()?,
        max_composite_contours: parser.read_be_u16()?,
        max_zones: parser.read_be_u16()?,
        max_twilight_points: parser.read_be_u16()?,
        max_storage: parser.read_be_u16()?,
        max_function_defs: parser.read_be_u16()?,
        max_instruction_defs: parser.read_be_u16()?,
        max_stack_elements: parser.read_be_u16()?,
        max_size_of_instructions: parser.read_be_u16()?,
        max_component_elements: parser.read_be_u16()?,
        max_component_depth: parser.read_be_u16()?,
    })
}

fn parse_glyf_table(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
    num_glyphs: u16,
) -> Result<GlyfTable, CapyError> {
    let mut glyphs = Vec::new();
    for _ in 0..num_glyphs {
        let glyph = parse_glyph_subtable(parser, font_directory_table)?;
        if glyph.number_of_contours < 0 {
            println!("Reached compound glyph");
            break;
        }
        glyphs.push(glyph);
    }
    Ok(GlyfTable { glyphs })
}

fn parse_glyph_subtable(
    parser: &mut ByteParser,
    font_directory_table: &FontDirectoryTable,
) -> Result<GlyfSubtable, CapyError> {
    let glyf_offset = lookup_offset_for_tag(TableTag::Glyf, font_directory_table)?;
    parser.set_offset(glyf_offset)?;

    let number_of_contours = parser.read_be_i16()?;
    let x_min = parser.read_be_i16()?;
    let y_min = parser.read_be_i16()?;
    let x_max = parser.read_be_i16()?;
    let y_max = parser.read_be_i16()?;

    let mut end_pts_of_contours = Vec::new();
    for i in 0..number_of_contours {
        end_pts_of_contours.push(parser.read_be_u16()?);
    }

    let instruction_length = parser.read_be_u16()?;
    let mut instructions = Vec::new();
    for _ in 0..instruction_length {
        instructions.push(parser.read_be_u8()?);
    }

    let num_points = end_pts_of_contours[number_of_contours as usize - 1] + 1;
    let mut flags = Vec::new();
    let mut i = 0;
    while i < num_points {
        let flag = parser.read_be_u8()?;
        flags.push(flag);
        if flag & 0x08 != 0 {
            let repeat_count = parser.read_be_u8()?;
            for _ in 0..repeat_count {
                flags.push(flag);
                i += 1;
            }
        }
        i += 1;
    }

    let mut x_coordinates = Vec::new();
    let mut y_coordinates = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for flag in &flags {
        if flag & 0x02 != 0 {
            let dx = parser.read_be_u8()?;
            x += if flag & 0x10 != 0 {
                dx as i16
            } else {
                -(dx as i16)
            };
        } else if flag & 0x10 == 0 {
            x += parser.read_be_i16()?;
        }
        x_coordinates.push(x);
    }

    for flag in &flags {
        if flag & 0x04 != 0 {
            let dy = parser.read_be_u8()?;
            y += if flag & 0x20 != 0 {
                dy as i16
            } else {
                -(dy as i16)
            };
        } else if flag & 0x20 == 0 {
            y += parser.read_be_i16()?;
        }
        y_coordinates.push(y);
    }

    Ok(GlyfSubtable {
        number_of_contours,
        x_min,
        y_min,
        x_max,
        y_max,
        end_pts_of_contours,
        instruction_length,
        instructions,
        flags,
        x_coordinates,
        y_coordinates,
    })
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
