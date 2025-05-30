use sf2g::{
    SfResult,
    graphics::{
        Color, Font, Rect, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::Vector2,
    window::{ContextSettings, Cursor, CursorType, Event, Style, mouse},
};

include!("../example_common.rs");

const DRAW_AREA_TOPLEFT: (u16, u16) = (300, 64);
const DRAW_GRID_WH: u8 = 16;
const DRAW_CELL_WH: u8 = 26;

fn gridindex(
    grid: &mut [bool; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize],
    x: usize,
    y: usize,
) -> Option<&mut bool> {
    grid.get_mut(y * DRAW_GRID_WH as usize + x)
}

fn mouse_over(rect: &Rect<i32>, mouse_x: i32, mouse_y: i32) -> bool {
    rect.contains(Vector2::new(mouse_x, mouse_y))
}

enum ButtonStyle {
    Normal,
    Highlighted,
    Selected,
    Error,
}

fn draw_button(
    rect: &Rect<i32>,
    shape: &mut RectangleShape,
    text: &mut Text,
    string: &str,
    render_window: &mut RenderWindow,
    style: ButtonStyle,
) {
    shape.set_position((rect.left as f32, rect.top as f32));
    shape.set_size((rect.width as f32, rect.height as f32));
    let (rect_fill, rect_outline, text_fill) = match style {
        ButtonStyle::Normal => (Color::TRANSPARENT, Color::WHITE, Color::WHITE),
        ButtonStyle::Highlighted => (Color::WHITE, Color::WHITE, Color::BLACK),
        ButtonStyle::Selected => (Color::GREEN, Color::GREEN, Color::BLACK),
        ButtonStyle::Error => (Color::RED, Color::BLACK, Color::BLACK),
    };
    shape.set_outline_color(rect_outline);
    shape.set_fill_color(rect_fill);
    text.tf.position = [rect.left as f32 + 12.0, rect.top as f32 + 8.0];
    text.set_fill_color(text_fill);
    text.set_string(string.to_owned());
    render_window.draw_rectangle_shape(shape, &RenderStates::DEFAULT);
    text.draw(render_window, &RenderStates::DEFAULT);
}

fn bstyle(highlighted: bool, selected: bool, error: bool) -> ButtonStyle {
    if error {
        ButtonStyle::Error
    } else if highlighted {
        ButtonStyle::Highlighted
    } else if selected {
        ButtonStyle::Selected
    } else {
        ButtonStyle::Normal
    }
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut cursor = Cursor::from_system(CursorType::Arrow)?;
    let mut rw = RenderWindow::new(
        (800, 800),
        "SFML cursor example",
        Style::CLOSE,
        &ContextSettings::default(),
    )?;
    rw.set_vertical_sync_enabled(true);
    let font = Font::from_file("sansation.ttf")?;
    let mut failed_index = usize::MAX;
    let mut selected_index = usize::MAX;
    let set_button = Rect::new(348, 500, 100, 32);
    let hotspot_button = Rect::new(458, 500, 100, 32);
    let clear_button = Rect::new(568, 500, 100, 32);
    let mut pixel_grid = [false; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize];
    let mut hotspot_selection = false;
    let mut hotspot_selected = false;
    let mut hotspot = Vector2::new(8, 8);
    let mut modif = false;

    let mut buttons = Vec::new();
    let cursor_types = [
        CursorType::Arrow,
        CursorType::ArrowWait,
        CursorType::Wait,
        CursorType::Text,
        CursorType::Hand,
        CursorType::SizeHorizontal,
        CursorType::SizeVertical,
        CursorType::SizeTopLeftBottomRight,
        CursorType::SizeBottomLeftTopRight,
        CursorType::SizeLeft,
        CursorType::SizeRight,
        CursorType::SizeTop,
        CursorType::SizeBottom,
        CursorType::SizeTopLeft,
        CursorType::SizeBottomRight,
        CursorType::SizeBottomLeft,
        CursorType::SizeTopRight,
        CursorType::SizeAll,
        CursorType::Cross,
        CursorType::Help,
        CursorType::NotAllowed,
    ];
    for i in 0..cursor_types.len() {
        buttons.push(Rect::new(16, 16 + i as i32 * 36, 250, 32));
    }

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::MouseButtonPressed {
                    button: mouse::Button::Left,
                    x,
                    y,
                } => {
                    for (i, b) in buttons.iter().enumerate() {
                        if mouse_over(b, x, y) {
                            match cursor.load_from_system(cursor_types[i]) {
                                Ok(()) => {
                                    unsafe {
                                        rw.set_mouse_cursor(&cursor);
                                    }
                                    selected_index = i;
                                }
                                Err(e) => {
                                    eprintln!("{e}");
                                    failed_index = i;
                                }
                            }
                        }
                    }
                    if mouse_over(&set_button, x, y) {
                        let mut pixels = [0; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize * 4];
                        for (i, px) in pixel_grid.iter().enumerate() {
                            let offset = i * 4;
                            if *px {
                                pixels[offset] = 255;
                                pixels[offset + 1] = 255;
                                pixels[offset + 2] = 255;
                                pixels[offset + 3] = 255;
                            }
                        }
                        unsafe {
                            match cursor.load_from_pixels(
                                &pixels,
                                Vector2::new(DRAW_GRID_WH as u32, DRAW_GRID_WH as u32),
                                hotspot,
                            ) {
                                Ok(()) => {
                                    rw.set_mouse_cursor(&cursor);
                                }
                                Err(e) => {
                                    eprintln!("{e}");
                                }
                            }
                        }
                        modif = false;
                    }
                    if mouse_over(&clear_button, x, y) {
                        for px in pixel_grid.iter_mut() {
                            *px = false;
                        }
                        modif = true;
                    }
                    if mouse_over(&hotspot_button, x, y) {
                        hotspot_selection = true;
                    }
                }
                Event::MouseButtonReleased {
                    button: mouse::Button::Left,
                    ..
                } => {
                    if hotspot_selected {
                        hotspot_selection = false;
                        hotspot_selected = false;
                    }
                }
                _ => {}
            }
        }
        let mut set_button_highlighted = false;
        let mut hotspot_button_highlighted = false;
        let mut clear_button_highlighted = false;
        // System cursor set button interactions
        let mp = rw.mouse_position();
        let mut highlight_index = usize::MAX;
        for (i, b) in buttons.iter().enumerate() {
            if mouse_over(b, mp.x, mp.y) {
                highlight_index = i;
            }
        }
        if mouse_over(&set_button, mp.x, mp.y) {
            set_button_highlighted = true;
        }
        if mouse_over(&hotspot_button, mp.x, mp.y) {
            hotspot_button_highlighted = true;
        }
        if mouse_over(&clear_button, mp.x, mp.y) {
            clear_button_highlighted = true;
        }
        // Grid interactions
        let rela_x = mp.x - DRAW_AREA_TOPLEFT.0 as i32;
        let rela_y = mp.y - DRAW_AREA_TOPLEFT.1 as i32;
        let (gx, gy) = (rela_x / DRAW_CELL_WH as i32, rela_y / DRAW_CELL_WH as i32);
        if gx >= 0 && gy >= 0 {
            if let Some(cell) = gridindex(&mut pixel_grid, gx as usize, gy as usize) {
                if hotspot_selection {
                    hotspot_selected = true;
                    hotspot = Vector2::new(gx as u32, gy as u32);
                    modif = true;
                } else if mouse::Button::Left.is_pressed() {
                    *cell = true;
                    modif = true;
                } else if mouse::Button::Right.is_pressed() {
                    *cell = false;
                    modif = true;
                }
            }
        }
        rw.clear(Color::BLACK);
        // Draw system cursor set buttons
        let mut shape = RectangleShape::default();
        let mut text = Text::new(String::new(), &font, 14);
        shape.set_outline_thickness(-1.0);
        shape.set_outline_color(Color::WHITE);
        for (i, b) in buttons.iter().enumerate() {
            let types = [
                "ARROW",
                "ARROW_WAIT",
                "WAIT",
                "TEXT",
                "HAND",
                "SIZE_HORIZONTAL",
                "SIZE_VERTICAL",
                "SIZE_TOP_LEFT_BOTTOM_RIGHT",
                "SIZE_BOTTOM_LEFT_TOP_RIGHT",
                "SIZE_LEFT",
                "SIZE_RIGHT",
                "SIZE_TOP",
                "SIZE_BOTTOM",
                "SIZE_TOP_LEFT",
                "SIZE_BOTTOM_RIGHT",
                "SIZE_BOTTOM_LEFT",
                "SIZE_TOP_RIGHT",
                "SIZE_ALL",
                "CROSS",
                "HELP",
                "NOT_ALLOWED",
            ];
            draw_button(
                b,
                &mut shape,
                &mut text,
                types[i],
                &mut rw,
                bstyle(highlight_index == i, selected_index == i, failed_index == i),
            );
        }
        // Draw pixel drawing grid
        shape.set_fill_color(Color::TRANSPARENT);
        for y in 0..DRAW_GRID_WH {
            for x in 0..DRAW_GRID_WH {
                if hotspot.x == x as u32 && hotspot.y == y as u32 {
                    shape.set_outline_color(Color::RED);
                } else {
                    shape.set_outline_color(Color::rgb(180, 180, 180));
                }
                if gridindex(&mut pixel_grid, x as usize, y as usize).is_some_and(|bool| *bool) {
                    shape.set_fill_color(Color::WHITE);
                } else {
                    shape.set_fill_color(Color::TRANSPARENT);
                }
                shape.set_size((DRAW_CELL_WH as f32, DRAW_CELL_WH as f32));
                shape.set_position((
                    DRAW_AREA_TOPLEFT.0 as f32 + (x as f32 * DRAW_CELL_WH as f32),
                    DRAW_AREA_TOPLEFT.1 as f32 + (y as f32 * DRAW_CELL_WH as f32),
                ));
                rw.draw_rectangle_shape(&shape, &RenderStates::DEFAULT);
            }
        }
        draw_button(
            &set_button,
            &mut shape,
            &mut text,
            if modif { "Set*" } else { "Set" },
            &mut rw,
            bstyle(set_button_highlighted, false, false),
        );
        draw_button(
            &hotspot_button,
            &mut shape,
            &mut text,
            "Hotspot",
            &mut rw,
            bstyle(hotspot_button_highlighted, hotspot_selection, false),
        );
        draw_button(
            &clear_button,
            &mut shape,
            &mut text,
            "Clear",
            &mut rw,
            bstyle(clear_button_highlighted, false, false),
        );
        rw.display();
    }
    Ok(())
}
