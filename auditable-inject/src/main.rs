// Shamelessly copied from rustc codebase:
// https://github.com/rust-lang/rust/blob/e100ec5bc7cd768ec17d75448b29c9ab4a39272b/compiler/rustc_codegen_ssa/src/back/metadata.rs#L233-L287

use object::write::{self, StandardSegment, Symbol, SymbolSection};
use object::{
    elf, Architecture, BinaryFormat, Endianness, FileFlags, Object, ObjectSection, SectionFlags,
    SectionKind, SymbolFlags, SymbolKind, SymbolScope,
};

pub fn create_compressed_metadata_file(
    format: &FormatDescription,
    compressed: &[u8],
    symbol_name: &str,
) -> Vec<u8> {
    let mut file = if let Some(file) = create_object_file(format) {
        file
    } else {
        // FIXME: this was copied from rustc codebase but looks sketchy
        return compressed.to_vec();
    };
    let section = file.add_section(
        file.segment_name(StandardSegment::Data).to_vec(),
        b".rustc".to_vec(),
        SectionKind::ReadOnlyData,
    );
    match file.format() {
        BinaryFormat::Elf => {
            // Explicitly set no flags to avoid SHF_ALLOC default for data section.
            file.section_mut(section).flags = SectionFlags::Elf { sh_flags: 0 };
        }
        _ => {}
    };
    let offset = file.append_section_data(section, &compressed, 1);

    // For MachO and probably PE this is necessary to prevent the linker from throwing away the
    // .rustc section. For ELF this isn't necessary, but it also doesn't harm.
    file.add_symbol(Symbol {
        name: symbol_name.as_bytes().to_vec(),
        value: offset,
        size: compressed.len() as u64,
        kind: SymbolKind::Data,
        scope: SymbolScope::Dynamic,
        weak: false,
        section: SymbolSection::Section(section),
        flags: SymbolFlags::None,
    });

    file.write().unwrap()
}

fn create_object_file(f: &FormatDescription) -> Option<write::Object<'static>> {
    // The equivalent function inside rustc contains spooky special-casing for MIPS and RISC-V:
    // https://github.com/rust-lang/rust/blob/03a8cc7df1d65554a4d40825b0490c93ac0f0236/compiler/rustc_codegen_ssa/src/back/metadata.rs#L133-L165
    // I am ignoring that in the prototype for now.
    // To get this into Cargo, presumably we will need a way to share that code between rustc and Cargo.
    // -- Shnatsel
    let mut file = write::Object::new(f.format, f.architecture, f.endian);
    Some(file)
}

pub struct FormatDescription {
    format: BinaryFormat,
    architecture: Architecture,
    endian: Endianness
}

fn guess_format() -> FormatDescription {
    todo!();
}

fn main() {
    println!("Hello, world!");
}
