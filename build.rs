use shaderc;
use std::{io::{Read, Write}, fs::File};

fn main() {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_source_language(shaderc::SourceLanguage::HLSL);
    for entry in std::fs::read_dir("src/shader/").unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().map(|s| s.to_str()).flatten() == Some("hlsl") {
            let mut shader = String::new();
            let mut file = File::open(entry.path()).unwrap();
            file.read_to_string(&mut shader).unwrap();
            let entry = entry.path();
            let input_file = entry.to_str().unwrap_or("unknown");
            let frag = compiler.compile_into_spirv(&shader, shaderc::ShaderKind::Fragment, input_file, "frag", Some(&options)).unwrap();
            let vert = compiler.compile_into_spirv(&shader, shaderc::ShaderKind::Vertex, input_file, "vert", Some(&options)).unwrap();

            let mut frag_out = File::create(entry.with_extension("frag.spv")).unwrap();
            frag_out.write_all(frag.as_binary_u8()).unwrap();
            let mut vert_out = File::create(entry.with_extension("vert.spv")).unwrap();
            vert_out.write_all(vert.as_binary_u8()).unwrap();
        }
    }
}