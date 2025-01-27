use std::sync::Arc;

use freya_node_state::Parse;
use skia_safe::Color;
use winit::window::Window;

/// Configuration for a Window.
#[derive(Clone)]
pub struct WindowConfig<T: Clone> {
    /// Width of the Window.
    pub width: f64,
    /// Height of the window.
    pub height: f64,
    /// Enable Window decorations.
    pub decorations: bool,
    /// Title for the Window.
    pub title: &'static str,
    /// Make the Window transparent or not.
    pub transparent: bool,
    /// A custom value to consume from your app.
    pub state: Option<T>,
    /// Background color of the Window.
    pub background: Color,
    /// Setup callback.
    pub on_setup: Option<WindowCallback>,
    /// Exit callback.
    pub on_exit: Option<WindowCallback>,
}

impl<T: Clone> Default for WindowConfig<T> {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 600.0,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
            background: Color::WHITE,
            on_setup: None,
            on_exit: None,
        }
    }
}

/// Launch configuration.
#[derive(Clone, Default)]
pub struct LaunchConfig<'a, T: Clone> {
    pub window: WindowConfig<T>,
    pub fonts: Vec<(&'a str, &'a [u8])>,
}

impl<'a, T: Clone> LaunchConfig<'a, T> {
    pub fn builder() -> LaunchConfigBuilder<'a, T> {
        LaunchConfigBuilder::default()
    }
}

pub type WindowCallback = Arc<Box<fn(&mut Window)>>;

/// Configuration Builder.
#[derive(Clone)]
pub struct LaunchConfigBuilder<'a, T> {
    pub width: f64,
    pub height: f64,
    pub decorations: bool,
    pub title: &'static str,
    pub transparent: bool,
    pub state: Option<T>,
    pub background: Color,
    pub fonts: Vec<(&'a str, &'a [u8])>,
    pub on_setup: Option<WindowCallback>,
    pub on_exit: Option<WindowCallback>,
}

impl<T> Default for LaunchConfigBuilder<'_, T> {
    fn default() -> Self {
        Self {
            width: 350.0,
            height: 350.0,
            decorations: true,
            title: "Freya app",
            transparent: false,
            state: None,
            background: Color::WHITE,
            fonts: Vec::default(),
            on_setup: None,
            on_exit: None,
        }
    }
}

impl<'a, T: Clone> LaunchConfigBuilder<'a, T> {
    /// Specify a Window width.
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Specify a Window height.
    pub fn with_height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    /// Whether the Window will have decorations or not.
    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Specify the Window title.
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    /// Make the Window transparent or not.
    pub fn with_transparency(mut self, transparency: bool) -> Self {
        self.transparent = transparency;
        self
    }

    /// Pass a custom value that your app will consume.
    pub fn with_state(mut self, state: T) -> Self {
        self.state = Some(state);
        self
    }

    /// Specify the Window background color.
    pub fn with_background(mut self, background: &str) -> Self {
        self.background = Color::parse(background).unwrap_or(Color::WHITE);
        self
    }

    /// Register a font.
    pub fn with_font(mut self, font_name: &'a str, font: &'a [u8]) -> Self {
        self.fonts.push((font_name, font));
        self
    }

    /// Register a callback that will be executed when the window is created.
    pub fn on_setup(mut self, callback: fn(&mut Window)) -> Self {
        self.on_setup = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Register a callback that will be executed when the window is closed.
    pub fn on_exit(mut self, callback: fn(&mut Window)) -> Self {
        self.on_exit = Some(Arc::new(Box::new(callback)));
        self
    }

    /// Build the configuration.
    pub fn build(self) -> LaunchConfig<'a, T> {
        LaunchConfig {
            window: WindowConfig {
                width: self.width,
                height: self.height,
                title: self.title,
                decorations: self.decorations,
                transparent: self.transparent,
                state: self.state,
                background: self.background,
                on_setup: self.on_setup,
                on_exit: self.on_exit,
            },
            fonts: self.fonts,
        }
    }
}
