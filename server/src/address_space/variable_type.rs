//! Contains the implementation of `VariableType` and `VariableTypeBuilder`.

use std::convert::TryFrom;

use opcua_types::service_types::VariableTypeAttributes;

use crate::address_space::{base::Base, node::NodeBase, node::Node};

node_builder_impl!(VariableTypeBuilder, VariableType);

node_builder_impl_generates_event!(VariableTypeBuilder);
node_builder_impl_subtype!(VariableTypeBuilder);

/// A `VariableType` is a type of node within the `AddressSpace`.
#[derive(Debug)]
pub struct VariableType {
    base: Base,
    data_type: NodeId,
    is_abstract: bool,
    value_rank: i32,
    value: Option<DataValue>,
    array_dimensions: Option<Vec<u32>>,
}

impl Default for VariableType {
    fn default() -> Self {
        Self {
            base: Base::new(NodeClass::VariableType, &NodeId::null(), "", ""),
            data_type: NodeId::null(),
            is_abstract: false,
            value_rank: -1,
            value: None,
            array_dimensions: None,
        }
    }
}
//node_base_impl!(VariableType);
use opcua_types::*;
use opcua_types::status_code::StatusCode;
use opcua_types::service_types::NodeClass;
use crate::address_space::node::NodeType;
impl Into<NodeType> for VariableType {
    fn into(self) -> NodeType { NodeType::VariableType(Box::new(self)) }
}
impl NodeBase for VariableType {
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

impl Node for VariableType {
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        match attribute_id {
            AttributeId::Value => self.value(),
            AttributeId::DataType => Some(Variant::from(self.data_type()).into()),
            AttributeId::IsAbstract => Some(Variant::from(self.is_abstract()).into()),
            AttributeId::ValueRank => Some(Variant::from(self.value_rank()).into()),
            // Optional attributes
            AttributeId::ArrayDimensions => self.array_dimensions().map(|v| Variant::from(v).into()),
            _ => self.base.get_attribute_max_age(attribute_id, max_age)
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        match attribute_id {
            AttributeId::DataType => {
                if let Variant::NodeId(v) = value {
                    self.set_data_type(*v);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::IsAbstract => {
                if let Variant::Boolean(v) = value {
                    self.set_is_abstract(v);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::ValueRank => {
                if let Variant::Int32(v) = value {
                    self.set_value_rank(v);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::Value => {
                self.set_value(value);
                Ok(())
            }
            AttributeId::ArrayDimensions => {
                let array_dimensions = <Vec<u32>>::try_from(&value);
                if let Ok(array_dimensions) = array_dimensions {
                    self.set_array_dimensions(&array_dimensions);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            _ => self.base.set_attribute(attribute_id, value)
        }
    }
}

impl VariableType {
    pub fn new<R, S>(node_id: &NodeId, browse_name: R, display_name: S, data_type: NodeId, is_abstract: bool, value_rank: i32) -> VariableType
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        VariableType {
            base: Base::new(NodeClass::VariableType, node_id, browse_name, display_name),
            data_type,
            is_abstract,
            value_rank,
            value: None,
            array_dimensions: None,
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: VariableTypeAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mandatory_attributes = AttributesMask::DISPLAY_NAME | AttributesMask::IS_ABSTRACT |
            AttributesMask::DATA_TYPE | AttributesMask::VALUE_RANK;
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(mandatory_attributes) {
            let mut node = Self::new(node_id, browse_name, attributes.display_name,
                                     attributes.data_type, attributes.is_abstract, attributes.value_rank);
            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            if mask.contains(AttributesMask::VALUE) {
                node.set_value(attributes.value);
            }
            if mask.contains(AttributesMask::ARRAY_DIMENSIONS) {
                node.set_array_dimensions(attributes.array_dimensions.unwrap().as_slice());
            }
            Ok(node)
        } else {
            error!("VariableType cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    pub fn data_type(&self) -> NodeId {
        self.data_type.clone()
    }

    pub fn set_data_type<T>(&mut self, data_type: T) where T: Into<NodeId> {
        self.data_type = data_type.into();
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }

    pub fn value_rank(&self) -> i32 {
        self.value_rank
    }

    pub fn set_value_rank(&mut self, value_rank: i32) {
        self.value_rank = value_rank;
    }

    pub fn array_dimensions(&self) -> Option<Vec<u32>> {
        self.array_dimensions.clone()
    }

    pub fn set_array_dimensions(&mut self, array_dimensions: &[u32]) {
        self.array_dimensions = Some(array_dimensions.to_vec());
    }

    pub fn value(&self) -> Option<DataValue> {
        self.value.clone()
    }

    pub fn set_value<V>(&mut self, value: V) where V: Into<Variant> {
        self.value = Some(DataValue::new(value));
    }
}