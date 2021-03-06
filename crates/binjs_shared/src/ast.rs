use std::fmt::*;

/// The path followed when walking an AST.
///
/// Designed to be used both to quickly find out how to contextually handle
/// a specific node and for error-reporting.
///
/// ```
/// use binjs_shared::ast::Path;
///
/// let mut path = Path::new();
/// assert!(path.get(0).is_none());
///
/// // Once we have entered both an interface and a field, `path.get(0)` will be `Some`.
/// path.enter_interface("Interface 1");
/// assert!(path.get(0).is_none());
///
/// path.enter_field("Field 1");
///
/// {
///   let item = path.get(0).unwrap();
///   assert_eq!(item.field, "Field 1");
///   assert_eq!(item.interface, "Interface 1");
/// }
///
/// path.enter_interface("Interface 2");
/// path.enter_field("Field 2");
///
/// {
///   let item = path.get(0).unwrap();
///   assert_eq!(item.field, "Field 2");
///   assert_eq!(item.interface, "Interface 2");
/// }
/// {
///   let item = path.get(1).unwrap();
///   assert_eq!(item.field, "Field 1");
///   assert_eq!(item.interface, "Interface 1");
/// }
///
/// // We need to exit the field before exiting the interface.
/// path.exit_field("Field 2"); // Exiting the wrong field would panic.
/// path.exit_interface("Interface 2"); // Exiting the wrong interface would panic.
/// path.exit_field("Field 1"); // Exiting the wrong field would panic.
/// path.exit_interface("Interface 1"); // Exiting the wrong interface would panic.
/// ```

pub struct Path<I, F> where I: Debug + PartialEq, F: Debug + PartialEq {
    /// Some(foo) if we have entered interface foo but no field yet.
    /// Otherwise, None.
    interface: Option<I>,
    items: Vec<PathItem<I, F>>,
}

#[derive(Debug)]
pub struct PathItem<I, F> where I: Debug + PartialEq, F: Debug + PartialEq {
    pub interface: I,
    pub field: F,
}
impl<I, F> Debug for Path<I, F> where I: Debug + PartialEq, F: Debug + PartialEq {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use itertools::Itertools;
        write!(f, "[{items}{more}]",
            items = self.items.iter()
                .map(|item| format!("{:?}.{:?}", item.interface, item.field))
                .format(" > "),
            more = if let Some(ref interface) = self.interface {
                format!(" > {:?}", interface)
            } else {
                "".to_string()
            }
        )
    }
}
impl<I, F> Path<I, F> where I: Debug + PartialEq, F: Debug + PartialEq {
    /// Create an empty `Path`.
    pub fn new() -> Self {
        Self {
            interface: None,
            items: vec![],
        }
    }

    /// Enter an interface.
    ///
    /// All calls to `enter_interface` MUST be balanced with calls
    /// to `exit_interface`.
    pub fn enter_interface(&mut self, node: I) {
        debug_assert!(self.interface.is_none());
        self.interface = Some(node);
    }
    pub fn exit_interface(&mut self, node: I) {
        let interface = self.interface.take()
            .expect("Could not exit_interface if we're not in an interface");
        debug_assert!(node == interface);
    }
    pub fn enter_field(&mut self, field: F) {
        let interface = self.interface.take()
            .unwrap();
        self.items.push(PathItem {
            interface,
            field,
        });
    }
    pub fn exit_field(&mut self, field: F) {
        debug_assert!(self.interface.is_none());
        let PathItem {
            interface,
            field: prev
        } = self.items.pop()
            .expect("Could not exit_field from an empty ASTath");
        debug_assert!(prev == field);
        self.interface = Some(interface);
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&PathItem<I, F>> {
        if index >= self.len() {
            return None;
        }
        Some(&self.items[self.len() - index - 1])
    }
}
