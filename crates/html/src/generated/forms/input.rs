pub mod element {
    /// The HTML `<input>` element
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input)
    #[doc(alias = "input")]
    #[non_exhaustive]
    pub struct Input {
        sys: html_sys::forms::Input,
        _children: Vec<super::child::InputChild>,
    }
    impl Input {
        /// Get the value of the `accept` attribute
        pub fn accept(&self) -> std::option::Option<&str> {
            self.sys.accept.as_deref()
        }
        /// Set the value of the `accept` attribute
        pub fn set_accept(&mut self, value: std::option::Option<String>) {
            self.sys.accept = value;
        }
        /// Get the value of the `alt` attribute
        pub fn alt(&self) -> std::option::Option<&str> {
            self.sys.alt.as_deref()
        }
        /// Set the value of the `alt` attribute
        pub fn set_alt(&mut self, value: std::option::Option<String>) {
            self.sys.alt = value;
        }
        /// Get the value of the `autocomplete` attribute
        pub fn autocomplete(&self) -> std::option::Option<&str> {
            self.sys.autocomplete.as_deref()
        }
        /// Set the value of the `autocomplete` attribute
        pub fn set_autocomplete(&mut self, value: std::option::Option<String>) {
            self.sys.autocomplete = value;
        }
        /// Get the value of the `checked` attribute
        pub fn checked(&self) -> std::option::Option<&str> {
            self.sys.checked.as_deref()
        }
        /// Set the value of the `checked` attribute
        pub fn set_checked(&mut self, value: std::option::Option<String>) {
            self.sys.checked = value;
        }
        /// Get the value of the `dirname` attribute
        pub fn dirname(&self) -> std::option::Option<&str> {
            self.sys.dirname.as_deref()
        }
        /// Set the value of the `dirname` attribute
        pub fn set_dirname(&mut self, value: std::option::Option<String>) {
            self.sys.dirname = value;
        }
        /// Get the value of the `disabled` attribute
        pub fn disabled(&self) -> std::option::Option<&str> {
            self.sys.disabled.as_deref()
        }
        /// Set the value of the `disabled` attribute
        pub fn set_disabled(&mut self, value: std::option::Option<String>) {
            self.sys.disabled = value;
        }
        /// Get the value of the `form` attribute
        pub fn form(&self) -> std::option::Option<&str> {
            self.sys.form.as_deref()
        }
        /// Set the value of the `form` attribute
        pub fn set_form(&mut self, value: std::option::Option<String>) {
            self.sys.form = value;
        }
        /// Get the value of the `formaction` attribute
        pub fn formaction(&self) -> std::option::Option<&str> {
            self.sys.formaction.as_deref()
        }
        /// Set the value of the `formaction` attribute
        pub fn set_formaction(&mut self, value: std::option::Option<String>) {
            self.sys.formaction = value;
        }
        /// Get the value of the `formenctype` attribute
        pub fn formenctype(&self) -> std::option::Option<&str> {
            self.sys.formenctype.as_deref()
        }
        /// Set the value of the `formenctype` attribute
        pub fn set_formenctype(&mut self, value: std::option::Option<String>) {
            self.sys.formenctype = value;
        }
        /// Get the value of the `formmethod` attribute
        pub fn formmethod(&self) -> std::option::Option<&str> {
            self.sys.formmethod.as_deref()
        }
        /// Set the value of the `formmethod` attribute
        pub fn set_formmethod(&mut self, value: std::option::Option<String>) {
            self.sys.formmethod = value;
        }
        /// Get the value of the `formnovalidate` attribute
        pub fn formnovalidate(&self) -> std::option::Option<&str> {
            self.sys.formnovalidate.as_deref()
        }
        /// Set the value of the `formnovalidate` attribute
        pub fn set_formnovalidate(&mut self, value: std::option::Option<String>) {
            self.sys.formnovalidate = value;
        }
        /// Get the value of the `formtarget` attribute
        pub fn formtarget(&self) -> std::option::Option<&str> {
            self.sys.formtarget.as_deref()
        }
        /// Set the value of the `formtarget` attribute
        pub fn set_formtarget(&mut self, value: std::option::Option<String>) {
            self.sys.formtarget = value;
        }
        /// Get the value of the `height` attribute
        pub fn height(&self) -> std::option::Option<&str> {
            self.sys.height.as_deref()
        }
        /// Set the value of the `height` attribute
        pub fn set_height(&mut self, value: std::option::Option<String>) {
            self.sys.height = value;
        }
        /// Get the value of the `list` attribute
        pub fn list(&self) -> std::option::Option<&str> {
            self.sys.list.as_deref()
        }
        /// Set the value of the `list` attribute
        pub fn set_list(&mut self, value: std::option::Option<String>) {
            self.sys.list = value;
        }
        /// Get the value of the `max` attribute
        pub fn max(&self) -> std::option::Option<&str> {
            self.sys.max.as_deref()
        }
        /// Set the value of the `max` attribute
        pub fn set_max(&mut self, value: std::option::Option<String>) {
            self.sys.max = value;
        }
        /// Get the value of the `maxlength` attribute
        pub fn maxlength(&self) -> std::option::Option<&str> {
            self.sys.maxlength.as_deref()
        }
        /// Set the value of the `maxlength` attribute
        pub fn set_maxlength(&mut self, value: std::option::Option<String>) {
            self.sys.maxlength = value;
        }
        /// Get the value of the `min` attribute
        pub fn min(&self) -> std::option::Option<&str> {
            self.sys.min.as_deref()
        }
        /// Set the value of the `min` attribute
        pub fn set_min(&mut self, value: std::option::Option<String>) {
            self.sys.min = value;
        }
        /// Get the value of the `minlength` attribute
        pub fn minlength(&self) -> std::option::Option<&str> {
            self.sys.minlength.as_deref()
        }
        /// Set the value of the `minlength` attribute
        pub fn set_minlength(&mut self, value: std::option::Option<String>) {
            self.sys.minlength = value;
        }
        /// Get the value of the `multiple` attribute
        pub fn multiple(&self) -> std::option::Option<&str> {
            self.sys.multiple.as_deref()
        }
        /// Set the value of the `multiple` attribute
        pub fn set_multiple(&mut self, value: std::option::Option<String>) {
            self.sys.multiple = value;
        }
        /// Get the value of the `name` attribute
        pub fn name(&self) -> std::option::Option<&str> {
            self.sys.name.as_deref()
        }
        /// Set the value of the `name` attribute
        pub fn set_name(&mut self, value: std::option::Option<String>) {
            self.sys.name = value;
        }
        /// Get the value of the `pattern` attribute
        pub fn pattern(&self) -> std::option::Option<&str> {
            self.sys.pattern.as_deref()
        }
        /// Set the value of the `pattern` attribute
        pub fn set_pattern(&mut self, value: std::option::Option<String>) {
            self.sys.pattern = value;
        }
        /// Get the value of the `placeholder` attribute
        pub fn placeholder(&self) -> std::option::Option<&str> {
            self.sys.placeholder.as_deref()
        }
        /// Set the value of the `placeholder` attribute
        pub fn set_placeholder(&mut self, value: std::option::Option<String>) {
            self.sys.placeholder = value;
        }
        /// Get the value of the `readonly` attribute
        pub fn readonly(&self) -> std::option::Option<&str> {
            self.sys.readonly.as_deref()
        }
        /// Set the value of the `readonly` attribute
        pub fn set_readonly(&mut self, value: std::option::Option<String>) {
            self.sys.readonly = value;
        }
        /// Get the value of the `required` attribute
        pub fn required(&self) -> std::option::Option<&str> {
            self.sys.required.as_deref()
        }
        /// Set the value of the `required` attribute
        pub fn set_required(&mut self, value: std::option::Option<String>) {
            self.sys.required = value;
        }
        /// Get the value of the `size` attribute
        pub fn size(&self) -> std::option::Option<&str> {
            self.sys.size.as_deref()
        }
        /// Set the value of the `size` attribute
        pub fn set_size(&mut self, value: std::option::Option<String>) {
            self.sys.size = value;
        }
        /// Get the value of the `src` attribute
        pub fn src(&self) -> std::option::Option<&str> {
            self.sys.src.as_deref()
        }
        /// Set the value of the `src` attribute
        pub fn set_src(&mut self, value: std::option::Option<String>) {
            self.sys.src = value;
        }
        /// Get the value of the `step` attribute
        pub fn step(&self) -> std::option::Option<&str> {
            self.sys.step.as_deref()
        }
        /// Set the value of the `step` attribute
        pub fn set_step(&mut self, value: std::option::Option<String>) {
            self.sys.step = value;
        }
        /// Get the value of the `type` attribute
        pub fn type_(&self) -> std::option::Option<&str> {
            self.sys.type_.as_deref()
        }
        /// Set the value of the `type` attribute
        pub fn set_type_(&mut self, value: std::option::Option<String>) {
            self.sys.type_ = value;
        }
        /// Get the value of the `value` attribute
        pub fn value(&self) -> std::option::Option<&str> {
            self.sys.value.as_deref()
        }
        /// Set the value of the `value` attribute
        pub fn set_value(&mut self, value: std::option::Option<String>) {
            self.sys.value = value;
        }
        /// Get the value of the `width` attribute
        pub fn width(&self) -> std::option::Option<&str> {
            self.sys.width.as_deref()
        }
        /// Set the value of the `width` attribute
        pub fn set_width(&mut self, value: std::option::Option<String>) {
            self.sys.width = value;
        }
    }
    impl crate::HtmlElement for Input {}
    impl crate::FlowContent for Input {}
    impl crate::PhrasingContent for Input {}
    impl std::convert::Into<html_sys::forms::Input> for Input {
        fn into(self) -> html_sys::forms::Input {
            self.sys
        }
    }
    impl From<html_sys::forms::Input> for Input {
        fn from(sys: html_sys::forms::Input) -> Self {
            Self { sys, _children: vec![] }
        }
    }
}
pub mod child {
    /// The permitted child items for the `Input` element
    pub enum InputChild {
        /// The Abbreviation element
        Abbreviation(crate::generated::all::Abbreviation),
        /// The Address element
        Address(crate::generated::all::Address),
        /// The Anchor element
        Anchor(crate::generated::all::Anchor),
        /// The BidirectionalIsolate element
        BidirectionalIsolate(crate::generated::all::BidirectionalIsolate),
        /// The BidirectionalTextOverride element
        BidirectionalTextOverride(crate::generated::all::BidirectionalTextOverride),
        /// The BlockQuote element
        BlockQuote(crate::generated::all::BlockQuote),
        /// The Bold element
        Bold(crate::generated::all::Bold),
        /// The Button element
        Button(crate::generated::all::Button),
        /// The Cite element
        Cite(crate::generated::all::Cite),
        /// The Code element
        Code(crate::generated::all::Code),
        /// The Data element
        Data(crate::generated::all::Data),
        /// The DataList element
        DataList(crate::generated::all::DataList),
        /// The Definition element
        Definition(crate::generated::all::Definition),
        /// The DeletedText element
        DeletedText(crate::generated::all::DeletedText),
        /// The DescriptionList element
        DescriptionList(crate::generated::all::DescriptionList),
        /// The Details element
        Details(crate::generated::all::Details),
        /// The Dialog element
        Dialog(crate::generated::all::Dialog),
        /// The Division element
        Division(crate::generated::all::Division),
        /// The Emphasis element
        Emphasis(crate::generated::all::Emphasis),
        /// The Fieldset element
        Fieldset(crate::generated::all::Fieldset),
        /// The Figure element
        Figure(crate::generated::all::Figure),
        /// The Footer element
        Footer(crate::generated::all::Footer),
        /// The Form element
        Form(crate::generated::all::Form),
        /// The Header element
        Header(crate::generated::all::Header),
        /// The ImageMap element
        ImageMap(crate::generated::all::ImageMap),
        /// The Input element
        Input(crate::generated::all::Input),
        /// The InsertedText element
        InsertedText(crate::generated::all::InsertedText),
        /// The Italic element
        Italic(crate::generated::all::Italic),
        /// The KeyboardInput element
        KeyboardInput(crate::generated::all::KeyboardInput),
        /// The Label element
        Label(crate::generated::all::Label),
        /// The LineBreak element
        LineBreak(crate::generated::all::LineBreak),
        /// The LineBreakOpportunity element
        LineBreakOpportunity(crate::generated::all::LineBreakOpportunity),
        /// The MarkText element
        MarkText(crate::generated::all::MarkText),
        /// The Menu element
        Menu(crate::generated::all::Menu),
        /// The Meter element
        Meter(crate::generated::all::Meter),
        /// The OrderedList element
        OrderedList(crate::generated::all::OrderedList),
        /// The Output element
        Output(crate::generated::all::Output),
        /// The Paragraph element
        Paragraph(crate::generated::all::Paragraph),
        /// The PreformattedText element
        PreformattedText(crate::generated::all::PreformattedText),
        /// The Progress element
        Progress(crate::generated::all::Progress),
        /// The Quotation element
        Quotation(crate::generated::all::Quotation),
        /// The RubyAnnotation element
        RubyAnnotation(crate::generated::all::RubyAnnotation),
        /// The SampleOutput element
        SampleOutput(crate::generated::all::SampleOutput),
        /// The Script element
        Script(crate::generated::all::Script),
        /// The Select element
        Select(crate::generated::all::Select),
        /// The SideComment element
        SideComment(crate::generated::all::SideComment),
        /// The Slot element
        Slot(crate::generated::all::Slot),
        /// The Span element
        Span(crate::generated::all::Span),
        /// The StrikeThrough element
        StrikeThrough(crate::generated::all::StrikeThrough),
        /// The Strong element
        Strong(crate::generated::all::Strong),
        /// The SubScript element
        SubScript(crate::generated::all::SubScript),
        /// The SuperScript element
        SuperScript(crate::generated::all::SuperScript),
        /// The Table element
        Table(crate::generated::all::Table),
        /// The Template element
        Template(crate::generated::all::Template),
        /// The TextArea element
        TextArea(crate::generated::all::TextArea),
        /// The ThematicBreak element
        ThematicBreak(crate::generated::all::ThematicBreak),
        /// The Time element
        Time(crate::generated::all::Time),
        /// The Underline element
        Underline(crate::generated::all::Underline),
        /// The UnorderedList element
        UnorderedList(crate::generated::all::UnorderedList),
        /// The Variable element
        Variable(crate::generated::all::Variable),
    }
}
