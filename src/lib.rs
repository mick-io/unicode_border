/// `TextBorderOptions` is a structure used to specify the configuration for text borders.
///
/// # Fields
///
/// * `border_char` - The character used to create the border.
/// * `border_thickness` - A tuple specifying the border thickness in the order (left, top, right, bottom).
/// * `margin_thickness` - A tuple specifying the margin thickness in the order (left, top, right, bottom).
/// * `prevent_trim` - A boolean flag indicating whether to prevent trimming whitespace from the message.
///
/// # Examples
///
/// ```
/// let options = TextBorderOptions {
///     border_char: '#',
///     border_thickness: (2, 2, 2, 2),
///     margin_thickness: (1, 1, 1, 1),
///     prevent_trim: true,
/// };
/// ```
pub struct TextBorderOptions {
    border_char: char,
    border_thickness: (usize, usize, usize, usize),
    margin_thickness: (usize, usize, usize, usize),
    prevent_trim: bool,
}

impl Default for TextBorderOptions {
    fn default() -> Self {
        Self {
            border_char: '*',
            border_thickness: (1, 1, 1, 1),
            margin_thickness: (0, 0, 0, 0),
            prevent_trim: false,
        }
    }
}

impl TextBorderOptions {
    fn create_border_line(&self, message: &str) -> String {
        self.border_char.to_string().repeat(
            message.len()
                + self.border_thickness.0
                + self.border_thickness.2
                + self.margin_thickness.0
                + self.margin_thickness.2,
        )
    }

    fn create_margin_line(&self, message: &str) -> String {
        format!(
            "{}{}{}",
            self.border_char.to_string().repeat(self.border_thickness.0),
            " ".repeat(message.len() + self.margin_thickness.0 + self.margin_thickness.2),
            self.border_char.to_string().repeat(self.border_thickness.2)
        )
    }

    fn create_message_line(&self, message: &str) -> String {
        format!(
            "{}{}{}{}{}",
            self.border_char.to_string().repeat(self.border_thickness.0),
            " ".repeat(self.margin_thickness.0),
            message,
            " ".repeat(self.margin_thickness.2),
            self.border_char.to_string().repeat(self.border_thickness.2)
        )
    }
}

/// Creates a string containing the input message, surrounded by a border and margin
/// as specified by the provided `TextBorderOptions`.
///
/// # Arguments
///
/// * `message` - The message (`&str`) to be surrounded by a border.
/// * `options` - An optional `TextBorderOptions` instance specifying the border and margin
///               configurations. If `None`, default options are used.
///
/// # Returns
///
/// * A `String` containing the input message surrounded by the specified border and margin.
///
/// # Examples
///
/// ```
/// let message = "Hello, World!";
///
/// let options = TextBorderOptions {
///     border_char: '#',
///     border_thickness: (2, 2, 2, 2),
///     margin_thickness: (1, 1, 1, 1),
///     prevent_trim: true,
/// };
///
/// let bordered_text = create_text_border(message, Some(options));
/// println!("{}", bordered_text);
/// ```
pub fn create_text_border(message: &str, options: Option<TextBorderOptions>) -> String {
    let opts = options.unwrap_or_default();

    let output_message = if opts.prevent_trim {
        message.to_string()
    } else {
        message.trim().to_string()
    };

    let horizontal_border = opts.create_border_line(&output_message);
    let margin_line = opts.create_margin_line(&output_message);

    let mut bordered_message = Vec::new();

    bordered_message.extend(vec![horizontal_border.clone(); opts.border_thickness.1]);
    bordered_message.extend(vec![margin_line.clone(); opts.margin_thickness.1]);

    bordered_message.push(opts.create_message_line(&output_message));

    bordered_message.extend(vec![margin_line.clone(); opts.margin_thickness.3]);
    bordered_message.extend(vec![horizontal_border; opts.border_thickness.3]);

    bordered_message.join("\n")
}
