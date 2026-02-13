#![allow(dead_code)]

const EXPECTED_MAGIC: u32 = 0xFAB11BAF;
const STRING_LITERAL_INFO_SIZE: usize = 8;
const IMAGE_DEF_SIZE: usize = 40;
const ASSEMBLY_DEF_SIZE: usize = 68;
const TYPE_DEF_SIZE: usize = 100;
const METADATA_USAGE_LIST_SIZE: usize = 8;
const METADATA_USAGE_PAIR_SIZE: usize = 8;

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }

    fn ensure(&self, n: usize) -> Result<(), String> {
        if self.pos + n > self.data.len() {
            Err(format!(
                "Read out of bounds: offset {} + {} > {}",
                self.pos,
                n,
                self.data.len()
            ))
        } else {
            Ok(())
        }
    }

    fn read_i32(&mut self) -> Result<i32, String> {
        self.ensure(4)?;
        let v = i32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(v)
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        self.ensure(4)?;
        let v = u32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(v)
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        self.ensure(2)?;
        let v = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(v)
    }

    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], String> {
        self.ensure(n)?;
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }
}

pub struct GlobalMetadataHeader {
    pub sanity: u32,
    pub version: i32,
    pub string_literal_offset: i32,
    pub string_literal_count: i32,
    pub string_literal_data_offset: i32,
    pub string_literal_data_count: i32,
    pub string_offset: i32,
    pub string_count: i32,
    pub events_offset: i32,
    pub events_count: i32,
    pub properties_offset: i32,
    pub properties_count: i32,
    pub methods_offset: i32,
    pub methods_count: i32,
    pub parameter_default_values_offset: i32,
    pub parameter_default_values_count: i32,
    pub field_default_values_offset: i32,
    pub field_default_values_count: i32,
    pub field_and_parameter_default_value_data_offset: i32,
    pub field_and_parameter_default_value_data_count: i32,
    pub field_marshaled_sizes_offset: i32,
    pub field_marshaled_sizes_count: i32,
    pub parameters_offset: i32,
    pub parameters_count: i32,
    pub fields_offset: i32,
    pub fields_count: i32,
    pub generic_parameters_offset: i32,
    pub generic_parameters_count: i32,
    pub generic_parameter_constraints_offset: i32,
    pub generic_parameter_constraints_count: i32,
    pub generic_containers_offset: i32,
    pub generic_containers_count: i32,
    pub nested_types_offset: i32,
    pub nested_types_count: i32,
    pub interfaces_offset: i32,
    pub interfaces_count: i32,
    pub vtable_methods_offset: i32,
    pub vtable_methods_count: i32,
    pub interface_offsets_offset: i32,
    pub interface_offsets_count: i32,
    pub type_definitions_offset: i32,
    pub type_definitions_count: i32,
    pub rgctx_entries_offset: i32,
    pub rgctx_entries_count: i32,
    pub images_offset: i32,
    pub images_count: i32,
    pub assemblies_offset: i32,
    pub assemblies_count: i32,
    pub metadata_usage_lists_offset: i32,
    pub metadata_usage_lists_count: i32,
    pub metadata_usage_pairs_offset: i32,
    pub metadata_usage_pairs_count: i32,
    pub field_refs_offset: i32,
    pub field_refs_count: i32,
    pub referenced_assemblies_offset: i32,
    pub referenced_assemblies_count: i32,
    pub attributes_info_offset: i32,
    pub attributes_info_count: i32,
    pub attribute_types_offset: i32,
    pub attribute_types_count: i32,
    pub unresolved_virtual_call_parameter_types_offset: i32,
    pub unresolved_virtual_call_parameter_types_count: i32,
    pub unresolved_virtual_call_parameter_ranges_offset: i32,
    pub unresolved_virtual_call_parameter_ranges_count: i32,
    pub windows_runtime_type_names_offset: i32,
    pub windows_runtime_type_names_size: i32,
    pub exported_type_definitions_offset: i32,
    pub exported_type_definitions_count: i32,
}

