//! Contains the implementation of `ReferenceType` and `ReferenceTypeBuilder`.

use opcua_types::service_types::ReferenceTypeAttributes;

use crate::address_space::{base::Base, node::NodeBase, node::Node};

node_builder_impl!(ReferenceTypeBuilder, ReferenceType);
node_builder_impl_subtype!(ReferenceTypeBuilder);

/// A `ReferenceType` is a type of node within the `AddressSpace`.
#[derive(Debug)]
pub struct ReferenceType {
    base: Base,
    symmetric: bool,
    is_abstract: bool,
    inverse_name: Option<LocalizedText>,
}

impl Default for ReferenceType {
    fn default() -> Self {
        Self {
            base: Base::new(NodeClass::VariableType, &NodeId::null(), "", ""),
            symmetric: false,
            is_abstract: false,
            inverse_name: None,
        }
    }
}

//node_base_impl!(ReferenceType);
use opcua_types::*;
use opcua_types::status_code::StatusCode;
use opcua_types::service_types::NodeClass;
use crate::address_space::node::NodeType;
impl Into<NodeType> for ReferenceType {
    fn into(self) -> NodeType { NodeType::ReferenceType(Box::new(self)) }
}
impl NodeBase for ReferenceType {
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
impl Node for ReferenceType {
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        match attribute_id {
            AttributeId::Symmetric => Some(Variant::from(self.symmetric()).into()),
            AttributeId::IsAbstract => Some(Variant::from(self.is_abstract()).into()),
            AttributeId::InverseName => self.inverse_name().map(|v| Variant::from(v).into()),
            _ => self.base.get_attribute_max_age(attribute_id, max_age)
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        match attribute_id {
            AttributeId::Symmetric => {
                if let Variant::Boolean(v) = value {
                    self.symmetric = v;
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::IsAbstract => {
                if let Variant::Boolean(v) = value {
                    self.is_abstract = v;
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::InverseName => {
                if let Variant::LocalizedText(v) = value {
                    self.inverse_name = Some(*v);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            _ => self.base.set_attribute(attribute_id, value)
        }
    }
}

impl ReferenceType {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, inverse_name: Option<LocalizedText>, symmetric: bool, is_abstract: bool) -> ReferenceType
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        ReferenceType {
            base: Base::new(NodeClass::ReferenceType, node_id, browse_name, display_name),
            symmetric,
            is_abstract,
            inverse_name,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: ReferenceTypeAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mandatory_attributes = AttributesMask::DISPLAY_NAME | AttributesMask::IS_ABSTRACT | AttributesMask::SYMMETRIC;
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(mandatory_attributes) {
            let mut node = Self::new(node_id, browse_name, attributes.display_name, None, false, false);
            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            if mask.contains(AttributesMask::IS_ABSTRACT) {
                node.set_is_abstract(attributes.is_abstract);
            }
            if mask.contains(AttributesMask::SYMMETRIC) {
                node.set_symmetric(attributes.is_abstract);
            }
            if mask.contains(AttributesMask::INVERSE_NAME) {
                node.set_inverse_name(attributes.inverse_name);
            }
            Ok(node)
        } else {
            error!("ReferenceType cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    pub fn symmetric(&self) -> bool {
        self.symmetric
    }

    pub fn set_symmetric(&mut self, symmetric: bool) {
        self.symmetric = symmetric;
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }

    pub fn inverse_name(&self) -> Option<LocalizedText> {
        self.inverse_name.clone()
    }

    pub fn set_inverse_name(&mut self, inverse_name: LocalizedText) {
        self.inverse_name = Some(inverse_name);
    }
}
