use oxc_allocator::Allocator;
use oxc_ast::ast::{Expression, JSXExpressionContainer};
use std::collections::HashMap;

/// Information about a dynamic attribute that needs runtime processing
#[derive(Debug, Clone)]
pub struct DynamicAttribute {
    pub name: String,
    pub expression: String, // We'll store the expression as a string for now
    pub is_style_object: bool,
    pub is_class_list: bool,
    pub requires_effect: bool,
}

/// Information about dynamic text insertions
#[derive(Debug, Clone)]
pub struct TextInsertion {
    pub expression: String,
    pub position: InsertionPosition,
}

/// Position information for text insertions
#[derive(Debug, Clone)]
pub enum InsertionPosition {
    BeforeNode(String), // Insert before this node reference
    AfterNode(String),  // Insert after this node reference
    AtEnd,              // Insert at the end (null position)
}

/// Template information including both static HTML and dynamic parts
#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub html: String,
    pub dynamic_attributes: Vec<DynamicAttribute>,
    pub text_insertions: Vec<TextInsertion>,
    pub has_dynamic_content: bool,
}

pub struct JSXTransformer<'a> {
    template_counter: usize,
    #[allow(dead_code)]
    allocator: &'a Allocator,
    pub templates: HashMap<String, String>,
    /// Enhanced template information for dynamic processing
    pub template_info: HashMap<String, TemplateInfo>,
    /// Required runtime imports for current transformation
    pub required_imports: std::collections::HashSet<String>,
}

