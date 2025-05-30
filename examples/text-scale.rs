use sf2g::{
    SfResult,
    graphics::{Color, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable},
    system::Vector2,
    window::{ContextSettings, Event, Style},
};

fn main() -> SfResult<()> {
    let mut rw = RenderWindow::new(
        (640, 480),
        "Text scale",
        Style::CLOSE,
        &ContextSettings::default(),
    )?;
    rw.set_framerate_limit(60);
    let font = Font::from_memory_static(include_bytes!("resources/sansation.ttf"))?;
    let mut text = Text::new("Scale me!".into(), &font, 10);
    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            if ev == Event::Closed {
                rw.close()
            }
        }
        rw.clear(Color::BLACK);
        let Vector2 { x: mx, y: my } = rw.mouse_position();
        let [x, y] = [1.0 + mx as f32 / 640., 1.0 + my as f32 / 480.];
        text.set_scale([x, y]);
        text.set_string(format!("Scale me! ({x}x{y})"));
        text.draw(&mut *rw, &RenderStates::DEFAULT);
        rw.display();
    }
    Ok(())
}
