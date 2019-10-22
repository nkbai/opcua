//! Contains the implementation of `Method` and `MethodBuilder`.

use opcua_types::service_types::DataTypeAttributes;

use crate::address_space::{base::Base, node::NodeBase, node::Node};

node_builder_impl!(DataTypeBuilder, DataType);

/// A `DataType` is a type of node within the `AddressSpace`.
#[derive(Debug)]
pub struct DataType {
    base: Base,
    is_abstract: bool,
}

impl Default for DataType {
    fn default() -> Self {
        Self {
            base: Base::new(NodeClass::DataType, &NodeId::null(), "", ""),
            is_abstract: false,
        }
    }
}
//node_base_impl!(DataType);
use opcua_types::*;
use opcua_types::status_code::StatusCode;
use opcua_types::service_types::NodeClass;
use crate::address_space::node::NodeType;
impl Into<NodeType> for DataType {
    fn into(self) -> NodeType { NodeType::DataType(Box::new(self)) }
}
impl NodeBase for DataType {
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

impl Node for DataType {
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        match attribute_id {
            AttributeId::IsAbstract => Some(Variant::from(self.is_abstract()).into()),
            _ => self.base.get_attribute_max_age(attribute_id, max_age)
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        match attribute_id {
            AttributeId::IsAbstract => {
                if let Variant::Boolean(v) = value {
                    self.set_is_abstract(v);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            _ => self.base.set_attribute(attribute_id, value)
        }
    }
}

impl DataType {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, is_abstract: bool) -> DataType
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        DataType {
            base: Base::new(NodeClass::DataType, node_id, browse_name, display_name),
            is_abstract,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: DataTypeAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(AttributesMask::DISPLAY_NAME | AttributesMask::IS_ABSTRACT) {
            let mut node = Self::new(node_id, browse_name, attributes.display_name, attributes.is_abstract);
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
            error!("DataType cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }
}
