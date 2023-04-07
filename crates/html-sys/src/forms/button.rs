/// The HTML `<button>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button)
#[doc(alias = "button")]
#[non_exhaustive]
pub struct Button {
    /// Whether the form control is disabled
pub disabled: std::option::Option<String>,
/// Associates the element with a form element
pub form: std::option::Option<String>,
/// URL to use for form submission
pub formaction: std::option::Option<String>,
/// Entry list encoding type to use for form submission
pub formenctype: std::option::Option<String>,
/// Variant to use for form submission
pub formmethod: std::option::Option<String>,
/// Bypass form control validation for form submission
pub formnovalidate: std::option::Option<String>,
/// Navigable for form submission
pub formtarget: std::option::Option<String>,
/// Name of the element to use for form submission and in the form.elements API
pub name: std::option::Option<String>,
/// Type of button
pub type_: std::option::Option<String>,
/// Value to be used for form submission
pub value: std::option::Option<String>,
/// 
pub access_key: std::option::Option<String>,
/// 
pub auto_capitalize: std::option::Option<String>,
/// 
pub autofocus: std::option::Option<String>,
/// 
pub content_editable: std::option::Option<String>,
/// 
pub direction: std::option::Option<String>,
/// 
pub draggable: std::option::Option<String>,
/// 
pub enter_key_hint: std::option::Option<String>,
/// 
pub hidden: std::option::Option<String>,
/// 
pub inert: std::option::Option<String>,
/// 
pub input_mode: std::option::Option<String>,
/// 
pub is_: std::option::Option<String>,
/// 
pub item_id: std::option::Option<String>,
/// 
pub item_prop: std::option::Option<String>,
/// 
pub item_ref: std::option::Option<String>,
/// 
pub item_scope: std::option::Option<String>,
/// 
pub item_type: std::option::Option<String>,
/// 
pub lang: std::option::Option<String>,
/// 
pub nonce: std::option::Option<String>,
/// 
pub popover: std::option::Option<String>,
/// 
pub spellcheck: std::option::Option<String>,
/// 
pub style: std::option::Option<String>,
/// 
pub tab_index: std::option::Option<String>,
/// 
pub title: std::option::Option<String>,
/// 
pub translate: std::option::Option<String>,

}

impl crate::RenderElement for Button {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<button")?;
if let Some(field) = self.disabled.as_ref() {
    write!(writer, r#""disabled="{}""#, field)?;
}
if let Some(field) = self.form.as_ref() {
    write!(writer, r#""form="{}""#, field)?;
}
if let Some(field) = self.formaction.as_ref() {
    write!(writer, r#""formaction="{}""#, field)?;
}
if let Some(field) = self.formenctype.as_ref() {
    write!(writer, r#""formenctype="{}""#, field)?;
}
if let Some(field) = self.formmethod.as_ref() {
    write!(writer, r#""formmethod="{}""#, field)?;
}
if let Some(field) = self.formnovalidate.as_ref() {
    write!(writer, r#""formnovalidate="{}""#, field)?;
}
if let Some(field) = self.formtarget.as_ref() {
    write!(writer, r#""formtarget="{}""#, field)?;
}
if let Some(field) = self.name.as_ref() {
    write!(writer, r#""name="{}""#, field)?;
}
if let Some(field) = self.type_.as_ref() {
    write!(writer, r#""type="{}""#, field)?;
}
if let Some(field) = self.value.as_ref() {
    write!(writer, r#""value="{}""#, field)?;
}
if let Some(field) = self.access_key.as_ref() {
    write!(writer, r#""accesskey="{}""#, field)?;
}
if let Some(field) = self.auto_capitalize.as_ref() {
    write!(writer, r#""autocapitalize="{}""#, field)?;
}
if let Some(field) = self.autofocus.as_ref() {
    write!(writer, r#""autofocus="{}""#, field)?;
}
if let Some(field) = self.content_editable.as_ref() {
    write!(writer, r#""contenteditable="{}""#, field)?;
}
if let Some(field) = self.direction.as_ref() {
    write!(writer, r#""dir="{}""#, field)?;
}
if let Some(field) = self.draggable.as_ref() {
    write!(writer, r#""draggable="{}""#, field)?;
}
if let Some(field) = self.enter_key_hint.as_ref() {
    write!(writer, r#""enterkeyhint="{}""#, field)?;
}
if let Some(field) = self.hidden.as_ref() {
    write!(writer, r#""hidden="{}""#, field)?;
}
if let Some(field) = self.inert.as_ref() {
    write!(writer, r#""inert="{}""#, field)?;
}
if let Some(field) = self.input_mode.as_ref() {
    write!(writer, r#""inputmode="{}""#, field)?;
}
if let Some(field) = self.is_.as_ref() {
    write!(writer, r#""is="{}""#, field)?;
}
if let Some(field) = self.item_id.as_ref() {
    write!(writer, r#""itemid="{}""#, field)?;
}
if let Some(field) = self.item_prop.as_ref() {
    write!(writer, r#""itemprop="{}""#, field)?;
}
if let Some(field) = self.item_ref.as_ref() {
    write!(writer, r#""itemref="{}""#, field)?;
}
if let Some(field) = self.item_scope.as_ref() {
    write!(writer, r#""itemscope="{}""#, field)?;
}
if let Some(field) = self.item_type.as_ref() {
    write!(writer, r#""itemtype="{}""#, field)?;
}
if let Some(field) = self.lang.as_ref() {
    write!(writer, r#""lang="{}""#, field)?;
}
if let Some(field) = self.nonce.as_ref() {
    write!(writer, r#""nonce="{}""#, field)?;
}
if let Some(field) = self.popover.as_ref() {
    write!(writer, r#""popover="{}""#, field)?;
}
if let Some(field) = self.spellcheck.as_ref() {
    write!(writer, r#""spellcheck="{}""#, field)?;
}
if let Some(field) = self.style.as_ref() {
    write!(writer, r#""style="{}""#, field)?;
}
if let Some(field) = self.tab_index.as_ref() {
    write!(writer, r#""tabindex="{}""#, field)?;
}
if let Some(field) = self.title.as_ref() {
    write!(writer, r#""title="{}""#, field)?;
}
if let Some(field) = self.translate.as_ref() {
    write!(writer, r#""translate="{}""#, field)?;
}
write!(writer, ">")?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</button>")?;
        
        Ok(())
    }
}