impl GlobalMetadataHeader {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            sanity: r.read_u32()?,
            version: r.read_i32()?,
            string_literal_offset: r.read_i32()?,
            string_literal_count: r.read_i32()?,
            string_literal_data_offset: r.read_i32()?,
            string_literal_data_count: r.read_i32()?,
            string_offset: r.read_i32()?,
            string_count: r.read_i32()?,
            events_offset: r.read_i32()?,
            events_count: r.read_i32()?,
            properties_offset: r.read_i32()?,
            properties_count: r.read_i32()?,
            methods_offset: r.read_i32()?,
            methods_count: r.read_i32()?,
            parameter_default_values_offset: r.read_i32()?,
            parameter_default_values_count: r.read_i32()?,
            field_default_values_offset: r.read_i32()?,
            field_default_values_count: r.read_i32()?,
            field_and_parameter_default_value_data_offset: r.read_i32()?,
            field_and_parameter_default_value_data_count: r.read_i32()?,
            field_marshaled_sizes_offset: r.read_i32()?,
            field_marshaled_sizes_count: r.read_i32()?,
            parameters_offset: r.read_i32()?,
            parameters_count: r.read_i32()?,
            fields_offset: r.read_i32()?,
            fields_count: r.read_i32()?,
            generic_parameters_offset: r.read_i32()?,
            generic_parameters_count: r.read_i32()?,
            generic_parameter_constraints_offset: r.read_i32()?,
            generic_parameter_constraints_count: r.read_i32()?,
            generic_containers_offset: r.read_i32()?,
            generic_containers_count: r.read_i32()?,
            nested_types_offset: r.read_i32()?,
            nested_types_count: r.read_i32()?,
            interfaces_offset: r.read_i32()?,
            interfaces_count: r.read_i32()?,
            vtable_methods_offset: r.read_i32()?,
            vtable_methods_count: r.read_i32()?,
            interface_offsets_offset: r.read_i32()?,
            interface_offsets_count: r.read_i32()?,
            type_definitions_offset: r.read_i32()?,
            type_definitions_count: r.read_i32()?,
            rgctx_entries_offset: r.read_i32()?,
            rgctx_entries_count: r.read_i32()?,
            images_offset: r.read_i32()?,
            images_count: r.read_i32()?,
            assemblies_offset: r.read_i32()?,
            assemblies_count: r.read_i32()?,
            metadata_usage_lists_offset: r.read_i32()?,
            metadata_usage_lists_count: r.read_i32()?,
            metadata_usage_pairs_offset: r.read_i32()?,
            metadata_usage_pairs_count: r.read_i32()?,
            field_refs_offset: r.read_i32()?,
            field_refs_count: r.read_i32()?,
            referenced_assemblies_offset: r.read_i32()?,
            referenced_assemblies_count: r.read_i32()?,
            attributes_info_offset: r.read_i32()?,
            attributes_info_count: r.read_i32()?,
            attribute_types_offset: r.read_i32()?,
            attribute_types_count: r.read_i32()?,
            unresolved_virtual_call_parameter_types_offset: r.read_i32()?,
            unresolved_virtual_call_parameter_types_count: r.read_i32()?,
            unresolved_virtual_call_parameter_ranges_offset: r.read_i32()?,
            unresolved_virtual_call_parameter_ranges_count: r.read_i32()?,
            windows_runtime_type_names_offset: r.read_i32()?,
            windows_runtime_type_names_size: r.read_i32()?,
            exported_type_definitions_offset: r.read_i32()?,
            exported_type_definitions_count: r.read_i32()?,
        })
    }
}

pub struct StringLiteralInfo {
    pub length: u32,
    pub offset: u32,
}

impl StringLiteralInfo {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            length: r.read_u32()?,
            offset: r.read_u32()?,
        })
    }
}

pub struct ImageDefinition {
    pub name_index: i32,
    pub assembly_index: i32,
    pub type_start: i32,
    pub type_count: u32,
    pub exported_type_start: i32,
    pub exported_type_count: u32,
    pub entry_point_index: i32,
    pub token: u32,
    pub custom_attribute_start: i32,
    pub custom_attribute_count: u32,
}

impl ImageDefinition {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            name_index: r.read_i32()?,
            assembly_index: r.read_i32()?,
            type_start: r.read_i32()?,
            type_count: r.read_u32()?,
            exported_type_start: r.read_i32()?,
            exported_type_count: r.read_u32()?,
            entry_point_index: r.read_i32()?,
            token: r.read_u32()?,
            custom_attribute_start: r.read_i32()?,
            custom_attribute_count: r.read_u32()?,
        })
    }
}

pub struct AssemblyNameDefinition {
    pub name_index: i32,
    pub culture_index: i32,
    pub hash_value_index: i32,
    pub public_key_index: i32,
    pub hash_alg: u32,
    pub hash_len: i32,
    pub flags: u32,
    pub major: i32,
    pub minor: i32,
    pub build: i32,
    pub revision: i32,
    pub public_key_token: [u8; 8],
}

