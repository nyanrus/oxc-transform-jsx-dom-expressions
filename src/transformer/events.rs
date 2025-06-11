/// Event handling transformation for Solid.js
/// 
/// This module handles the transformation of event handlers:
/// - onClick={handler} -> delegated events
/// - onMount, onCleanup lifecycle events
/// - Custom event transformations

use oxc_ast::ast::{JSXAttribute, Expression};

pub struct EventTransformer;

impl EventTransformer {
    pub fn new() -> Self {
        Self
    }

    /// Check if an attribute is an event handler
    pub fn is_event_attribute(attr_name: &str) -> bool {
        attr_name.starts_with("on") && attr_name.len() > 2
    }

    /// Check if an event should be delegated
    pub fn is_delegated_event(event_name: &str) -> bool {
        matches!(
            event_name.to_lowercase().as_str(),
            "click" | "input" | "change" | "submit" | "focus" | "blur" | "keydown" | "keyup"
        )
    }

    /// Transform event handler attributes
    pub fn transform_event_handler(&self, attribute: &JSXAttribute) -> Result<EventHandling, EventError> {
        let attr_name = self.get_attribute_name(attribute)?;
        
        if !Self::is_event_attribute(&attr_name) {
            return Err(EventError::NotAnEvent(attr_name));
        }

        let event_name = self.extract_event_name(&attr_name)?;
        let handler_expr = self.extract_handler_expression(attribute)?;
        
        // Convert Expression to String representation for now
        // TODO: Implement proper expression to string conversion
        let handler = format!("/* handler expression */");

        if Self::is_delegated_event(&event_name) {
            Ok(EventHandling::Delegated {
                event_name,
                handler,
            })
        } else {
            Ok(EventHandling::Direct {
                event_name,
                handler,
            })
        }
    }

    /// Transform lifecycle events (onMount, onCleanup)
    pub fn transform_lifecycle_event(&self, _attribute: &JSXAttribute) -> Result<LifecycleEvent, EventError> {
        // TODO: Handle Solid.js lifecycle events
        // onMount={callback} -> createEffect(callback)
        // onCleanup={callback} -> onCleanup(callback)
        Err(EventError::NotImplemented("Lifecycle events"))
    }

    /// Extract attribute name from JSX attribute
    fn get_attribute_name(&self, _attribute: &JSXAttribute) -> Result<String, EventError> {
        // TODO: Extract attribute name from JSXAttribute
        // Handle both regular and namespaced attribute names
        Ok("onClick".to_string()) // Placeholder
    }

    /// Extract event name from attribute name (onClick -> click)
    fn extract_event_name(&self, attr_name: &str) -> Result<String, EventError> {
        if attr_name.len() <= 2 || !attr_name.starts_with("on") {
            return Err(EventError::InvalidEventName(attr_name.to_string()));
        }

        let event_name = &attr_name[2..]; // Remove "on" prefix
        Ok(event_name.to_lowercase())
    }

    /// Extract handler expression from JSX attribute
    fn extract_handler_expression(&self, _attribute: &JSXAttribute) -> Result<Expression, EventError> {
        // TODO: Extract the expression from JSXAttribute value
        todo!("Extract handler expression")
    }

    /// Generate delegated event binding code
    pub fn generate_delegated_binding(&self, event_name: &str, _handler: &Expression) -> Result<String, EventError> {
        // TODO: Generate code for delegated event handling
        // This will be used in template generation
        Ok(format!("/* delegated {} */", event_name))
    }

    /// Generate direct event binding code  
    pub fn generate_direct_binding(&self, event_name: &str, _handler: &Expression) -> Result<String, EventError> {
        // TODO: Generate code for direct event handling
        Ok(format!("addEventListener('{}', {})", event_name, "handler"))
    }
}

impl Default for EventTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum EventHandling {
    /// Events that use Solid's delegation system
    Delegated {
        event_name: String,
        handler: String, // Use String instead of Expression for now
    },
    /// Events that are bound directly to elements
    Direct {
        event_name: String,
        handler: String, // Use String instead of Expression for now
    },
}

#[derive(Debug)]
pub enum LifecycleEvent {
    Mount(String), // Use String instead of Expression for now
    Cleanup(String), // Use String instead of Expression for now
}

#[derive(Debug)]
pub enum EventError {
    NotImplemented(&'static str),
    NotAnEvent(String),
    InvalidEventName(String),
    InvalidHandler(String),
    UnsupportedEvent(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_attribute_detection() {
        assert!(EventTransformer::is_event_attribute("onClick"));
        assert!(EventTransformer::is_event_attribute("onSubmit"));
        assert!(EventTransformer::is_event_attribute("onMount"));
        assert!(!EventTransformer::is_event_attribute("class"));
        assert!(!EventTransformer::is_event_attribute("id"));
        assert!(!EventTransformer::is_event_attribute("on")); // Too short
    }

    #[test]
    fn test_delegated_event_detection() {
        assert!(EventTransformer::is_delegated_event("click"));
        assert!(EventTransformer::is_delegated_event("input"));
        assert!(EventTransformer::is_delegated_event("submit"));
        assert!(!EventTransformer::is_delegated_event("mount"));
        assert!(!EventTransformer::is_delegated_event("resize"));
    }

    #[test]
    fn test_event_name_extraction() {
        let transformer = EventTransformer::new();
        assert_eq!(transformer.extract_event_name("onClick").unwrap(), "click");
        assert_eq!(transformer.extract_event_name("onSubmit").unwrap(), "submit");
        assert_eq!(transformer.extract_event_name("onKeyDown").unwrap(), "keydown");
        
        assert!(transformer.extract_event_name("class").is_err());
        assert!(transformer.extract_event_name("on").is_err());
    }
}
