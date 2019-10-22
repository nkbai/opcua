//! Contains the implementation of `Object` and `ObjectBuilder`.

use opcua_types::service_types::ObjectAttributes;

use crate::address_space::{
    EventNotifier,
    base::Base, node::NodeBase, node::Node,
};

//node_builder_impl!(ObjectBuilder, Object);
use crate::address_space::{
    address_space::AddressSpace,
    references::ReferenceDirection,
};
pub struct ObjectBuilder {
    node: Object,
    references: Vec<(NodeId, NodeId, ReferenceDirection)>,
}
impl ObjectBuilder {
    pub fn new<T, S>(node_id: &NodeId, browse_name: T, display_name: S) -> Self
        where T: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        Self {
            node: Object::default(),
            references: Vec::with_capacity(10),
        }
            .node_id(node_id.clone())
            .browse_name(browse_name)
            .display_name(display_name)
    }

    pub fn get_node_id(&self) -> NodeId {
        self.node.node_id()
    }

    fn node_id(mut self, node_id: NodeId) -> Self {
        let _ = self.node.base.set_node_id(node_id);
        self
    }

    fn browse_name<V>(mut self, browse_name: V) -> Self where V: Into<QualifiedName> {
        let _ = self.node.base.set_browse_name(browse_name);
        self
    }

    fn display_name<V>(mut self, display_name: V) -> Self where V: Into<LocalizedText> {
        self.node.set_display_name(display_name.into());
        self
    }


    pub fn is_valid(&self) -> bool {
        self.node.is_valid()
    }


    pub fn description<V>(mut self, description: V) -> Self where V: Into<LocalizedText> {
        self.node.set_description(description.into());
        self
    }


    pub fn reference<T>(mut self, node_id: T, reference_type_id: ReferenceTypeId, reference_direction: ReferenceDirection) -> Self
        where T: Into<NodeId>
    {
        self.references.push((node_id.into(), reference_type_id.into(), reference_direction));
        self
    }


    pub fn organizes<T>(self, organizes_id: T) -> Self where T: Into<NodeId> {
        self.reference(organizes_id, ReferenceTypeId::Organizes, ReferenceDirection::Forward)
    }


    pub fn organized_by<T>(self, organized_by_id: T) -> Self where T: Into<NodeId> {
        self.reference(organized_by_id, ReferenceTypeId::Organizes, ReferenceDirection::Inverse)
    }


    pub fn build(self) -> Object {
        if self.is_valid() {
            self.node
        } else {
            panic!("The node is not valid, node id = {:?}", self.node.base.node_id());
        }
    }


    pub fn insert(self, address_space: &mut AddressSpace) -> bool {
        if self.is_valid() {
            if !self.references.is_empty() {
                let references = self.references.iter().map(|v| {
                    (&v.0, &v.1, v.2)
                }).collect::<Vec<_>>();
                address_space.insert(self.node, Some(references.as_slice()))
            } else {
                address_space.insert::<Object, ReferenceTypeId>(self.node, None)
            }
        } else {
            panic!("The node is not valid, node id = {:?}", self.node.base.node_id());
        }
    }
}
//node_builder_impl_component_of!(ObjectBuilder);
impl ObjectBuilder {
    pub fn component_of<T>(self, component_of_id: T) -> Self where T: Into<NodeId> {
        self.reference(component_of_id, ReferenceTypeId::HasComponent, ReferenceDirection::Inverse)
    }

    pub fn has_component<T>(self, has_component_id: T) -> Self where T: Into<NodeId> {
        self.reference(has_component_id, ReferenceTypeId::HasComponent, ReferenceDirection::Forward)
    }
}
//node_builder_impl_property_of!(ObjectBuilder);
impl ObjectBuilder {
    pub fn has_property<T>(self, has_component_id: T) -> Self where T: Into<NodeId> {
        self.reference(has_component_id, ReferenceTypeId::HasProperty, ReferenceDirection::Forward)
    }