impl AssemblyNameDefinition {
    fn read(r: &mut Reader) -> Result<Self, String> {
        let name_index = r.read_i32()?;
        let culture_index = r.read_i32()?;
        let hash_value_index = r.read_i32()?;
        let public_key_index = r.read_i32()?;
        let hash_alg = r.read_u32()?;
        let hash_len = r.read_i32()?;
        let flags = r.read_u32()?;
        let major = r.read_i32()?;
        let minor = r.read_i32()?;
        let build = r.read_i32()?;
        let revision = r.read_i32()?;
        let token_bytes = r.read_bytes(8)?;
        let mut public_key_token = [0u8; 8];
        public_key_token.copy_from_slice(token_bytes);
        Ok(Self {
            name_index,
            culture_index,
            hash_value_index,
            public_key_index,
            hash_alg,
            hash_len,
            flags,
            major,
            minor,
            build,
            revision,
            public_key_token,
        })
    }
}

pub struct AssemblyDefinition {
    pub image_index: i32,
    pub token: u32,
    pub referenced_assembly_start: i32,
    pub referenced_assembly_count: i32,
    pub aname: AssemblyNameDefinition,
}

impl AssemblyDefinition {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            image_index: r.read_i32()?,
            token: r.read_u32()?,
            referenced_assembly_start: r.read_i32()?,
            referenced_assembly_count: r.read_i32()?,
            aname: AssemblyNameDefinition::read(r)?,
        })
    }
}

pub struct TypeDefinition {
    pub name_index: i32,
    pub namespace_index: i32,
    pub byval_type_index: i32,
    pub byref_type_index: i32,
    pub declaring_type_index: i32,
    pub parent_index: i32,
    pub element_type_index: i32,
    pub rgctx_start_index: i32,
    pub rgctx_count: i32,
    pub generic_container_index: i32,
    pub flags: u32,
    pub field_start: i32,
    pub method_start: i32,
    pub event_start: i32,
    pub property_start: i32,
    pub nested_types_start: i32,
    pub interfaces_start: i32,
    pub vtable_start: i32,
    pub interface_offsets_start: i32,
    pub method_count: u16,
    pub property_count: u16,
    pub field_count: u16,
    pub event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    pub interfaces_count: u16,
    pub interface_offsets_count: u16,
    pub bitfield: u32,
    pub token: u32,
}

impl TypeDefinition {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            name_index: r.read_i32()?,
            namespace_index: r.read_i32()?,
            byval_type_index: r.read_i32()?,
            byref_type_index: r.read_i32()?,
            declaring_type_index: r.read_i32()?,
            parent_index: r.read_i32()?,
            element_type_index: r.read_i32()?,
            rgctx_start_index: r.read_i32()?,
            rgctx_count: r.read_i32()?,
            generic_container_index: r.read_i32()?,
            flags: r.read_u32()?,
            field_start: r.read_i32()?,
            method_start: r.read_i32()?,
            event_start: r.read_i32()?,
            property_start: r.read_i32()?,
            nested_types_start: r.read_i32()?,
            interfaces_start: r.read_i32()?,
            vtable_start: r.read_i32()?,
            interface_offsets_start: r.read_i32()?,
            method_count: r.read_u16()?,
            property_count: r.read_u16()?,
            field_count: r.read_u16()?,
            event_count: r.read_u16()?,
            nested_type_count: r.read_u16()?,
            vtable_count: r.read_u16()?,
            interfaces_count: r.read_u16()?,
            interface_offsets_count: r.read_u16()?,
            bitfield: r.read_u32()?,
            token: r.read_u32()?,
        })
    }
}

pub struct MetadataUsageList {
    pub start: u32,
    pub count: u32,
}

impl MetadataUsageList {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            start: r.read_u32()?,
            count: r.read_u32()?,
        })
    }
}

pub struct MetadataUsagePair {
    pub destination_index: u32,
    pub encoded_source_index: u32,
}

impl MetadataUsagePair {
    fn read(r: &mut Reader) -> Result<Self, String> {
        Ok(Self {
            destination_index: r.read_u32()?,
            encoded_source_index: r.read_u32()?,
        })
    }
}

pub struct GlobalMetadata {
    pub header: GlobalMetadataHeader,
    pub string_literals: Vec<String>,
    pub images: Vec<ImageDefinition>,
    pub assemblies: Vec<AssemblyDefinition>,
    pub type_definitions: Vec<TypeDefinition>,
    pub metadata_usage_lists: Vec<MetadataUsageList>,
    pub metadata_usage_pairs: Vec<MetadataUsagePair>,
    pub string_literal_infos: Vec<StringLiteralInfo>,
    string_data: Vec<u8>,
}

