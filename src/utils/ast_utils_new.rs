/// AST utility functions for working with OXC AST nodes
/// 
/// This module provides helper functions for:
/// - Creating and manipulating AST nodes
/// - Traversing AST structures
/// - Common transformations

use oxc_ast::ast::{JSXElement, JSXChild, JSXAttribute};

/// Helper functions for JSX element manipulation
pub struct AstUtils;

impl AstUtils {
    /// Check if a JSX element has dynamic content
    pub fn has_dynamic_content(element: &JSXElement) -> bool {
        // 動的属性または動的子要素があればtrue
        for attr in &element.opening_element.attributes {
            if let JSXAttributeItem::Attribute(jsx_attr) = attr {
                if let Some(JSXAttributeValue::ExpressionContainer(_)) = &jsx_attr.value {
                    return true;
                }
            } else if let JSXAttributeItem::SpreadAttribute(_) = attr {
                return true;
            }
        }
        for child in &element.children {
            match child {
                JSXChild::ExpressionContainer(_) => return true,
                JSXChild::Element(e) => {
                    if AstUtils::has_dynamic_content(e) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    /// Count the number of dynamic expressions in a JSX element
    pub fn count_dynamic_expressions(element: &JSXElement) -> usize {
        let mut count = 0;
        for attr in &element.opening_element.attributes {
            if let JSXAttributeItem::Attribute(jsx_attr) = attr {
                if let Some(JSXAttributeValue::ExpressionContainer(_)) = &jsx_attr.value {
                    count += 1;
                }
            } else if let JSXAttributeItem::SpreadAttribute(_) = attr {
                count += 1;
            }
        }
        for child in &element.children {
            match child {
                JSXChild::ExpressionContainer(_) => count += 1,
                JSXChild::Element(e) => count += AstUtils::count_dynamic_expressions(e),
                _ => {}
            }
        }
        count
    }

    /// Extract the tag name from a JSX element
    pub fn get_element_name(element: &JSXElement) -> Option<String> {
        use oxc_ast::ast::JSXElementName;
        match &element.opening_element.name {
            JSXElementName::Identifier(ident) => Some(ident.name.to_string()),
            JSXElementName::IdentifierReference(ident) => Some(ident.name.to_string()),
            JSXElementName::NamespacedName(ns) => Some(format!("{}:{}", ns.namespace.name, ns.name.name)),
            JSXElementName::MemberExpression(_) => None, // 複雑な場合は未対応
            JSXElementName::ThisExpression(_) => None,
        }
    }

    /// Check if a JSX element is self-closing
    pub fn is_self_closing(element: &JSXElement) -> bool {
        element.opening_element.self_closing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_utils() {
        // Placeholder tests - these will be implemented later
        // when we have proper JSX element creation utilities
        assert!(true);
    }
}

// コメントやテスト名の "SolidTransform" などを "DomExpressionsTransform" にリネーム
