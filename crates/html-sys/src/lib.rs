pub mod forms;
pub mod metadata;
pub mod interactive;
pub mod edits;
pub mod embedded;
pub mod scripting;
pub mod root;
pub mod text;
pub mod tables;
pub mod sections;

/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}