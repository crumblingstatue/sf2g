//! Implementation derived from SFML C++ code under zlib license.

use {
    super::{Glyph, PrimitiveType, RenderStates, RenderTarget, Vertex},
    crate::{
        graphics::{Color, FloatRect, Font, TextStyle, Transform, Transformable},
        system::Vector2f,
    },
    core::f32,
};

/// Graphical text
///
/// Text is a drawable type that allows to easily
/// display some text with custom style and color on a render target.
///
/// __Note:__
/// Currently, it is not feasible to store text long term.
/// A common pattern with rust-sfml is to create a `Text` right before you start drawing,
/// and draw all the text you want with it. You can change its properties using
/// `set_font`, `set_position`, `set_string`, etc., before drawing it, as many times as you need
/// to.
#[derive(Debug)]
pub struct Text<'s> {
    string: String,
    font: Option<&'s Font>,
    character_size: u32,
    letter_spacing_factor: f32,
    line_spacing_factor: f32,
    style: TextStyle,
    fill_color: Color,
    outline_color: Color,
    outline_thickness: f32,
    vertices: Vec<Vertex>,
    outline_vertices: Vec<Vertex>,
    bounds: FloatRect,
    geometry_need_update: bool,
    _font_texture_id: u64, // TODO
    origin: Vector2f,
    position: Vector2f,
    rotation: f32,
    scale: Vector2f,
    transform: Transform,
    transform_need_update: bool,
    inverse_transform: Transform,
    _inverse_transform_need_update: bool, // TODO
}

