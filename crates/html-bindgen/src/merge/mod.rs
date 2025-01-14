//! Unify all the parsed sources into a single, final source.

use std::collections::HashMap;

use crate::parse::{Attribute, ParsedCategory, ParsedElement, ParsedInterface, ParsedRelationship};
use crate::Result;
use serde::{Deserialize, Serialize};

/// The final source of truth we used to generate code from.
///
/// Created by combining all of the previously parsed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergedElement {
    pub tag_name: String,
    pub struct_name: String,
    pub submodule_name: String,
    pub mdn_link: String,
    pub has_global_attributes: bool,
    pub has_closing_tag: bool,
    pub attributes: Vec<Attribute>,
    pub dom_interface: String,
    pub content_categories: Vec<MergedCategory>,
    pub permitted_child_elements: Vec<String>,
}

/// Each element in HTML falls into zero or more categories that group elements
/// with similar characteristics together.
///
/// Unlike `ParsedCategory`, this can no longer hold any child elements.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MergedCategory {
    Metadata,
    Flow,
    Sectioning,
    Heading,
    Phrasing,
    Embedded,
    Interactive,
    Palpable,
    ScriptSupporting,
    Transparent,
}

impl From<ParsedCategory> for MergedCategory {
    fn from(value: ParsedCategory) -> Self {
        match value {
            ParsedCategory::Metadata => Self::Metadata,
            ParsedCategory::Flow => Self::Flow,
            ParsedCategory::Sectioning => Self::Sectioning,
            ParsedCategory::Heading => Self::Heading,
            ParsedCategory::Phrasing => Self::Phrasing,
            ParsedCategory::Embedded => Self::Embedded,
            ParsedCategory::Interactive => Self::Interactive,
            ParsedCategory::Palpable => Self::Palpable,
            ParsedCategory::ScriptSupporting => Self::ScriptSupporting,
            ParsedCategory::Transparent => Self::Transparent,
        }
    }
}

pub fn merge(
    parsed_elements: impl Iterator<Item = Result<ParsedElement>>,
    parsed_interfaces: impl Iterator<Item = Result<ParsedInterface>>,
) -> Result<Vec<MergedElement>> {
    let mut elements = HashMap::new();
    for el in parsed_elements {
        let el = el?;
        let key = el.struct_name.clone();
        elements.insert(key, el);
    }

    let mut interfaces = HashMap::new();
    for interface in parsed_interfaces {
        let interface = interface?;
        let key = interface.name.clone();
        interfaces.insert(key, interface);
    }

    let by_content_type = categorize_elements(&elements);
    let mut children_map = children_per_element(&elements, &by_content_type);
    insert_text_content(&elements, &mut children_map);
    let attributes_map = merge_attributes(&elements, &interfaces);

    let mut output = vec![];
    for (_, el) in elements.into_iter() {
        let mut permitted_child_elements = children_map.get(&el.struct_name).unwrap().clone();
        permitted_child_elements.dedup();
        let attributes = attributes_map.get(&el.struct_name).unwrap().clone();
        output.push(MergedElement {
            tag_name: el.tag_name,
            struct_name: el.struct_name,
            submodule_name: el.submodule_name,
            mdn_link: el.mdn_link,
            has_global_attributes: el.has_global_attributes,
            has_closing_tag: el.has_closing_tag,
            dom_interface: el.dom_interface,
            content_categories: convert_parsed_categories(&el.content_categories),
            attributes,
            permitted_child_elements,
        })
    }
    Ok(output)
}

/// In order to correctly handle `PhrasingContent` we add one more item to the
/// mix: `Text`, which in later stages we'll replace with a Rust string type.
fn insert_text_content(
    elements: &HashMap<String, ParsedElement>,
    children_map: &mut HashMap<String, Vec<String>>,
) {
    for (_, parent_el) in elements {
        let has_phrasing = parent_el
            .permitted_content
            .contains(&ParsedRelationship::Category(ParsedCategory::Phrasing));
        let has_transparent = parent_el
            .permitted_content
            .contains(&ParsedRelationship::Category(ParsedCategory::Transparent));
        let has_flow = parent_el
            .permitted_content
            .contains(&ParsedRelationship::Category(ParsedCategory::Flow));

        if has_phrasing || has_flow || has_transparent {
            children_map
                .get_mut(&parent_el.struct_name)
                .unwrap()
                .push("Text".to_owned());
        }
    }
}

/// Create a hashmap that keeps track of which elements
/// belong to which content type.
///
/// Also extract the individual elements related to each content type, and keep
/// them for later insertion into the parent types.
fn categorize_elements(
    elements: &HashMap<String, ParsedElement>,
) -> HashMap<ParsedCategory, Vec<String>> {
    let mut output = HashMap::new();
    for (name, el) in elements {
        for cat in &el.content_categories {
            let vec: &mut Vec<_> = output.entry(cat.clone()).or_default();
            vec.push(name.clone());
        }
    }

    // make sure we add the "transparent" and "script-supporting" content categories
    let _ = output.entry(ParsedCategory::Transparent).or_default();
    let _ = output.entry(ParsedCategory::ScriptSupporting).or_default();

    output
}