impl GlobalMetadata {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut r = Reader::new(data, 0);
        let header = GlobalMetadataHeader::read(&mut r)?;

        let str_start = header.string_offset as usize;
        let str_end = str_start + header.string_count as usize;
        let string_data = data
            .get(str_start..str_end)
            .ok_or("String data section out of bounds")?
            .to_vec();

        let info_count = header.string_literal_count as usize / STRING_LITERAL_INFO_SIZE;
        let mut r = Reader::new(data, header.string_literal_offset as usize);
        let mut literal_infos = Vec::with_capacity(info_count);
        for _ in 0..info_count {
            literal_infos.push(StringLiteralInfo::read(&mut r)?);
        }

        let lit_data_start = header.string_literal_data_offset as usize;
        let lit_data_end = lit_data_start + header.string_literal_data_count as usize;
        let lit_data = data
            .get(lit_data_start..lit_data_end)
            .ok_or("String literal data section out of bounds")?;
        let mut string_literals = Vec::with_capacity(info_count);
        for info in &literal_infos {
            let start = info.offset as usize;
            let end = start + info.length as usize;
            if end <= lit_data.len() {
                string_literals.push(String::from_utf8_lossy(&lit_data[start..end]).into_owned());
            } else {
                string_literals.push(String::new());
            }
        }

        let img_count = header.images_count as usize / IMAGE_DEF_SIZE;
        let mut r = Reader::new(data, header.images_offset as usize);
        let mut images = Vec::with_capacity(img_count);
        for _ in 0..img_count {
            images.push(ImageDefinition::read(&mut r)?);
        }

        let asm_count = header.assemblies_count as usize / ASSEMBLY_DEF_SIZE;
        let mut r = Reader::new(data, header.assemblies_offset as usize);
        let mut assemblies = Vec::with_capacity(asm_count);
        for _ in 0..asm_count {
            assemblies.push(AssemblyDefinition::read(&mut r)?);
        }

        let type_count = header.type_definitions_count as usize / TYPE_DEF_SIZE;
        let mut r = Reader::new(data, header.type_definitions_offset as usize);
        let mut type_definitions = Vec::with_capacity(type_count);
        for _ in 0..type_count {
            type_definitions.push(TypeDefinition::read(&mut r)?);
        }

        let list_count = header.metadata_usage_lists_count as usize / METADATA_USAGE_LIST_SIZE;
        let mut r = Reader::new(data, header.metadata_usage_lists_offset as usize);
        let mut metadata_usage_lists = Vec::with_capacity(list_count);
        for _ in 0..list_count {
            metadata_usage_lists.push(MetadataUsageList::read(&mut r)?);
        }

        let pair_count = header.metadata_usage_pairs_count as usize / METADATA_USAGE_PAIR_SIZE;
        let mut r = Reader::new(data, header.metadata_usage_pairs_offset as usize);
        let mut metadata_usage_pairs = Vec::with_capacity(pair_count);
        for _ in 0..pair_count {
            metadata_usage_pairs.push(MetadataUsagePair::read(&mut r)?);
        }

        Ok(Self {
            header,
            string_literals,
            images,
            assemblies,
            type_definitions,
            metadata_usage_lists,
            metadata_usage_pairs,
            string_literal_infos: literal_infos,
            string_data,
        })
    }

    pub fn is_valid(&self) -> bool {
        self.header.sanity == EXPECTED_MAGIC
    }

    pub fn magic_bytes(&self) -> [u8; 4] {
        self.header.sanity.to_le_bytes()
    }

    pub fn get_string(&self, index: i32) -> Option<&str> {
        if index < 0 || index as usize >= self.string_data.len() {
            return None;
        }
        let start = index as usize;
        let end = self.string_data[start..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| start + p)
            .unwrap_or(self.string_data.len());
        std::str::from_utf8(&self.string_data[start..end]).ok()
    }
}

pub fn decrypt_string_literals(metadata: &GlobalMetadata, data: &mut [u8]) -> Result<(), String> {
    let lit_data_start = metadata.header.string_literal_data_offset as usize;
    
    for info in &metadata.string_literal_infos {
        let start = lit_data_start + info.offset as usize;
        let end = start + info.length as usize;
        
        if let Some(slice) = data.get_mut(start..end) {
            let xor_key = (info.length as u8) ^ 0x2E;
            for byte in slice {
                *byte ^= xor_key;
            }
        }
    }

    Ok(())
}