// Add an underline or strikethrough line to the vertex array
fn add_line(
    vertices: &mut Vec<Vertex>,
    line_length: f32,
    line_top: f32,
    color: Color,
    offset: f32,
    thickness: f32,
    outline_thickness: f32,
) {
    let top = f32::floor(line_top + offset - (thickness / 2.0) + 0.5);
    let bottom = top + f32::floor(thickness + 0.5);
    vertices.extend_from_slice(&[
        Vertex {
            position: Vector2f::new(-outline_thickness, top - outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vector2f::new(line_length + outline_thickness, top - outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vector2f::new(-outline_thickness, bottom + outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vector2f::new(-outline_thickness, bottom + outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vector2f::new(line_length + outline_thickness, top - outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
        Vertex {
            position: Vector2f::new(line_length + outline_thickness, bottom + outline_thickness),
            color,
            tex_coords: Vector2f::new(1.0, 1.0),
        },
    ]);
    /*
    vertices.append(sf::Vertex(sf::Vector2f(-outlineThickness,             top    - outlineThickness), color, sf::Vector2f(1, 1)));
    vertices.append(sf::Vertex(sf::Vector2f(lineLength + outlineThickness, top    - outlineThickness), color, sf::Vector2f(1, 1)));
    vertices.append(sf::Vertex(sf::Vector2f(-outlineThickness,             bottom + outlineThickness), color, sf::Vector2f(1, 1)));
    vertices.append(sf::Vertex(sf::Vector2f(-outlineThickness,             bottom + outlineThickness), color, sf::Vector2f(1, 1)));
    vertices.append(sf::Vertex(sf::Vector2f(lineLength + outlineThickness, top    - outlineThickness), color, sf::Vector2f(1, 1)));
    vertices.append(sf::Vertex(sf::Vector2f(lineLength + outlineThickness, bottom + outlineThickness), color, sf::Vector2f(1, 1)));*/
}

// Add a glyph quad to the vertex array
fn add_glyph_quad(
    vertices: &mut Vec<Vertex>,
    position: Vector2f,
    color: Color,
    glyph: Glyph,
    italic_shear: f32,
) {
    let padding = 1.0;
    let left = glyph.bounds().left - padding;
    let top = glyph.bounds().top - padding;
    let right = glyph.bounds().left + glyph.bounds().width + padding;
    let bottom = glyph.bounds().top + glyph.bounds().height + padding;

    let u1 = glyph.texture_rect().left as f32 - padding;
    let v1 = glyph.texture_rect().top as f32 - padding;
    let u2 = (glyph.texture_rect().left + glyph.texture_rect().width) as f32 + padding;
    let v2 = (glyph.texture_rect().top + glyph.texture_rect().height) as f32 + padding;
    vertices.extend_from_slice(&[
        Vertex {
            position: Vector2f::new(position.x + left - italic_shear * top, position.y + top),
            color,
            tex_coords: Vector2f::new(u1, v1),
        },
        Vertex {
            position: Vector2f::new(position.x + right - italic_shear * top, position.y + top),
            color,
            tex_coords: Vector2f::new(u2, v1),
        },
        Vertex {
            position: Vector2f::new(
                position.x + left - italic_shear * bottom,
                position.y + bottom,
            ),
            color,
            tex_coords: Vector2f::new(u1, v2),
        },
        Vertex {
            position: Vector2f::new(
                position.x + left - italic_shear * bottom,
                position.y + bottom,
            ),
            color,
            tex_coords: Vector2f::new(u1, v2),
        },
        Vertex {
            position: Vector2f::new(position.x + right - italic_shear * top, position.y + top),
            color,
            tex_coords: Vector2f::new(u2, v1),
        },
        Vertex {
            position: Vector2f::new(
                position.x + right - italic_shear * bottom,
                position.y + bottom,
            ),
            color,
            tex_coords: Vector2f::new(u2, v2),
        },
    ]);
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self {
            string: Default::default(),
            font: Default::default(),
            character_size: 30,
            letter_spacing_factor: 1.0,
            line_spacing_factor: 1.0,
            style: TextStyle::REGULAR,
            fill_color: Color::rgb(255, 255, 255),
            outline_color: Color::rgb(0, 0, 0),
            outline_thickness: 0.0,
            vertices: Default::default(),
            outline_vertices: Default::default(),
            bounds: Default::default(),
            geometry_need_update: Default::default(),
            _font_texture_id: Default::default(),
            origin: Default::default(),
            position: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
            transform: Default::default(),
            transform_need_update: Default::default(),
            inverse_transform: Default::default(),
            _inverse_transform_need_update: Default::default(),
        }
    }
}

impl<'s> Text<'s> {
    /// Create a new text with initialized value
    ///
    /// Default value for characterSize on SFML is 30.
    ///
    /// # Arguments
    /// * string - The string of the text
    /// * font - The font to display the Text
    /// * characterSize - The size of the Text
    #[must_use]
    pub fn new(string: String, font: &'s Font, character_size: u32) -> Text<'s> {
        let mut text = Text::default();
        text.set_string(string);
        text.set_font(font);
        text.set_character_size(character_size);
        text
    }

    /// Set the string of a text
    ///
    /// A text's string is empty by default.
    ///
    /// # Arguments
    /// * string - New string
    pub fn set_string(&mut self, string: String) {
        if self.string != string {
            self.string = string;
            self.geometry_need_update = true;
        }
    }

    /// Get the string of a text
    #[must_use]
    pub fn string(&self) -> &str {
        &self.string
    }

    /// Get the size of the characters
    ///
    /// Return the size of the characters
    #[must_use]
    pub fn character_size(&self) -> u32 {
        todo!()
    }

    /// Set the font of the text
    ///
    /// The font argument refers to a texture that must
    /// exist as long as the text uses it. Indeed, the text
    /// doesn't store its own copy of the font, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the font is destroyed and the text tries to
    /// use it, the behaviour is undefined.
    ///
    /// font - New font
    pub fn set_font(&mut self, font: &'s Font) {
        self.font = Some(font);
    }

    /// Set the style of a text
    ///
    /// You can pass a combination of one or more styles, for
    /// example Bold | Italic.
    /// The default style is Regular.
    ///
    /// # Arguments
    /// * style - New style
    pub fn set_style(&mut self, style: TextStyle) {
        self.style = style;
    }

    /// Set the size of the characters of a text
    ///
    /// The default size is 30.
    ///
    /// # Arguments
    /// * size - The new character size, in pixels
    pub fn set_character_size(&mut self, size: u32) {
        self.character_size = size;
    }

    /// Get the style of a text
    ///
    /// Return the current string style (see Style enum)
    #[must_use]
    pub fn style(&self) -> TextStyle {
        self.style
    }

    /// Get the font of a text
    /// If the text has no font attached, a None is returned.
    /// The returned pointer is const, which means that you can't
    /// modify the font when you retrieve it with this function.
    #[must_use]
    pub fn font(&self) -> Option<&'s Font> {
        self.font
    }

    /// Set the fill color of the text.
    ///
    /// By default, the text's fill color is opaque white. Setting the fill color to a transparent
    /// color with an outline will cause the outline to be displayed in the fill area of the text.
    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
    }

    /// Set the outline color of the text.
    ///
    /// By default, the text's outline color is opaque black.
    pub fn set_outline_color(&mut self, color: Color) {
        self.outline_color = color;
    }

    /// Set the thickness of the text's outline.
    ///
    /// By default, the outline thickness is 0.
    ///
    /// Be aware that using a negative value for the outline thickness will cause distorted
    /// rendering.
    pub fn set_outline_thickness(&mut self, thickness: f32) {
        self.outline_thickness = thickness;
    }

    /// Returns the fill color of the text.
    #[must_use]
    pub fn fill_color(&self) -> Color {
        self.fill_color
    }

    /// Returns the outline color of the text.
    #[must_use]
    pub fn outline_color(&self) -> Color {
        self.outline_color
    }

    /// Returns the outline thickness of the text, in pixels.
    #[must_use]
    pub fn outline_thickness(&self) -> f32 {
        self.outline_thickness
    }

    /// Return the position of the index-th character in a text
    ///
    /// This function computes the visual position of a character
    /// from its index in the string. The returned position is
    /// in global coordinates (translation, rotation, scale and
    /// origin are applied).
    /// If index is out of range, the position of the end of
    /// the string is returned.
    ///
    /// # Arguments
    /// * index - The index of the character
    ///
    /// Return the position of the character
    #[must_use]
    pub fn find_character_pos(&mut self, index: usize) -> Vector2f {
        let Some(font) = &self.font else {
            return Vector2f::default();
        };
        let is_bold = self.style.contains(TextStyle::BOLD);
        let mut ws_width = font
            .glyph(' ' as u32, self.character_size, is_bold, 0.0)
            .advance();
        let letter_spacing = (ws_width / 3.0) * (self.letter_spacing_factor - 1.0);
        ws_width += letter_spacing;
        let line_spacing = font.line_spacing(self.character_size) * self.line_spacing_factor;
        let mut pos = Vector2f::default();
        let mut prev_ch = 0;
        for ch in self.string.chars().take(index) {
            let kerning = if is_bold {
                font.bold_kerning(prev_ch, ch as u32, self.character_size)
            } else {
                font.kerning(prev_ch, ch as u32, self.character_size)
            };
            pos.x += kerning;
            prev_ch = ch as u32;
            match ch {
                ' ' => pos.x += ws_width,
                '\t' => pos.x += ws_width * 4.0,
                '\n' => {
                    pos.y += line_spacing;
                    pos.x = 0.0;
                }
                _ => {}
            }
            pos.x += font
                .glyph(ch as u32, self.character_size, is_bold, 0.0)
                .advance()
                + letter_spacing;
        }
        self.get_transform().transform_point(pos)
    }

    fn get_transform(&mut self) -> Transform {
        if self.transform_need_update {
            let angle = -self.rotation * std::f32::consts::PI / 180.0;
            let cosine = angle.cos();
            let sine = angle.sin();
            let sxc = self.scale.x * cosine;
            let syc = self.scale.y * cosine;
            let sxs = self.scale.x * sine;
            let sys = self.scale.y * sine;
            let tx = -self.origin.x * sxc - self.origin.y * sys + self.position.x;
            let ty = self.origin.x * sxs - self.origin.y * syc + self.position.y;
            self.transform = Transform::new(sxc, sys, tx, -sxs, syc, ty, 0.0, 0.0, 1.0);
            self.transform_need_update = false;
        }
        self.transform
    }

    /// Get the local bounding rectangle of a text
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        todo!()
    }

    /// Get the global bounding rectangle of a text
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// text in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        todo!()
    }
    /// Get the size of the line spacing factor.
    #[must_use]
    pub fn line_spacing(&self) -> f32 {
        todo!()
    }
    /// Set the line spacing factor.
    ///
    /// The default spacing between lines is defined by the font.
    /// This method enables you to set a factor for the spacing between lines.
    /// By default the line spacing factor is 1.
    pub fn set_line_spacing(&mut self, factor: f32) {
        self.line_spacing_factor = factor;
    }
    /// Get the size of the letter spacing factor.
    #[must_use]
    pub fn letter_spacing(&self) -> f32 {
        self.letter_spacing_factor
    }
    /// Set the letter spacing factor.
    ///
    /// The default spacing between letters is defined by the font.
    /// This factor doesn't directly apply to the existing spacing between each character,
    /// it rather adds a fixed space between them which is calculated from the font metrics and
    /// the character size. Note that factors below 1 (including negative numbers) bring
    /// characters closer to each other. By default the letter spacing factor is 1.
    pub fn set_letter_spacing(&mut self, factor: f32) {
        self.letter_spacing_factor = factor;
    }
    pub(crate) fn draw<RT: RenderTarget>(&mut self, rt: &mut RT, rs: &RenderStates) {
        let mut rs = *rs;
        self.ensure_geometry_update(); // TODO: Place inside if-let (borrow conflict)
        if let Some(font) = &self.font {
            rs.transform.combine(&self.transform);
            rs.transform.translate(self.position.x, self.position.y);
            rs.texture = Some(font.texture(self.character_size));
            if self.outline_thickness != 0.0 {
                rt.draw_primitives(&self.outline_vertices, PrimitiveType::TRIANGLES, &rs);
            }
            rt.draw_primitives(&self.vertices, PrimitiveType::TRIANGLES, &rs);
        }
    }
    fn ensure_geometry_update(&mut self) {
        let Some(font) = self.font else { return };

        /*// Do nothing, if geometry has not changed and the font texture has not changed
        if (!m_geometryNeedUpdate && m_font->getTexture(m_characterSize).m_cacheId == m_fontTextureId)
            return;*/
        // <- TODO (m_cacheId)
        // Save the current fonts texture id
        //m_fontTextureId = m_font->getTexture(m_characterSize).m_cacheId; <- TODO as well
        self.geometry_need_update = false;
        self.vertices.clear();
        self.outline_vertices.clear();
        self.bounds = FloatRect::default();
        if self.string.is_empty() {
            return;
        }
        let is_bold = self.style.contains(TextStyle::BOLD);
        let is_underlined = self.style.contains(TextStyle::UNDERLINED);
        let is_strike_through = self.style.contains(TextStyle::STRIKETHROUGH);
        let italic_shear = if self.style.contains(TextStyle::ITALIC) {
            0.209
        } else {
            0.0
        };
        let underline_offset = font.underline_position(self.character_size);
        let underline_thickness = font.underline_thickness(self.character_size);
        // Compute the location of the strike through dynamically
        // We use the center point of the lowercase 'x' glyph as the reference
        // We reuse the underline thickness as the thickness of the strike through as well
        let x_bounds = font
            .glyph('x' as u32, self.character_size, is_bold, 0.0)
            .bounds();
        let strikethrough_offset = x_bounds.top + x_bounds.height / 2.0;
        let mut ws_width = font
            .glyph(' ' as u32, self.character_size, is_bold, 0.0)
            .advance();
        let letter_spacing = (ws_width / 3.0) * (self.letter_spacing_factor - 1.0);
        ws_width += letter_spacing;
        let line_spacing = font.line_spacing(self.character_size) * self.line_spacing_factor;
        let mut x = 0.0;
        let mut y = self.character_size as f32;
        let mut min_x = self.character_size as f32;
        let mut min_y = self.character_size as f32;
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        let mut prev_char = 0;
        for ch in self.string.chars() {
            if ch == '\r' {
                continue;
            }
            let kerning = if is_bold {
                font.bold_kerning(prev_char, ch as u32, self.character_size)
            } else {
                font.kerning(prev_char, ch as u32, self.character_size)
            };
            x += kerning;
            if is_underlined && (ch == '\n' && prev_char != '\n' as u32) {
                add_line(
                    &mut self.vertices,
                    x,
                    y,
                    self.fill_color,
                    underline_offset,
                    underline_thickness,
                    0.0,
                );

                if self.outline_thickness != 0.0 {
                    add_line(
                        &mut self.outline_vertices,
                        x,
                        y,
                        self.outline_color,
                        underline_offset,
                        underline_thickness,
                        self.outline_thickness,
                    );
                }
            }
            if is_strike_through && (ch == '\n' && prev_char != '\n' as u32) {
                add_line(
                    &mut self.vertices,
                    x,
                    y,
                    self.fill_color,
                    strikethrough_offset,
                    underline_thickness,
                    0.0,
                );

                if self.outline_thickness != 0.0 {
                    add_line(
                        &mut self.outline_vertices,
                        x,
                        y,
                        self.outline_color,
                        strikethrough_offset,
                        underline_thickness,
                        self.outline_thickness,
                    );
                }
            }
            prev_char = ch as u32;
            if ch == ' ' || ch == '\n' || ch == '\t' {
                min_x = f32::min(min_x, x);
                min_y = f32::min(min_y, y);

                match ch {
                    ' ' => x += ws_width,
                    '\t' => x += ws_width * 4.0,
                    '\n' => {
                        y += line_spacing;
                        x = 0.0;
                    }
                    _ => {}
                }

                max_x = f32::max(max_x, x);
                max_y = f32::max(max_y, y);
                continue;
            }
            if self.outline_thickness != 0.0 {
                let glyph = font.glyph(
                    ch as u32,
                    self.character_size,
                    is_bold,
                    self.outline_thickness,
                );
                add_glyph_quad(
                    &mut self.outline_vertices,
                    Vector2f::new(x, y),
                    self.outline_color,
                    glyph,
                    italic_shear,
                );
            }
            let glyph = font.glyph(ch as u32, self.character_size, is_bold, 0.0);
            add_glyph_quad(
                &mut self.vertices,
                Vector2f::new(x, y),
                self.fill_color,
                glyph,
                italic_shear,
            );
            let left = glyph.bounds().left;
            let top = glyph.bounds().top;
            let right = glyph.bounds().left + glyph.bounds().width;
            let bottom = glyph.bounds().top + glyph.bounds().height;
            min_x = f32::min(min_x, x + left - italic_shear * bottom);
            max_x = f32::max(max_x, right - italic_shear * top);
            min_y = f32::min(min_y, y + top);
            max_y = f32::max(max_y, y + bottom);
            x += glyph.advance() + letter_spacing;
        }
        if self.outline_thickness != 0.0 {
            let outline = f32::abs(f32::ceil(self.outline_thickness));
            min_x -= outline;
            max_x += outline;
            min_y -= outline;
            max_y += outline;
        }
        if is_underlined && (x > 0.0) {
            add_line(
                &mut self.vertices,
                x,
                y,
                self.fill_color,
                underline_offset,
                underline_thickness,
                0.0,
            );

            if self.outline_thickness != 0.0 {
                add_line(
                    &mut self.outline_vertices,
                    x,
                    y,
                    self.outline_color,
                    underline_offset,
                    underline_thickness,
                    self.outline_thickness,
                );
            }
        }
        if is_strike_through && (x > 0.0) {
            add_line(
                &mut self.vertices,
                x,
                y,
                self.fill_color,
                strikethrough_offset,
                underline_thickness,
                0.0,
            );
            if self.outline_thickness != 0.0 {
                add_line(
                    &mut self.outline_vertices,
                    x,
                    y,
                    self.outline_color,
                    strikethrough_offset,
                    underline_thickness,
                    self.outline_thickness,
                );
            }
        }
        self.bounds.left = min_x;
        self.bounds.top = min_y;
        self.bounds.width = max_x - min_x;
        self.bounds.height = max_y - min_y;
    }
}

impl<'s> Clone for Text<'s> {
    /// Return a new Text or panic! if there is not enough memory
    fn clone(&self) -> Text<'s> {
        todo!()
    }
}

impl Transformable for Text<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        self.position = position.into();
    }
    fn set_rotation(&mut self, angle: f32) {
        self.rotation = angle;
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        self.scale = scale.into();
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.origin = origin.into();
    }
    fn position(&self) -> Vector2f {
        self.position
    }
    fn rotation(&self) -> f32 {
        self.rotation
    }
    fn get_scale(&self) -> Vector2f {
        self.scale
    }
    fn origin(&self) -> Vector2f {
        self.origin
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        let offset = offset.into();
        self.transform.translate(offset.x, offset.y);
    }
    fn rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        self.scale = factors.into();
    }
    fn transform(&self) -> &Transform {
        &self.transform
    }
    fn inverse_transform(&self) -> &Transform {
        &self.inverse_transform
    }
}