    pub fn property_of<T>(self, component_of_id: T) -> Self where T: Into<NodeId> {
        self.reference(component_of_id, ReferenceTypeId::HasProperty, ReferenceDirection::Inverse)
    }
}

impl ObjectBuilder {
    pub fn is_folder(self) -> Self {
        self.has_type_definition(ObjectTypeId::FolderType)
    }

    pub fn event_notifier(mut self, event_notifier: EventNotifier) -> Self {
        self.node.set_event_notifier(event_notifier);
        self
    }

    pub fn has_type_definition<T>(self, type_id: T) -> Self where T: Into<NodeId> {
        self.reference(type_id, ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward)
    }

    pub fn has_event_source<T>(self, source_id: T) -> Self where T: Into<NodeId> {
        self.reference(source_id, ReferenceTypeId::HasEventSource, ReferenceDirection::Forward)
    }
}

/// An `Object` is a type of node within the `AddressSpace`.
#[derive(Debug)]
pub struct Object {
    base: Base,
    event_notifier: EventNotifier,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            base: Base::new(NodeClass::Object, &NodeId::null(), "", ""),
            event_notifier: EventNotifier::empty(),
        }
    }
}
//node_base_impl!(Object);
use opcua_types::*;
use opcua_types::status_code::StatusCode;
use opcua_types::service_types::NodeClass;
use crate::address_space::node::NodeType;
impl Into<NodeType> for Object {
    fn into(self) -> NodeType { NodeType::Object(Box::new(self)) }
}
impl NodeBase for Object {
    fn node_class(&self) -> NodeClass {
        self.base.node_class()
    }

    fn node_id(&self) -> NodeId {
        self.base.node_id()
    }

    fn browse_name(&self) -> QualifiedName {
        self.base.browse_name()
    }

    fn display_name(&self) -> LocalizedText {
        self.base.display_name()
    }

    fn set_display_name(&mut self, display_name: LocalizedText) {
        self.base.set_display_name(display_name);
    }

    fn description(&self) -> Option<LocalizedText> {
        self.base.description()
    }

    fn set_description(&mut self, description: LocalizedText) {
        self.base.set_description(description);
    }

    fn write_mask(&self) -> Option<WriteMask> {
        self.base.write_mask()
    }

    fn set_write_mask(&mut self, write_mask: WriteMask) {
        self.base.set_write_mask(write_mask);
    }

    fn user_write_mask(&self) -> Option<WriteMask> {
        self.base.user_write_mask()
    }

    fn set_user_write_mask(&mut self, user_write_mask: WriteMask) {
        self.base.set_user_write_mask(user_write_mask)
    }
}

impl Node for Object {
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        match attribute_id {
            AttributeId::EventNotifier => Some(Variant::from(self.event_notifier().bits()).into()),
            _ => self.base.get_attribute_max_age(attribute_id, max_age)
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        match attribute_id {
            AttributeId::EventNotifier => {
                if let Variant::Byte(v) = value {
                    self.set_event_notifier(EventNotifier::from_bits_truncate(v));
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            _ => self.base.set_attribute(attribute_id, value)
        }
    }
}

impl Object {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, event_notifier: EventNotifier) -> Object
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        Object {
            base: Base::new(NodeClass::Object, node_id, browse_name, display_name),
            event_notifier,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: ObjectAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mandatory_attributes = AttributesMask::DISPLAY_NAME | AttributesMask::EVENT_NOTIFIER;

        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(mandatory_attributes) {
            let event_notifier = EventNotifier::from_bits_truncate(attributes.event_notifier);
            let mut node = Self::new(node_id, browse_name, attributes.display_name, event_notifier);
            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            Ok(node)
        } else {
            error!("Object cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    pub fn event_notifier(&self) -> EventNotifier {
        self.event_notifier
    }

    pub fn set_event_notifier(&mut self, event_notifier: EventNotifier) {
        self.event_notifier = event_notifier;
    }
}

