pub mod matcell {
    use iced::advanced::{
        widget::{
            self, Widget, 
            tree::{self, Tree}, 
            operation::{self, Operation},
        },
        layout::{self, Layout},
        overlay::{self, Overlay},
        Shell, Clipboard, renderer,
    };
    use iced::advanced::text::{self, Text};
    use iced::time::{Duration, Instant};
    use iced::advanced::mouse::{self, click};
    use iced::{keyboard, touch, window};
    use iced::event::{self, Event};
    use iced::{Rectangle, Length, Size, Pixels, Point, 
        Vector, Element, Alignment, alignment, Padding,
        Background, Color, BorderRadius, Command};
    use iced::widget::text_input::{self, StyleSheet, Appearance};
    // #[path = "../cell.rs"]
    // pub mod cell;

    pub mod cursor;
    pub mod value;
    pub mod editor;
    use value::Value;
    use cursor::Cursor;
    use editor::Editor;

    pub struct MatCell<'a, Message, Renderer> 
    where 
        Renderer: text::Renderer,
        Renderer::Theme: StyleSheet,
    {
        id: Option<Id>,
        pos: usize,
        placeholder: String,
        value: Value,
        is_secure: bool,
        font: Option<Renderer::Font>,
        width: Length,
        height: Length,
        padding: Padding,
        size: Option<f32>,
        line_height: text::LineHeight,
        on_input: Option<Box<dyn Fn((usize, String)) -> Message + 'a>>,
        on_paste: Option<Box<dyn Fn((usize, String)) -> Message + 'a>>,
        on_submit: Option<Message>,
        icon: Option<Icon<Renderer::Font>>,
        style: <Renderer::Theme as StyleSheet>::Style,
    }

    pub const DEF_PAD: Padding = Padding::new(6.0);

    impl<'a, Message, Renderer> MatCell<'a, Message, Renderer> 
    where 
        Message: Clone, 
        Renderer: text::Renderer,
        Renderer::Theme: StyleSheet,
    {
        pub fn new(placeholder: &str, value: &str, pos: usize) -> Self {
            MatCell {
                pos,
                id: None, 
                placeholder: String::from(placeholder),
                value: Value::new(value),
                is_secure: false,
                font: None,
                width: Length::Fixed(100.),
                height: Length::Fixed(20.),
                padding: DEF_PAD,
                size: None,
                // line_height: text::LineHeight::Relative(0.2),
                line_height: text::LineHeight::default(),
                on_input: None,
                on_paste: None,
                on_submit: None,
                icon: None,
                style: Default::default(),
            }
        }

        pub fn id(mut self, id: Id) -> Self {
            self.id = Some(id);
            self
        }

        pub fn password(mut self) -> Self {
            self.is_secure = true;
            self
        }

        pub fn on_input<F>(mut self, callback: F) -> Self 
        where 
            F: 'a + Fn((usize, String)) -> Message,
        {
            self.on_input = Some(Box::new(callback));
            self
        }

        pub fn on_paste(
            mut self,
            on_paste: impl Fn((usize, String)) -> Message + 'a,
        ) -> Self {
            self.on_paste = Some(Box::new(on_paste));
            self
        }

        pub fn on_submit(mut self, message: Message) -> Self {
            self.on_submit = Some(message);
            self
        }

        pub fn font(mut self, font: Renderer::Font) -> Self {
            self.font = Some(font);
            self
        }

        pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self {
            self.icon = Some(icon);
            self
        }

        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.width = width.into();
            self
        }

        pub fn height(mut self, height: impl Into<Length>) -> Self {
            self.height = height.into();
            self
        }

        pub fn line_height(
            mut self, 
            line_height: impl Into<text::LineHeight>
        ) -> Self {
            self.line_height = line_height.into();
            self
        }

        pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
            self.padding = padding.into();
            self
        }

        pub fn size(mut self, size: impl Into<Pixels>) -> Self {
            self.size = Some(size.into().0);
            self
        }

        fn style(
            mut self, 
            style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
        ) -> Self {
            self.style = style.into();
            self
        }

        pub fn draw(
            &self, 
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            value: Option<&Value>,
        ) {
            draw(
                renderer,
                theme,
                layout,
                cursor,
                tree.state.downcast_ref::<State>(),
                value.unwrap_or(&self.value),
                &self.placeholder,
                self.size,
                self.line_height,
                self.font,
                self.on_input.is_none(),
                self.is_secure,
                self.icon.as_ref(),
                &self.style,
            )
        }
    }

    impl<'a, Message, Renderer> Widget<Message, Renderer>
        for MatCell<'a, Message, Renderer> 
    where
        Message: Clone,
        Renderer: text::Renderer,
        Renderer::Theme: StyleSheet,
    {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<State>()
        }

        fn state(&self) -> tree::State {
            tree::State::new(State::new())
        }

        fn diff(&self, tree: &mut Tree) {
            let state = tree.state.downcast_mut::<State>();

            if self.on_input.is_none() {
                state.last_click = None;
                state.is_focused = None;
                state.is_pasting = None;
                state.is_dragging = false;
            }
        }

        fn width(&self) -> Length {
            self.width
        }

        fn height(&self) -> Length {
            self.height
        }

        fn layout(
            &self,
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            layout(
                renderer,
                limits,
                self.width,
                self.height,
                self.padding,
                self.size,
                self.line_height,
                self.icon.as_ref(),
            )
        }

        fn operate(
            &self,
            tree: &mut Tree,
            _layout: Layout<'_>,
            _renderer: &Renderer,
            operation: &mut dyn Operation<Message>,
        ) {
            let state = tree.state.downcast_mut::<State>();

            operation.focusable(state, self.id.as_ref().map(|id| &id.0));
            operation.text_input(state, self.id.as_ref().map(|id| &id.0));
        }

        fn on_event(
            &mut self,
            tree: &mut Tree,
            event: Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            _viewport: &Rectangle,
        ) -> event::Status {
            update(
                self.pos,
                event, 
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &mut self.value,
                self.size,
                self.line_height,
                self.font,
                self.is_secure,
                self.on_input.as_deref(),
                self.on_paste.as_deref(),
                &self.on_submit,
                || tree.state.downcast_mut::<State>(),
            )
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Renderer::Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            draw(
                renderer,
                theme,
                layout,
                cursor,
                tree.state.downcast_ref::<State>(),
                &self.value,
                &self.placeholder,
                self.size,
                self.line_height,
                self.font,
                self.on_input.is_none(),
                self.is_secure,
                self.icon.as_ref(),
                &self.style,
            )
        }

        fn mouse_interaction(
            &self,
            _state: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            _viewport: &Rectangle,
            _renderer: &Renderer,
        ) -> mouse::Interaction {
            mouse_interaction(layout, cursor, self.on_input.is_none())
        }
    }

    impl<'a, Message, Renderer> From<MatCell<'a, Message, Renderer>>
        for Element<'a, Message, Renderer>
    where 
        Message: 'a + Clone,
        Renderer: 'a + text::Renderer,
        Renderer::Theme: StyleSheet,
    {
        fn from(matcell: MatCell<'a, Message, Renderer>) -> Self {
            Element::new(matcell)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Icon<Font> {
        pub font: Font,
        pub code_point: char,
        pub size: Option<f32>,
        pub spacing: f32,
        pub side: Side,
    }

    #[derive(Debug, Clone)]
    pub enum Side {
        Right,
        Left,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Id(widget::Id);

    impl Id {
        pub fn new(id: impl Into<std::borrow::Cow<'static, str>>) -> Self {
            Self(widget::Id::new(id))
        }

        pub fn unique() -> Self {
            Self(widget::Id::unique())
        }
    }

    impl From<Id> for widget::Id {
        fn from(id: Id) -> Self {
            id.0
        }
    }

    pub fn focus<Message: 'static>(id: Id) -> Command<Message> {
        Command::widget(operation::focusable::focus(id.0))
    }

    pub fn move_cursor_to_end<Message: 'static>(id: Id) -> Command<Message> {
        Command::widget(operation::text_input::move_cursor_to_end(id.0))
    }

    pub fn move_cursor_to_front<Message: 'static>(id: Id) -> Command<Message> {
        Command::widget(operation::text_input::move_cursor_to_front(id.0))
    }

    pub fn move_cursor_to<Message: 'static>(
        id: Id,
        position: usize,
    ) -> Command<Message> {
        Command::widget(operation::text_input::move_cursor_to(id.0, position))
    }

    pub fn select_all<Message: 'static>(id: Id) -> Command<Message> {
        Command::widget(operation::text_input::select_all(id.0))
    }

    pub fn layout<Renderer>(
        renderer: &Renderer,
        limits: &layout::Limits,
        width: Length,
        height: Length,
        padding: Padding,
        size: Option<f32>,
        line_height: text::LineHeight,
        icon: Option<&Icon<Renderer::Font>>,
    ) -> layout::Node 
    where 
        Renderer: text::Renderer,
    {
        let text_size = size.unwrap_or_else(|| renderer.default_size());
        let padding = padding.fit(Size::ZERO, limits.max());
        let limits = limits
            .width(width)
            .pad(padding)
            .height(height);
            // .height(line_height.to_absolute(Pixels(text_size)));

        let text_bounds = limits.resolve(Size::ZERO);

        if let Some(icon) = icon {
            let icon_width = renderer.measure_width(
                &icon.code_point.to_string(),
                icon.size.unwrap_or_else(|| renderer.default_size()),
                icon.font,
                text::Shaping::Advanced,
            );

            let mut text_node = layout::Node::new(
                text_bounds - Size::new(icon_width + icon.spacing, 0.0),
            );

            let mut icon_node = 
                layout::Node::new(Size::new(icon_width, text_bounds.height));

            match icon.side {
                Side::Right => {
                    text_node.move_to(Point::new(padding.left, padding.top));

                    icon_node.move_to(Point::new(
                        padding.left + text_bounds.width - icon_width,
                        padding.top,
                    ));
                }
                Side::Left => {
                    text_node.move_to(Point::new(
                        padding.left + icon_width + icon.spacing,
                        padding.top,
                    ));

                    icon_node.move_to(Point::new(padding.left, padding.top));
                }
            };

            layout::Node::with_children(
                text_bounds.pad(padding),
                vec![text_node, icon_node],
            )
        } else {
            let mut text = layout::Node::new(text_bounds);
            text.move_to(Point::new(padding.left, padding.top));

            layout::Node::with_children(text_bounds.pad(padding), vec![text])
        }
    }

    pub fn update<'a, Message, Renderer>(
        pos: usize,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        value: &mut Value,
        size: Option<f32>,
        line_height: text::LineHeight,
        font: Option<Renderer::Font>,
        is_secure: bool,
        on_input: Option<&dyn Fn((usize, String)) -> Message>,
        on_paste: Option<&dyn Fn((usize, String)) -> Message>,
        on_submit: &Option<Message>,
        state: impl FnOnce() -> &'a mut State,
    ) -> event::Status
    where
        Message: Clone,
        Renderer: text::Renderer,
    {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let state = state();

                let click_position = if on_input.is_some() {
                    cursor.position_over(layout.bounds())
                } else {
                    None
                };

                state.is_focused = if click_position.is_some() {
                    state.is_focused.or_else(|| {
                        let now = Instant::now();

                        Some(Focus {
                            updated_at: now,
                            now,
                            is_window_focused: true,
                        })
                    })
                } else {
                    None
                };

                if let Some(cursor_position) = click_position {
                    let text_layout = layout.children().next().unwrap();
                    let target = cursor_position.x - text_layout.bounds().x;

                    let click =
                        mouse::Click::new(cursor_position, state.last_click);

                    match click.kind() {
                        click::Kind::Single => {
                            let position = if target > 0.0 {
                                let value = if is_secure {
                                    value.secure()
                                } else {
                                    value.clone()
                                };

                                find_cursor_position(
                                    renderer,
                                    text_layout.bounds(),
                                    font,
                                    size,
                                    line_height,
                                    &value,
                                    state,
                                    target,
                                )
                            } else {
                                None
                            }
                            .unwrap_or(0);

                            if state.keyboard_modifiers.shift() {
                                state.cursor.select_range(
                                    state.cursor.start(value),
                                    position,
                                );
                            } else {
                                state.cursor.move_to(position);
                            }
                            state.is_dragging = true;
                        }
                        click::Kind::Double => {
                            if is_secure {
                                state.cursor.select_all(value);
                            } else {
                                let position = find_cursor_position(
                                    renderer,
                                    text_layout.bounds(),
                                    font,
                                    size,
                                    line_height,
                                    value,
                                    state,
                                    target,
                                )
                                .unwrap_or(0);

                                state.cursor.select_range(
                                    value.previous_start_of_word(position),
                                    value.next_end_of_word(position),
                                );
                            }

                            state.is_dragging = false;
                        }
                        click::Kind::Triple => {
                            state.cursor.select_all(value);
                            state.is_dragging = false;
                        }
                    }

                    state.last_click = Some(click);

                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state().is_dragging = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                let state = state();

                if state.is_dragging {
                    let text_layout = layout.children().next().unwrap();
                    let target = position.x - text_layout.bounds().x;

                    let value = if is_secure {
                        value.secure()
                    } else {
                        value.clone()
                    };

                    let position = find_cursor_position(
                        renderer,
                        text_layout.bounds(),
                        font,
                        size,
                        line_height,
                        &value,
                        state,
                        target,
                    )
                    .unwrap_or(0);

                    state
                        .cursor
                        .select_range(state.cursor.start(&value), position);

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard::Event::CharacterReceived(c)) => {
                let state = state();

                if let Some(focus) = &mut state.is_focused {
                    let Some(on_input) = on_input else { return event::Status::Ignored };

                    if state.is_pasting.is_none()
                        && !state.keyboard_modifiers.command()
                        && !c.is_control()
                    {
                        let mut editor = Editor::new(value, &mut state.cursor);

                        editor.insert(c);

                        let message = (on_input)((pos, editor.contents()));
                        shell.publish(message);

                        focus.updated_at = Instant::now();

                        return event::Status::Captured;
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => {
                let state = state();

                if let Some(focus) = &mut state.is_focused {
                    let Some(on_input) = on_input else { return event::Status::Ignored };

                    let modifiers = state.keyboard_modifiers;
                    focus.updated_at = Instant::now();

                    match key_code {
                        keyboard::KeyCode::Enter
                        | keyboard::KeyCode::NumpadEnter => {
                            if let Some(on_submit) = on_submit.clone() {
                                shell.publish(on_submit);
                            }
                        }
                        keyboard::KeyCode::Backspace => {
                            if platform::is_jump_modifier_pressed(modifiers)
                                && state.cursor.selection(value).is_none()
                            {
                                if is_secure {
                                    let cursor_pos = state.cursor.end(value);
                                    state.cursor.select_range(0, cursor_pos);
                                } else {
                                    state.cursor.select_left_by_words(value);
                                }
                            }

                            let mut editor = Editor::new(value, &mut state.cursor);
                            editor.backspace();

                            let message = (on_input)((pos, editor.contents()));
                            shell.publish(message);
                        }
                        keyboard::KeyCode::Delete => {
                            if platform::is_jump_modifier_pressed(modifiers)
                                && state.cursor.selection(value).is_none()
                            {
                                if is_secure {
                                    let cursor_pos = state.cursor.end(value);
                                    state
                                        .cursor
                                        .select_range(cursor_pos, value.len());
                                } else {
                                    state.cursor.select_right_by_words(value);
                                }
                            }

                            let mut editor = Editor::new(value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)((pos, editor.contents()));
                            shell.publish(message);
                        }
                        keyboard::KeyCode::Left => {
                            if platform::is_jump_modifier_pressed(modifiers)
                                && !is_secure
                            {
                                if modifiers.shift() {
                                    state.cursor.select_left_by_words(value);
                                } else {
                                    state.cursor.move_left_by_words(value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_left(value)
                            } else {
                                state.cursor.move_left(value);
                            }
                        }
                        keyboard::KeyCode::Right => {
                            if platform::is_jump_modifier_pressed(modifiers)
                                && !is_secure
                            {
                                if modifiers.shift() {
                                    state.cursor.select_right_by_words(value);
                                } else {
                                    state.cursor.move_right_by_words(value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_right(value)
                            } else {
                                state.cursor.move_right(value);
                            }
                        }
                        keyboard::KeyCode::Home => {
                            if modifiers.shift() {
                                state
                                    .cursor
                                    .select_range(state.cursor.start(value), 0);
                            } else {
                                state.cursor.move_to(0);
                            }
                        }
                        keyboard::KeyCode::End => {
                            if modifiers.shift() {
                                state.cursor.select_range(
                                    state.cursor.start(value),
                                    value.len(),
                                );
                            } else {
                                state.cursor.move_to(value.len());
                            }
                        }
                        keyboard::KeyCode::C
                            if state.keyboard_modifiers.command() =>
                        {
                            if let Some((start, end)) =
                                state.cursor.selection(value)
                            {
                                clipboard
                                    .write(value.select(start, end).to_string());
                            }
                        }
                        keyboard::KeyCode::X
                            if state.keyboard_modifiers.command() =>
                        {
                            if let Some((start, end)) =
                                state.cursor.selection(value)
                            {
                                clipboard
                                    .write(value.select(start, end).to_string());
                            }

                            let mut editor = Editor::new(value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)((pos, editor.contents()));
                            shell.publish(message);
                        }
                        keyboard::KeyCode::V => {
                            if state.keyboard_modifiers.command() {
                                let content = match state.is_pasting.take() {
                                    Some(content) => content,
                                    None => {
                                        let content: String = clipboard
                                            .read()
                                            .unwrap_or_default()
                                            .chars()
                                            .filter(|c| !c.is_control())
                                            .collect();

                                        Value::new(&content)
                                    }
                                };

                                let mut editor =
                                    Editor::new(value, &mut state.cursor);

                                editor.paste(content.clone());

                                let message = if let Some(paste) = &on_paste {
                                    (paste)((pos, editor.contents()))
                                } else {
                                    (on_input)((pos, editor.contents()))
                                };
                                shell.publish(message);

                                state.is_pasting = Some(content);
                            } else {
                                state.is_pasting = None;
                            }
                        }
                        keyboard::KeyCode::A
                            if state.keyboard_modifiers.command() =>
                        {
                            state.cursor.select_all(value);
                        }
                        keyboard::KeyCode::Escape => {
                            state.is_focused = None;
                            state.is_dragging = false;
                            state.is_pasting = None;

                            state.keyboard_modifiers =
                                keyboard::Modifiers::default();
                        }
                        keyboard::KeyCode::Tab
                        | keyboard::KeyCode::Up
                        | keyboard::KeyCode::Down => {
                            return event::Status::Ignored;
                        }
                        _ => {}
                    }

                    return event::Status::Captured;
                }
            }
            Event::Keyboard(keyboard::Event::KeyReleased { key_code, .. }) => {
                let state = state();

                if state.is_focused.is_some() {
                    match key_code {
                        keyboard::KeyCode::V => {
                            state.is_pasting = None;
                        }
                        keyboard::KeyCode::Tab
                        | keyboard::KeyCode::Up
                        | keyboard::KeyCode::Down => {
                            return event::Status::Ignored;
                        }
                        _ => {}
                    }

                    return event::Status::Captured;
                } else {
                    state.is_pasting = None;
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                let state = state();

                state.keyboard_modifiers = modifiers;
            }
            Event::Window(window::Event::Unfocused) => {
                let state = state();

                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = false;
                }
            }
            Event::Window(window::Event::Focused) => {
                let state = state();

                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = true;
                    focus.updated_at = Instant::now();

                    shell.request_redraw(window::RedrawRequest::NextFrame);
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                let state = state();

                if let Some(focus) = &mut state.is_focused {
                    if focus.is_window_focused {
                        focus.now = now;

                        let millis_until_redraw = CURSOR_BLINK_INTERVAL_MILLIS
                            - (now - focus.updated_at).as_millis()
                                % CURSOR_BLINK_INTERVAL_MILLIS;

                        shell.request_redraw(window::RedrawRequest::At(
                            now + Duration::from_millis(millis_until_redraw as u64),
                        ));
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    /// Draws the [`TextInput`] with the given [`Renderer`], overriding its
    /// [`Value`] if provided.
    ///
    /// [`Renderer`]: text::Renderer
    pub fn draw<Renderer>(
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        state: &State,
        value: &Value,
        placeholder: &str,
        size: Option<f32>,
        line_height: text::LineHeight,
        font: Option<Renderer::Font>,
        is_disabled: bool,
        is_secure: bool,
        icon: Option<&Icon<Renderer::Font>>,
        style: &<Renderer::Theme as StyleSheet>::Style,
    ) where
        Renderer: text::Renderer,
        Renderer::Theme: StyleSheet,
    {
        let secure_value = is_secure.then(|| value.secure());
        let value = secure_value.as_ref().unwrap_or(value);

        let bounds = layout.bounds();

        let mut children_layout = layout.children();
        let text_bounds = children_layout.next().unwrap().bounds();

        let is_mouse_over = cursor.is_over(bounds);

        let appearance = if is_disabled {
            theme.disabled(style)
        } else if state.is_focused() {
            theme.focused(style)
        } else if is_mouse_over {
            theme.hovered(style)
        } else {
            theme.active(style)
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: appearance.border_radius,
                border_width: appearance.border_width,
                border_color: appearance.border_color,
            },
            appearance.background,
        );

        if let Some(icon) = icon {
            let icon_layout = children_layout.next().unwrap();

            renderer.fill_text(Text {
                content: &icon.code_point.to_string(),
                size: icon.size.unwrap_or_else(|| renderer.default_size()),
                line_height: text::LineHeight::default(),
                font: icon.font,
                color: appearance.icon_color,
                bounds: Rectangle {
                    y: text_bounds.center_y(),
                    ..icon_layout.bounds()
                },
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Center,
                shaping: text::Shaping::Advanced,
            });
        }

        let text = value.to_string();
        let font = font.unwrap_or_else(|| renderer.default_font());
        let size = size.unwrap_or_else(|| renderer.default_size());

        let (cursor, offset) = if let Some(focus) = state
            .is_focused
            .as_ref()
            .filter(|focus| focus.is_window_focused)
        {
            match state.cursor.state(value) {
                cursor::State::Index(position) => {
                    let (text_value_width, offset) =
                        measure_cursor_and_scroll_offset(
                            renderer,
                            text_bounds,
                            value,
                            size,
                            position,
                            font,
                        );

                    let is_cursor_visible = ((focus.now - focus.updated_at)
                        .as_millis()
                        / CURSOR_BLINK_INTERVAL_MILLIS)
                        % 2
                        == 0;

                    let cursor = if is_cursor_visible {
                        Some((
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: text_bounds.x + text_value_width,
                                    y: text_bounds.y,
                                    width: 1.0,
                                    height: text_bounds.height,
                                },
                                border_radius: 0.0.into(),
                                border_width: 0.0,
                                border_color: Color::TRANSPARENT,
                            },
                            theme.value_color(style),
                        ))
                    } else {
                        None
                    };

                    (cursor, offset)
                }
                cursor::State::Selection { start, end } => {
                    let left = start.min(end);
                    let right = end.max(start);

                    let (left_position, left_offset) =
                        measure_cursor_and_scroll_offset(
                            renderer,
                            text_bounds,
                            value,
                            size,
                            left,
                            font,
                        );

                    let (right_position, right_offset) =
                        measure_cursor_and_scroll_offset(
                            renderer,
                            text_bounds,
                            value,
                            size,
                            right,
                            font,
                        );

                    let width = right_position - left_position;

                    (
                        Some((
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: text_bounds.x + left_position,
                                    y: text_bounds.y,
                                    width,
                                    height: text_bounds.height,
                                },
                                border_radius: 0.0.into(),
                                border_width: 0.0,
                                border_color: Color::TRANSPARENT,
                            },
                            theme.selection_color(style),
                        )),
                        if end == right {
                            right_offset
                        } else {
                            left_offset
                        },
                    )
                }
            }
        } else {
            (None, 0.0)
        };

        let text_width = renderer.measure_width(
            if text.is_empty() { placeholder } else { &text },
            size,
            font,
            text::Shaping::Advanced,
        );

        let render = |renderer: &mut Renderer| {
            if let Some((cursor, color)) = cursor {
                renderer.fill_quad(cursor, color);
            } else {
                renderer.with_translation(Vector::ZERO, |_| {});
            }

            renderer.fill_text(Text {
                content: if text.is_empty() { placeholder } else { &text },
                color: if text.is_empty() {
                    theme.placeholder_color(style)
                } else if is_disabled {
                    theme.disabled_color(style)
                } else {
                    theme.value_color(style)
                },
                font,
                bounds: Rectangle {
                    y: text_bounds.center_y(),
                    width: f32::INFINITY,
                    ..text_bounds
                },
                size,
                line_height,
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Center,
                shaping: text::Shaping::Advanced,
            });
        };

        if text_width > text_bounds.width {
            renderer.with_layer(text_bounds, |renderer| {
                renderer.with_translation(Vector::new(-offset, 0.0), render)
            });
        } else {
            render(renderer);
        }
    }

    /// Computes the current [`mouse::Interaction`] of the [`TextInput`].
    pub fn mouse_interaction(
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        is_disabled: bool,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if is_disabled {
                mouse::Interaction::NotAllowed
            } else {
                mouse::Interaction::Text
            }
        } else {
            mouse::Interaction::default()
        }
    }

    /// The state of a [`TextInput`].
    #[derive(Debug, Default, Clone)]
    pub struct State {
        is_focused: Option<Focus>,
        is_dragging: bool,
        is_pasting: Option<Value>,
        last_click: Option<mouse::Click>,
        cursor: Cursor,
        keyboard_modifiers: keyboard::Modifiers,
        // TODO: Add stateful horizontal scrolling offset
    }

    #[derive(Debug, Clone, Copy)]
    struct Focus {
        updated_at: Instant,
        now: Instant,
        is_window_focused: bool,
    }

    impl State {
        /// Creates a new [`State`], representing an unfocused [`TextInput`].
        pub fn new() -> Self {
            Self::default()
        }

        /// Creates a new [`State`], representing a focused [`TextInput`].
        pub fn focused() -> Self {
            Self {
                is_focused: None,
                is_dragging: false,
                is_pasting: None,
                last_click: None,
                cursor: Cursor::default(),
                keyboard_modifiers: keyboard::Modifiers::default(),
            }
        }

        /// Returns whether the [`TextInput`] is currently focused or not.
        pub fn is_focused(&self) -> bool {
            self.is_focused.is_some()
        }

        /// Returns the [`Cursor`] of the [`TextInput`].
        pub fn cursor(&self) -> Cursor {
            self.cursor
        }

        /// Focuses the [`TextInput`].
        pub fn focus(&mut self) {
            let now = Instant::now();

            self.is_focused = Some(Focus {
                updated_at: now,
                now,
                is_window_focused: true,
            });

            self.move_cursor_to_end();
        }

        /// Unfocuses the [`TextInput`].
        pub fn unfocus(&mut self) {
            self.is_focused = None;
        }

        /// Moves the [`Cursor`] of the [`TextInput`] to the front of the input text.
        pub fn move_cursor_to_front(&mut self) {
            self.cursor.move_to(0);
        }

        /// Moves the [`Cursor`] of the [`TextInput`] to the end of the input text.
        pub fn move_cursor_to_end(&mut self) {
            self.cursor.move_to(usize::MAX);
        }

        /// Moves the [`Cursor`] of the [`TextInput`] to an arbitrary location.
        pub fn move_cursor_to(&mut self, position: usize) {
            self.cursor.move_to(position);
        }

        /// Selects all the content of the [`TextInput`].
        pub fn select_all(&mut self) {
            self.cursor.select_range(0, usize::MAX);
        }
    }

    impl operation::Focusable for State {
        fn is_focused(&self) -> bool {
            State::is_focused(self)
        }

        fn focus(&mut self) {
            State::focus(self)
        }

        fn unfocus(&mut self) {
            State::unfocus(self)
        }
    }

    impl operation::TextInput for State {
        fn move_cursor_to_front(&mut self) {
            State::move_cursor_to_front(self)
        }

        fn move_cursor_to_end(&mut self) {
            State::move_cursor_to_end(self)
        }

        fn move_cursor_to(&mut self, position: usize) {
            State::move_cursor_to(self, position)
        }

        fn select_all(&mut self) {
            State::select_all(self)
        }
    }

    mod platform {
        use iced::keyboard;

        pub fn is_jump_modifier_pressed(modifiers: keyboard::Modifiers) -> bool {
            if cfg!(target_os = "macos") {
                modifiers.alt()
            } else {
                modifiers.control()
            }
        }
    }

    fn offset<Renderer>(
        renderer: &Renderer,
        text_bounds: Rectangle,
        font: Renderer::Font,
        size: f32,
        value: &Value,
        state: &State,
    ) -> f32
    where
        Renderer: text::Renderer,
    {
        if state.is_focused() {
            let cursor = state.cursor();

            let focus_position = match cursor.state(value) {
                cursor::State::Index(i) => i,
                cursor::State::Selection { end, .. } => end,
            };

            let (_, offset) = measure_cursor_and_scroll_offset(
                renderer,
                text_bounds,
                value,
                size,
                focus_position,
                font,
            );

            offset
        } else {
            0.0
        }
    }

    fn measure_cursor_and_scroll_offset<Renderer>(
        renderer: &Renderer,
        text_bounds: Rectangle,
        value: &Value,
        size: f32,
        cursor_index: usize,
        font: Renderer::Font,
    ) -> (f32, f32)
    where
        Renderer: text::Renderer,
    {
        let text_before_cursor = value.until(cursor_index).to_string();

        let text_value_width = renderer.measure_width(
            &text_before_cursor,
            size,
            font,
            text::Shaping::Advanced,
        );

        let offset = ((text_value_width + 5.0) - text_bounds.width).max(0.0);

        (text_value_width, offset)
    }

    /// Computes the position of the text cursor at the given X coordinate of
    /// a [`TextInput`].
    fn find_cursor_position<Renderer>(
        renderer: &Renderer,
        text_bounds: Rectangle,
        font: Option<Renderer::Font>,
        size: Option<f32>,
        line_height: text::LineHeight,
        value: &Value,
        state: &State,
        x: f32,
    ) -> Option<usize>
    where
        Renderer: text::Renderer,
    {
        let font = font.unwrap_or_else(|| renderer.default_font());
        let size = size.unwrap_or_else(|| renderer.default_size());

        let offset = offset(renderer, text_bounds, font, size, value, state);
        let value = value.to_string();

        let char_offset = renderer
            .hit_test(
                &value,
                size,
                line_height,
                font,
                Size::INFINITY,
                text::Shaping::Advanced,
                Point::new(x + offset, text_bounds.height / 2.0),
                true,
            )
            .map(text::Hit::cursor)?;

        Some(
            unicode_segmentation::UnicodeSegmentation::graphemes(
                &value[..char_offset],
                true,
            )
            .count(),
        )
    }

    const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;        
}