impl<'a> JSXTransformer<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self {
            template_counter: 0,
            allocator,
            templates: HashMap::new(),
            template_info: HashMap::new(),
            required_imports: std::collections::HashSet::new(),
        }
    }

    pub fn get_templates(&self) -> &HashMap<String, String> {
        &self.templates
    }

    pub fn create_template_declaration(&self, template_name: &str, template_html: &str) -> String {
        format!(
            "var {} = /*#__PURE__*/ _$template(`{}`);",
            template_name, template_html
        )
    }

    fn get_next_template_name(&mut self) -> String {
        self.template_counter += 1;
        if self.template_counter == 1 {
            "_tmpl$".to_string()
        } else {
            format!("_tmpl${}", self.template_counter)
        }
    }

    /// Transform a JSX element into a dom-expressions template call
    pub fn transform_jsx_element(
        &mut self,
        element: &oxc_ast::ast::JSXElement,
    ) -> Result<String, TransformError> {
        // Extract static template structure and collect dynamic attributes and text insertions
        let (template_html, dynamic_attrs, text_insertions) =
            self.extract_template_with_dynamics_and_text(element);

        // Generate unique template name
        let template_name = self.get_next_template_name();

        // Store template for later code generation
        self.templates
            .insert(template_name.clone(), template_html.clone());

        // Store enhanced template info
        let template_info = TemplateInfo {
            html: template_html,
            dynamic_attributes: dynamic_attrs.clone(),
            text_insertions: text_insertions.clone(),
            has_dynamic_content: !dynamic_attrs.is_empty() || !text_insertions.is_empty(),
        };
        self.template_info
            .insert(template_name.clone(), template_info);

        // Generate appropriate call (static or dynamic)
        if dynamic_attrs.is_empty() && text_insertions.is_empty() {
            Ok(format!("{}()", template_name))
        } else {
            Ok(self.generate_dynamic_wrapper_with_text(
                &template_name,
                &dynamic_attrs,
                &text_insertions,
            ))
        }
    }

    /// Extract template HTML from a JSX element (legacy method)
    fn extract_template(&self, element: &oxc_ast::ast::JSXElement) -> String {
        let (html, _) = self.extract_template_with_dynamics(element);
        html
    }

    /// Extract template HTML and collect dynamic attributes from a JSX element
    fn extract_template_with_dynamics(
        &self,
        element: &oxc_ast::ast::JSXElement,
    ) -> (String, Vec<DynamicAttribute>) {
        use oxc_ast::ast::{
            JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild, JSXElementName,
        };

        let mut dynamic_attributes = Vec::new();

        match &element.opening_element.name {
            JSXElementName::Identifier(ident) => {
                let tag_name = &ident.name;
                let mut html = String::new();

                // Start opening tag
                html.push('<');
                html.push_str(tag_name);

                // Add attributes
                for attr in &element.opening_element.attributes {
                    match attr {
                        JSXAttributeItem::Attribute(attr) => {
                            if let JSXAttributeName::Identifier(name_ident) = &attr.name {
                                let attr_name = &name_ident.name;

                                // Convert 'for' to 'for' (keep as is in template)
                                let attr_name = if attr_name == "for" { "for" } else { attr_name };

                                match &attr.value {
                                    Some(JSXAttributeValue::StringLiteral(lit)) => {
                                        html.push(' ');
                                        html.push_str(attr_name);
                                        html.push('=');
                                        // Check if quotes are needed (dom-expressions style)
                                        let value = &lit.value;
                                        if self.needs_quotes(value) {
                                            html.push('"');
                                            html.push_str(value);
                                            html.push('"');
                                        } else {
                                            html.push_str(value);
                                        }
                                    }
                                    Some(JSXAttributeValue::ExpressionContainer(
                                        expr_container,
                                    )) => {
                                        // Extract expression as string - this is a simplified approach
                                        let expression_str =
                                            self.extract_expression_string(expr_container);

                                        // Create dynamic attribute info
                                        let is_style_object =
                                            attr_name == "style" && expression_str.starts_with('{');
                                        let is_class_list = attr_name == "classList";
                                        let requires_effect =
                                            self.expression_requires_effect(&expression_str);

                                        let dynamic_attr = DynamicAttribute {
                                            name: attr_name.to_string(),
                                            expression: expression_str,
                                            is_style_object,
                                            is_class_list,
                                            requires_effect,
                                        };

                                        dynamic_attributes.push(dynamic_attr);

                                        // Don't add to template HTML - will be handled dynamically
                                    }
                                    None => {
                                        // Boolean attribute
                                        html.push(' ');
                                        html.push_str(attr_name);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {} // Handle other attribute types if needed
                    }
                }

                // Check if self-closing
                if element.closing_element.is_none() && element.children.is_empty() {
                    html.push('>');
                    return (html, dynamic_attributes);
                }

                html.push('>');

                // Add children
                for child in &element.children {
                    match child {
                        JSXChild::Text(text) => {
                            let text_content = text.value.trim();
                            if !text_content.is_empty() {
                                html.push_str(text_content);
                            }
                        }
                        JSXChild::Element(child_element) => {
                            let (child_html, _child_dynamics) =
                                self.extract_template_with_dynamics(child_element);
                            html.push_str(&child_html);
                            // TODO: Merge child dynamics with current dynamics
                        }
                        JSXChild::ExpressionContainer(_expr_container) => {
                            // For now, skip dynamic expressions in templates
                            // This will be enhanced in future iterations
                        }
                        _ => {} // Handle other child types if needed
                    }
                }

                // Add closing tag if not self-closing
                if element.closing_element.is_some() {
                    html.push_str("</");
                    html.push_str(tag_name);
                    html.push('>');
                }

                (html, dynamic_attributes)
            }
            _ => ("<!-- unsupported JSX -->".to_string(), dynamic_attributes),
        }
    }

    /// Extract template HTML, dynamic attributes, and text insertions from a JSX element
    fn extract_template_with_dynamics_and_text(
        &self,
        element: &oxc_ast::ast::JSXElement,
    ) -> (String, Vec<DynamicAttribute>, Vec<TextInsertion>) {
        use oxc_ast::ast::{
            JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild, JSXElementName,
        };

        let mut dynamic_attributes = Vec::new();
        let mut text_insertions = Vec::new();

        match &element.opening_element.name {
            JSXElementName::Identifier(ident) => {
                let tag_name = &ident.name;
                let mut html = String::new();

                // Start opening tag
                html.push('<');
                html.push_str(tag_name);

                // Add attributes (same as existing logic)
                for attr in &element.opening_element.attributes {
                    match attr {
                        JSXAttributeItem::Attribute(attr) => {
                            if let JSXAttributeName::Identifier(name_ident) = &attr.name {
                                let attr_name = &name_ident.name;
                                let attr_name = if attr_name == "for" { "for" } else { attr_name };

                                match &attr.value {
                                    Some(JSXAttributeValue::StringLiteral(lit)) => {
                                        html.push(' ');
                                        html.push_str(attr_name);
                                        html.push('=');
                                        let value = &lit.value;
                                        if self.needs_quotes(value) {
                                            html.push('"');
                                            html.push_str(value);
                                            html.push('"');
                                        } else {
                                            html.push_str(value);
                                        }
                                    }
                                    Some(JSXAttributeValue::ExpressionContainer(
                                        expr_container,
                                    )) => {
                                        let expression_str =
                                            self.extract_expression_string(expr_container);
                                        let is_style_object =
                                            attr_name == "style" && expression_str.starts_with('{');
                                        let is_class_list = attr_name == "classList";
                                        let requires_effect =
                                            self.expression_requires_effect(&expression_str);

                                        let dynamic_attr = DynamicAttribute {
                                            name: attr_name.to_string(),
                                            expression: expression_str,
                                            is_style_object,
                                            is_class_list,
                                            requires_effect,
                                        };
                                        dynamic_attributes.push(dynamic_attr);
                                    }
                                    None => {
                                        html.push(' ');
                                        html.push_str(attr_name);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // Check if self-closing
                if element.closing_element.is_none() && element.children.is_empty() {
                    html.push('>');
                    return (html, dynamic_attributes, text_insertions);
                }

                html.push('>');

                // Process children with text interpolation support
                self.process_children_with_text_interpolation(
                    &element.children,
                    &mut html,
                    &mut text_insertions,
                );

                // Add closing tag if not self-closing
                if element.closing_element.is_some() {
                    html.push_str("</");
                    html.push_str(tag_name);
                    html.push('>');
                }

                (html, dynamic_attributes, text_insertions)
            }
            _ => (
                "<!-- unsupported JSX -->".to_string(),
                dynamic_attributes,
                text_insertions,
            ),
        }
    }

    /// Process JSX children and handle text interpolation
    fn process_children_with_text_interpolation(
        &self,
        children: &[oxc_ast::ast::JSXChild],
        html: &mut String,
        text_insertions: &mut Vec<TextInsertion>,
    ) {
        use oxc_ast::ast::JSXChild;

        let mut has_leading_text = false;
        let mut insertion_counter = 0;

        for (index, child) in children.iter().enumerate() {
            match child {
                JSXChild::Text(text) => {
                    let text_content = text.value.as_str();
                    let trimmed = text_content.trim();

                    if !trimmed.is_empty() {
                        html.push_str(trimmed);
                        has_leading_text = true;
                    } else if !text_content.is_empty() {
                        // Handle whitespace-only text nodes
                        let normalized_space =
                            if text_content.contains('\n') || text_content.len() > 1 {
                                " "
                            } else {
                                text_content
                            };
                        html.push_str(normalized_space);
                        has_leading_text = true;
                    }
                }
                JSXChild::ExpressionContainer(expr_container) => {
                    // This is a text interpolation - extract the expression
                    let expression_str = self.extract_expression_string(expr_container.as_ref());

                    // Add marker to template for insertion point
                    if index == children.len() - 1 {
                        // Last expression - insert at end
                        text_insertions.push(TextInsertion {
                            expression: expression_str,
                            position: InsertionPosition::AtEnd,
                        });
                    } else if !has_leading_text && index == 0 {
                        // First expression with no leading text - insert before first node
                        text_insertions.push(TextInsertion {
                            expression: expression_str,
                            position: InsertionPosition::BeforeNode("_el$.firstChild".to_string()),
                        });
                    } else {
                        // Expression in the middle - need placeholder
                        html.push_str(" ");
                        text_insertions.push(TextInsertion {
                            expression: expression_str,
                            position: InsertionPosition::AtEnd,
                        });
                    }

                    insertion_counter += 1;
                }
                JSXChild::Element(child_element) => {
                    let (child_html, _child_dynamics, _child_insertions) =
                        self.extract_template_with_dynamics_and_text(child_element);
                    html.push_str(&child_html);
                }
                _ => {}
            }
        }
    }

    /// Extract expression as string (improved for Phase 2)
    fn extract_expression_string(&self, expr_container: &JSXExpressionContainer) -> String {
        use oxc_ast::ast::JSXExpression;

        match &expr_container.expression {
            JSXExpression::Identifier(ident) => ident.name.to_string(),
            JSXExpression::StringLiteral(str_lit) => format!("\"{}\"", str_lit.value),
            JSXExpression::NumericLiteral(num_lit) => num_lit.value.to_string(),
            JSXExpression::BinaryExpression(_) => "expr".to_string(), // Placeholder for complex expressions
            JSXExpression::CallExpression(_) => "expr()".to_string(), // Placeholder for function calls
            _ => "expr".to_string(), // Fallback for other expression types
        }
    }

    /// Check if attribute value needs quotes (simple heuristic)
    fn needs_quotes(&self, value: &str) -> bool {
        // Don't quote simple alphanumeric values, but quote values with spaces or special chars
        value.contains(' ') || value.contains('"') || value.contains('\'') || value.is_empty()
    }

    /// Add a dynamic attribute for later processing
    pub fn add_dynamic_attribute(&mut self, attr_name: &str, expression: &str) {
        // Analyze the attribute to determine processing strategy
        let is_style_object = attr_name == "style" && expression.starts_with('{');
        let is_class_list = attr_name == "classList";
        let requires_effect = self.expression_requires_effect(expression);

        let _dynamic_attr = DynamicAttribute {
            name: attr_name.to_string(),
            expression: expression.to_string(),
            is_style_object,
            is_class_list,
            requires_effect,
        };

        // For now, we'll store this in a temporary way
        // TODO: Associate with specific template
        self.required_imports.insert("setAttribute".to_string());
        if requires_effect {
            self.required_imports.insert("effect".to_string());
        }
        if is_style_object {
            self.required_imports.insert("style".to_string());
        }
        if is_class_list {
            self.required_imports.insert("classList".to_string());
        }
    }

    /// Determine if an expression requires an effect wrapper
    fn expression_requires_effect(&self, expression: &str) -> bool {
        // Simple heuristic: if expression contains function calls, it probably needs an effect
        expression.contains('(') && expression.contains(')')
    }

    /// Generate runtime imports needed for current transformation
    pub fn get_required_imports(&self) -> Vec<String> {
        let mut imports = Vec::new();

        // Always need template
        imports.push("template as _$template".to_string());

        for import in &self.required_imports {
            match import.as_str() {
                "setAttribute" => imports.push("setAttribute as _$setAttribute".to_string()),
                "effect" => imports.push("effect as _$effect".to_string()),
                "style" => imports.push("style as _$style".to_string()),
                "classList" => imports.push("classList as _$classList".to_string()),
                "insert" => imports.push("insert as _$insert".to_string()),
                _ => {}
            }
        }

        imports
    }

    /// Generate IIFE wrapper for dynamic attribute processing
    pub fn generate_dynamic_wrapper(
        &self,
        template_name: &str,
        dynamic_attrs: &[DynamicAttribute],
    ) -> String {
        if dynamic_attrs.is_empty() {
            return format!("{}()", template_name);
        }

        let mut wrapper = String::new();
        wrapper.push_str("(() => {\n");
        wrapper.push_str(&format!("  var _el$ = {}();\n", template_name));

        // Generate dynamic attribute assignments
        for (_index, attr) in dynamic_attrs.iter().enumerate() {
            match attr.name.as_str() {
                "style" if attr.is_style_object => {
                    if attr.requires_effect {
                        wrapper.push_str(&format!(
                            "  _$effect(_$p => _$style(_el$, {}, _$p));\n",
                            attr.expression
                        ));
                    } else {
                        wrapper.push_str(&format!("  _$style(_el$, {});\n", attr.expression));
                    }
                }
                "classList" => {
                    wrapper.push_str(&format!("  _$classList(_el$, {});\n", attr.expression));
                }
                _ => {
                    if attr.requires_effect {
                        wrapper.push_str(&format!(
                            "  _$effect(() => _$setAttribute(_el$, \"{}\", {}));\n",
                            attr.name, attr.expression
                        ));
                    } else {
                        wrapper.push_str(&format!(
                            "  _$setAttribute(_el$, \"{}\", {});\n",
                            attr.name, attr.expression
                        ));
                    }
                }
            }
        }

        wrapper.push_str("  return _el$;\n");
        wrapper.push_str("})()");
        wrapper
    }

    /// Generate IIFE wrapper for dynamic attribute processing and text insertions
    pub fn generate_dynamic_wrapper_with_text(
        &mut self,
        template_name: &str,
        dynamic_attrs: &[DynamicAttribute],
        text_insertions: &[TextInsertion],
    ) -> String {
        if dynamic_attrs.is_empty() && text_insertions.is_empty() {
            return format!("{}()", template_name);
        }

        let mut wrapper = String::new();
        wrapper.push_str("(() => {\n");
        wrapper.push_str(&format!("  var _el$ = {}();\n", template_name));

        // Generate text insertion calls first
        for insertion in text_insertions {
            match &insertion.position {
                InsertionPosition::AtEnd => {
                    wrapper.push_str(&format!(
                        "  _$insert(_el$, {}, null);\n",
                        insertion.expression
                    ));
                }
                InsertionPosition::BeforeNode(node_ref) => {
                    wrapper.push_str(&format!(
                        "  _$insert(_el$, {}, {});\n",
                        insertion.expression, node_ref
                    ));
                }
                InsertionPosition::AfterNode(node_ref) => {
                    wrapper.push_str(&format!(
                        "  _$insert(_el$, {}, {}.nextSibling);\n",
                        insertion.expression, node_ref
                    ));
                }
            }
        }

        // Generate dynamic attribute assignments
        for (_index, attr) in dynamic_attrs.iter().enumerate() {
            match attr.name.as_str() {
                "style" if attr.is_style_object => {
                    if attr.requires_effect {
                        wrapper.push_str(&format!(
                            "  _$effect(_$p => _$style(_el$, {}, _$p));\n",
                            attr.expression
                        ));
                    } else {
                        wrapper.push_str(&format!("  _$style(_el$, {});\n", attr.expression));
                    }
                }
                "classList" => {
                    wrapper.push_str(&format!("  _$classList(_el$, {});\n", attr.expression));
                }
                _ => {
                    if attr.requires_effect {
                        wrapper.push_str(&format!(
                            "  _$effect(() => _$setAttribute(_el$, \"{}\", {}));\n",
                            attr.name, attr.expression
                        ));
                    } else {
                        wrapper.push_str(&format!(
                            "  _$setAttribute(_el$, \"{}\", {});\n",
                            attr.name, attr.expression
                        ));
                    }
                }
            }
        }

        wrapper.push_str("  return _el$;\n");
        wrapper.push_str("})()");
        wrapper
    }
}

#[derive(Debug)]
pub enum TransformError {
    #[allow(dead_code)]
    NotImplemented(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsx_transformer_creation() {
        let allocator = Allocator::default();
        let transformer = JSXTransformer::new(&allocator);
        assert_eq!(transformer.template_counter, 0);
    }
}
