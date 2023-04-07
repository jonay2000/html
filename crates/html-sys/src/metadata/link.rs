/// The HTML `<link>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link)
#[doc(alias = "link")]
#[non_exhaustive]
pub struct Link {
    global_attributes: crate::GlobalAttributes,
    /// Address of the hyperlink
pub href: std::option::Option<String>,
/// How the element handles crossorigin requests
pub crossorigin: std::option::Option<String>,
/// Relationship between the document containing the hyperlink and the destination resource
pub rel: std::option::Option<String>,
/// Applicable media
pub media: std::option::Option<String>,
/// Integrity metadata used in Subresource Integrity checks [SRI]
pub integrity: std::option::Option<String>,
/// Language of the linked resource
pub hreflang: std::option::Option<String>,
/// Hint for the type of the referenced resource
pub type_: std::option::Option<String>,
/// Referrer policy for fetches initiated by the element
pub referrerpolicy: std::option::Option<String>,
/// Sizes of the icons (for rel="icon")
pub sizes: std::option::Option<String>,
/// Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (for rel="preload")
pub imagesrcset: std::option::Option<String>,
/// Image sizes for different page layouts (for rel="preload")
pub imagesizes: std::option::Option<String>,
/// Potential destination for a preload request (for rel="preload" and rel="modulepreload")
pub as_: std::option::Option<String>,
/// Whether the element is potentially render-blocking
pub blocking: std::option::Option<String>,
/// Color to use when customizing a site's icon (for rel="mask-icon")
pub color: std::option::Option<String>,
/// Whether the link is disabled
pub disabled: std::option::Option<String>,
/// Sets the priority for fetches initiated by the element
pub fetchpriority: std::option::Option<String>,

}

impl crate::RenderElement for Link {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<link")?;
if let Some(field) = self.href.as_ref() {
    write!(writer, r#""href="{}""#, field)?;
}
if let Some(field) = self.crossorigin.as_ref() {
    write!(writer, r#""crossorigin="{}""#, field)?;
}
if let Some(field) = self.rel.as_ref() {
    write!(writer, r#""rel="{}""#, field)?;
}
if let Some(field) = self.media.as_ref() {
    write!(writer, r#""media="{}""#, field)?;
}
if let Some(field) = self.integrity.as_ref() {
    write!(writer, r#""integrity="{}""#, field)?;
}
if let Some(field) = self.hreflang.as_ref() {
    write!(writer, r#""hreflang="{}""#, field)?;
}
if let Some(field) = self.type_.as_ref() {
    write!(writer, r#""type="{}""#, field)?;
}
if let Some(field) = self.referrerpolicy.as_ref() {
    write!(writer, r#""referrerpolicy="{}""#, field)?;
}
if let Some(field) = self.sizes.as_ref() {
    write!(writer, r#""sizes="{}""#, field)?;
}
if let Some(field) = self.imagesrcset.as_ref() {
    write!(writer, r#""imagesrcset="{}""#, field)?;
}
if let Some(field) = self.imagesizes.as_ref() {
    write!(writer, r#""imagesizes="{}""#, field)?;
}
if let Some(field) = self.as_.as_ref() {
    write!(writer, r#""as="{}""#, field)?;
}
if let Some(field) = self.blocking.as_ref() {
    write!(writer, r#""blocking="{}""#, field)?;
}
if let Some(field) = self.color.as_ref() {
    write!(writer, r#""color="{}""#, field)?;
}
if let Some(field) = self.disabled.as_ref() {
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.fetchpriority.as_ref() {
    write!(writer, r#""fetchpriority="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        
        Ok(())
    }
}
impl std::ops::Deref for Link {
    type Target = crate::GlobalAttributes;

    fn deref(&self) -> &Self::Target {
        &self.global_attributes
    }
}

impl std::ops::DerefMut for Link {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attributes
    }
}
