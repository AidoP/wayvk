use ash::vk;
use fontdue::{Font, FontSettings, layout::{GlyphRasterConfig, Layout, LayoutSettings, TextStyle, WrapStyle}};

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[repr(C)]
struct Vulkan {
    _opaque: [u8; 0]
}
impl Deref for Vulkan {
    type Target = Ft;
    fn deref(&self) -> &Ft {
        // Ensure Font* is the first parameter in struct vk
        unsafe {
            &*(self as *const _ as *const Ft)
        }
    }
}
impl DerefMut for Vulkan {
    fn deref_mut(&mut self) -> &mut Ft {
        unsafe {
            &mut *(self as *mut _ as *mut Ft)
        }
    }
}

#[repr(C)]
struct StagingBuffer {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    memory_requirements: vk::MemoryRequirements,
    buffer_len: u32
}

#[repr(C)]
struct Glyph {
    image: vk::Image,
    view: vk::ImageView,
    sampler: vk::Sampler,
    memory: vk::DeviceMemory,
    memory_requirements: vk::MemoryRequirements,
    descriptor: vk::DescriptorSet
}

#[repr(C)]
struct GlyphPushConstant {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

extern "C" {
    fn vk_staging_buffer_create(vk: *mut Vulkan, data: *const u8, data_len: usize) -> StagingBuffer;
    fn vk_staging_buffer_destroy(vk: *mut Vulkan, staging: *mut StagingBuffer);
    fn vk_staging_buffer_start_transfer(vk: *mut Vulkan) -> vk::CommandBuffer;
    fn vk_staging_buffer_end_transfer(vk: *mut Vulkan, transfer_buffer: vk::CommandBuffer);

    fn vk_create_glyph(vk: *mut Vulkan, staging: *mut StagingBuffer, transfer_buffer: vk::CommandBuffer, width: u32, height: u32) -> Glyph;
    fn vk_destroy_glyph(vk: *mut Vulkan, glyph: *mut Glyph);
    fn vk_draw_glyph(vk: *mut Vulkan, glyph: *mut Glyph, layout: GlyphPushConstant, image_index: u32);
}

#[repr(C)]
struct Ft {
    glyphs: Box<HashMap<GlyphRasterConfig, Glyph>>,
    font: Box<Font>
}

const FONT_CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '`', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=',
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+',
    '[', ']', '{', '}', '\\', '|', ';', ':', '\'', '"', ',', '<', '.', '>', '/', '?'
];
const FONT_SIZE: f32 = 12.0;

#[no_mangle]
extern "C" fn ft_load() -> Ft {
    let mut settings = FontSettings::default();
    settings.scale = FONT_SIZE;

    let raw_font = include_bytes!("/usr/share/fonts/noto/NotoSans-Regular.ttf") as &[u8];
    let font = Box::new(Font::from_bytes(raw_font, settings).unwrap());

    Ft {
        glyphs: Box::new(HashMap::new()),
        font
    }
}

#[no_mangle]
extern "C" fn ft_unload(mut ft: Ft, vk: *mut Vulkan) {
    // font gets dropped here with RAII
    for (_, glyph) in ft.glyphs.iter_mut() {
        unsafe { vk_destroy_glyph(vk, glyph) }
    }
}

#[no_mangle]
extern "C" fn ft_raster(ft: &mut Ft, vk: *mut Vulkan) {
    let mut staging_buffers = Vec::new();
    for &character in FONT_CHARS {
        let (metrics, bitmap) = ft.font.rasterize(character, FONT_SIZE);
        staging_buffers.push((character, metrics, unsafe { vk_staging_buffer_create(vk, bitmap.as_ptr(), bitmap.len()) }));
    }
    let transfer_buffer = unsafe { vk_staging_buffer_start_transfer(vk) };
    for (character, metrics, buffer) in staging_buffers.iter_mut() {
        unsafe {
            ft.glyphs.insert(
                GlyphRasterConfig {
                    c: *character,
                    px: FONT_SIZE,
                    font_index: 0
                },
                vk_create_glyph(vk, buffer, transfer_buffer, metrics.width as _, metrics.height as _)
            );
        }
    }
    unsafe { vk_staging_buffer_end_transfer(vk, transfer_buffer) }
    for (_, _, buffer) in staging_buffers.iter_mut() {
        unsafe {
            vk_staging_buffer_destroy(vk, buffer);
        }
    }
    staging_buffers.clear();
}

#[no_mangle]
extern "C" fn ft_get_character<'a>(font: &mut Ft, character: u8) -> *mut Glyph {
    let character = character as _;
    let glyph = GlyphRasterConfig {
        c: character,
        px: FONT_SIZE,
        font_index: 0
    };
    if let Some(glyph_data) = font.glyphs.get_mut(&glyph) {
        glyph_data
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
extern "C" fn ft_draw_string(vk: &mut Vulkan, string: *const u8, string_len: usize, image_index: u32) {
    let mut layout = Layout::new();
    let settings = LayoutSettings {
        include_whitespace: false,
        wrap_style: WrapStyle::Letter,
        max_width: Some(1920.0),
        max_height: Some(1080.0),
        ..Default::default()
    };
    let mut output = Vec::new();
    let fonts = &[vk.font.as_ref()];
    let text = &[
        &TextStyle::new(std::str::from_utf8(unsafe { std::slice::from_raw_parts(string, string_len) }).unwrap(), FONT_SIZE, 0)
    ];
    layout.layout_horizontal(fonts, text, &settings, &mut output);
    for glyph in output {
        unsafe {
            vk_draw_glyph(vk, vk.glyphs.get_mut(&glyph.key).expect("Character has not been rasterized"), GlyphPushConstant {
                x: glyph.x,
                y: glyph.y,
                width: glyph.width as _,
                height: glyph.height as _
            }, image_index);
        }
    }
}

#[no_mangle]
extern "C" fn ft_glyph_count(ft: &mut Ft) -> usize {
    ft.glyphs.len()
}