/// Which child elements belong to the parent element?
/// We decide which element is a valid child element in the following way:
///  1. We look at an element's `permitted_child_elements` field, to figure out
///     which categories can be taken as child elements.
///  2. Then we look at each element in the category, and see whether it can take
///     the parent element as its parent.
///  3. If the parent can have the child, and the child can have the parent, we
///     add it to the parent's child list.
fn children_per_element(
    elements: &HashMap<String, ParsedElement>,
    by_content_type: &HashMap<ParsedCategory, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    // Because not all elements will have children,
    // we create empty lists for all elements first.
    let mut output = elements
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();

    // First we start by iterating over all elements.
    for (_, parent_el) in elements {
        // Then we take a look at which elements they can take as children.
        for child_relationship in &parent_el.permitted_content {
            match child_relationship {
                ParsedRelationship::Element(child_el_name) => {
                    // Check that the child can have the current element as a
                    // parent.
                    if child_el_name != "Text" {
                        let child_el = elements.get(child_el_name).unwrap();
                        if !child_can_have_parent(child_el, parent_el) {
                            continue;
                        }
                    }
                    output
                        .get_mut(&parent_el.struct_name)
                        .unwrap()
                        .push(child_el_name.to_owned());
                }
                ParsedRelationship::Category(child_category) => {
                    for child_el_name in by_content_type.get(&child_category).unwrap() {
                        let child_el = elements.get(child_el_name).unwrap();
                        if child_can_have_parent(child_el, parent_el) {
                            output
                                .get_mut(&parent_el.struct_name)
                                .unwrap()
                                .push(child_el.struct_name.to_owned());
                        }
                    }
                }
            }
        }
    }

    // Check whether
    fn child_can_have_parent(child: &ParsedElement, proposed_parent: &ParsedElement) -> bool {
        // If the parent allows "transparent" content, then all children are valid.
        // The spec will often add more contstraints, but we don't yet have type
        // states so we ignore them for now.
        if proposed_parent
            .permitted_content
            .contains(&ParsedRelationship::Category(ParsedCategory::Transparent))
        {
            return true;
        }

        // Check whether the child can have the parent element as a parent.
        for parent_relationship in &child.permitted_parents {
            match parent_relationship {
                ParsedRelationship::Element(parent_el_name) => {
                    if parent_el_name == &proposed_parent.struct_name {
                        return true;
                    }
                }
                ParsedRelationship::Category(parent_category) => {
                    if proposed_parent
                        .content_categories
                        .contains(&parent_category)
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    // Some elements belong to more than one category, so they can end
    // up in the list more than once. This makes sure that the list
    // is always in order, and only contains unique elements.
    output.iter_mut().for_each(|(_, value)| {
        value.dedup();
        value.sort()
    });

    output
}

/// Merge WebIDL attributes into the regular attributes list.
fn merge_attributes(
    elements: &HashMap<String, ParsedElement>,
    interfaces: &HashMap<String, ParsedInterface>,
) -> HashMap<String, Vec<Attribute>> {
    let mut output = elements
        .iter()
        .map(|(name, _)| (name.clone(), vec![]))
        .collect::<HashMap<_, _>>();

    let interface_map = interfaces
        .iter()
        .map(|(name, interface)| {
            let map = interface
                .attributes
                .iter()
                .map(|attr| (attr.name.to_lowercase().clone(), attr.clone()))
                .collect::<HashMap<String, Attribute>>();
            (name.clone(), map)
        })
        .collect::<HashMap<String, _>>();

    for (_, el) in elements {
        let interface = match interface_map.get(&el.dom_interface) {
            Some(interface) => interface,
            None => {
                let vec = output.entry(el.struct_name.clone()).or_default();
                vec.extend(el.attributes.iter().cloned());
                continue;
            }
        };

        for attr in &el.attributes {
            let attr = match interface.get(&attr.name) {
                Some(other) => Attribute {
                    name: attr.name.clone(),
                    description: attr.description.clone(),
                    field_name: other.field_name.clone(),
                    ty: other.ty.clone(),
                },
                None => attr.clone(),
            };
            let vec = output.entry(el.struct_name.clone()).or_default();
            vec.push(attr);
        }
    }
    for (_, vec) in output.iter_mut() {
        vec.dedup();
    }
    output
}

/// Take a list of parsed categories and output a list of merged categories + a
/// list of child elements.
fn convert_parsed_categories(categories: &[ParsedCategory]) -> Vec<MergedCategory> {
    categories.into_iter().cloned().map(Into::into).collect()
}
