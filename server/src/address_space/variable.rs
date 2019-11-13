//! Contains the implementation of `Variable` and `VariableBuilder`.

use std::sync::{Arc, Mutex};
use std::convert::{Into, TryFrom};

use opcua_types::node_ids::DataTypeId;

use crate::{
    callbacks::{AttributeGetter, AttributeSetter},
    address_space::{
        AccessLevel, UserAccessLevel,
        AttrFnGetter, AttrFnSetter,
        base::Base,
        node::{NodeBase, Node},
    },
};
use opcua_types::service_types::VariableAttributes;

// This is a builder object for constructing variable nodes programmatically.

//node_builder_impl!(VariableBuilder, Variable);
use crate::address_space::{
    address_space::AddressSpace,
    references::ReferenceDirection,
};

pub struct VariableBuilder {
    node: Variable,
    references: Vec<(NodeId, NodeId, ReferenceDirection)>,
}

impl VariableBuilder {
    pub fn new<T, S>(node_id: &NodeId, browse_name: T, display_name: S) -> Self
        where T: Into<QualifiedName>,
              S: Into<LocalizedText>,
    {
        Self {
            node: Variable::default(),
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


    pub fn build(self) -> Variable {
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
                address_space.insert::<Variable, ReferenceTypeId>(self.node, None)
            }
        } else {
            panic!("The node is not valid, node id = {:?}", self.node.base.node_id());
        }
    }
}

//node_builder_impl_component_of!(VariableBuilder);

impl VariableBuilder {
    pub fn component_of<T>(self, component_of_id: T) -> Self where T: Into<NodeId> {
        self.reference(component_of_id, ReferenceTypeId::HasComponent, ReferenceDirection::Inverse)
    }

    pub fn has_component<T>(self, has_component_id: T) -> Self where T: Into<NodeId> {
        self.reference(has_component_id, ReferenceTypeId::HasComponent, ReferenceDirection::Forward)
    }
}

//node_builder_impl_property_of!(VariableBuilder);
impl VariableBuilder {
    pub fn has_property<T>(self, has_component_id: T) -> Self where T: Into<NodeId> {
        self.reference(has_component_id, ReferenceTypeId::HasProperty, ReferenceDirection::Forward)
    }

    pub fn property_of<T>(self, component_of_id: T) -> Self where T: Into<NodeId> {
        self.reference(component_of_id, ReferenceTypeId::HasProperty, ReferenceDirection::Inverse)
    }
}

impl VariableBuilder {
    /// Sets the value of the variable.
    pub fn value<V>(mut self, value: V) -> Self where V: Into<Variant> {
        let _ = self.node.set_value(value);
        self
    }

    /// Sets the data type of the variable.
    pub fn data_type<T>(mut self, data_type: T) -> Self where T: Into<NodeId> {
        self.node.set_data_type(data_type);
        self
    }

    /// Sets the historizing flag for the variable.
    pub fn historizing(mut self, historizing: bool) -> Self {
        self.node.set_historizing(historizing);
        self
    }

    /// Sets the access level for the variable.
    pub fn access_level(mut self, access_level: AccessLevel) -> Self {
        self.node.set_access_level(access_level);
        self
    }

    /// Sets the user access level for the variable.
    pub fn user_access_level(mut self, user_access_level: UserAccessLevel) -> Self {
        self.node.set_user_access_level(user_access_level);
        self
    }

    /// Sets the value rank for the variable.
    pub fn value_rank(mut self, value_rank: i32) -> Self {
        self.node.set_value_rank(value_rank);
        self
    }

    /// Sets the array dimensions for the variable.
    pub fn array_dimensions(mut self, array_dimensions: &[u32]) -> Self {
        self.node.set_array_dimensions(array_dimensions);
        self
    }

    /// Makes the variable writable (by default it isn't)
    pub fn writable(mut self) -> Self {
        self.node.set_access_level(self.node.access_level() & AccessLevel::CURRENT_WRITE);
        self
    }

    /// Sets the minimum sampling interval for the variable.
    pub fn minimum_sampling_interval(mut self, minimum_sampling_interval: f64) -> Self {
        self.node.set_minimum_sampling_interval(minimum_sampling_interval);
        self
    }

    /// Sets a value getter function for the variable. Whenever the value of a variable
    /// needs to be fetched (e.g. from a monitored item subscription), this function will be called
    /// to get the value.
    pub fn value_getter<F>(mut self, getter: F) -> Self where
        F: FnMut(&NodeId, AttributeId, f64) -> Result<Option<DataValue>, StatusCode> + Send + 'static
    {
        self.node.set_value_getter(Arc::new(Mutex::new(AttrFnGetter::new(getter))));
        self
    }

    /// Sets a value setter function for the variable. Whenever the value of a variable is set via
    /// a service, this function will be called to set the value. It is up to the implementation
    /// to decide what to do if that happens.
    pub fn value_setter<F>(mut self, setter: F) -> Self where
        F: FnMut(&NodeId, AttributeId, DataValue) -> Result<(), StatusCode> + Send + 'static
    {
        self.node.set_value_setter(Arc::new(Mutex::new(AttrFnSetter::new(setter))));
        self
    }

    /// Add a reference to the variable indicating it has a type of another node.
    pub fn has_type_definition<T>(self, type_id: T) -> Self where T: Into<NodeId> {
        self.reference(type_id, ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward)
    }

    /// Add a reference to the variable indicating it has a modelling rule of another node.
    pub fn has_modelling_rule<T>(self, type_id: T) -> Self where T: Into<NodeId> {
        self.reference(type_id, ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward)
    }
}

// Note we use derivative builder macro so we can skip over the value getter / setter

/// A `Variable` is a type of node within the `AddressSpace`.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Variable {
    base: Base,
    /**
    数据类型表示为地址空间中的节点。 此属性包含一个-
    这种节点的N“eld,从而定义了 Value属性的数据类型
    */
    data_type: NodeId,
    /**
    指示服务器目前是杏收集了 Vau的历史。 AccewLeveJ
    属性不提供这些信息， 它仅指定是否有历史可用
    */
    historizing: bool,
    /**
    标识值是否是一个数组， 如果它是一个数组， 它允许
    指定数组的维度
    */
    value_rank: i32,
    /**
    变量的实际值。 该值的数据类型由DataType、ValueRank和AnayDimensions 厲性指定
    */
    value: DataValue,
    /**
一个位掩码Vdue属性的当前值是否可读可写， 以及
Vdue的历史是否可读和可改变
*/
    access_level: u8,
    ///与AccesLevd包含相同的信息， 但需耍考虑到用户的
    ///访问权限
    user_access_level: u8,
    /**
    这个可选属性允许指定数组的大小， 只能在值是一个数组
    时使用。 对尸数组的每一个维数对应的条目定义维度的长度
    */
    array_dimensions: Option<Vec<u32>>,
    /**
    这个可选属性提供信息指明OPC UA服务器需耍多长
    时间能检测到va”厲性的变化。 对于服务器不直接管理
    的值， 例如， 一个遐度传感器的温度.服务器可能需要
    扫描设备变化（轮询〉 ， 因此无法比最小时间间隔更快检
    測到变化
    */
    minimum_sampling_interval: Option<f64>,
    #[derivative(Debug = "ignore")]
    value_setter: Option<Arc<Mutex<dyn AttributeSetter + Send>>>,
    #[derivative(Debug = "ignore")]
    value_getter: Option<Arc<Mutex<dyn AttributeGetter + Send>>>,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            base: Base::new(NodeClass::Variable, &NodeId::null(), "", ""),
            data_type: NodeId::null(),
            historizing: false,
            value_rank: -1,
            value: Variant::Empty.into(),
            access_level: UserAccessLevel::CURRENT_READ.bits(),
            user_access_level: AccessLevel::CURRENT_READ.bits(),
            array_dimensions: None,
            minimum_sampling_interval: None,
            value_getter: None,
            value_setter: None,
        }
    }
}

//node_base_impl!(Variable);
use opcua_types::*;
use opcua_types::status_code::StatusCode;
use opcua_types::service_types::NodeClass;
use crate::address_space::node::NodeType;

impl Into<NodeType> for Variable {
    fn into(self) -> NodeType { NodeType::Variable(Box::new(self)) }
}

impl NodeBase for Variable {
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

impl Node for Variable {
    fn get_attribute_max_age(&self, attribute_id: AttributeId, max_age: f64) -> Option<DataValue> {
        match attribute_id {
            // Mandatory attributes
            AttributeId::Value => Some(self.value()),
            AttributeId::DataType => Some(Variant::from(self.data_type()).into()),
            AttributeId::Historizing => Some(Variant::from(self.historizing()).into()),
            AttributeId::ValueRank => Some(Variant::from(self.value_rank()).into()),
            AttributeId::AccessLevel => Some(Variant::from(self.access_level().bits()).into()),
            AttributeId::UserAccessLevel => Some(Variant::from(self.user_access_level().bits()).into()),
            // Optional attributes
            AttributeId::ArrayDimensions => self.array_dimensions().map(|v| Variant::from(v).into()),
            AttributeId::MinimumSamplingInterval => self.minimum_sampling_interval().map(|v| Variant::from(v).into()),
            _ => self.base.get_attribute_max_age(attribute_id, max_age)
        }
    }

    fn set_attribute(&mut self, attribute_id: AttributeId, value: Variant) -> Result<(), StatusCode> {
        match attribute_id {
            AttributeId::DataType => if let Variant::NodeId(v) = value {
                self.set_data_type(*v);
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            AttributeId::Historizing => if let Variant::Boolean(v) = value {
                self.set_historizing(v);
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            AttributeId::ValueRank => if let Variant::Int32(v) = value {
                self.set_value_rank(v);
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            AttributeId::Value => {
                self.set_value(value);
                Ok(())
            }
            AttributeId::AccessLevel => if let Variant::Byte(v) = value {
                self.set_access_level(AccessLevel::from_bits_truncate(v));
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            AttributeId::UserAccessLevel => if let Variant::Byte(v) = value {
                self.set_user_access_level(UserAccessLevel::from_bits_truncate(v));
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            AttributeId::ArrayDimensions => {
                let array_dimensions = <Vec<u32>>::try_from(&value);
                if let Ok(array_dimensions) = array_dimensions {
                    self.set_array_dimensions(&array_dimensions);
                    Ok(())
                } else {
                    Err(StatusCode::BadTypeMismatch)
                }
            }
            AttributeId::MinimumSamplingInterval => if let Variant::Double(v) = value {
                self.set_minimum_sampling_interval(v);
                Ok(())
            } else {
                Err(StatusCode::BadTypeMismatch)
            },
            _ => self.base.set_attribute(attribute_id, value)
        }
    }
}

impl Variable {
    /// Creates a new variable. Note that data type, value rank and historizing are mandatory
    /// attributes of the Variable but not required by the constructor. The data type and value rank
    /// are inferred from the value. Historizing is not supported so is always false. If the
    /// inferred types for data type or value rank are wrong, they may be explicitly set, or
    /// call `new_data_value()` instead.
    pub fn new<R, S, V>(node_id: &NodeId, browse_name: R, display_name: S, value: V) -> Variable
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              V: Into<Variant>
    {
        let value = value.into();
        let data_type = value.data_type();
        if let Some(data_type) = data_type {
            Variable::new_data_value(node_id, browse_name, display_name, data_type, value)
        } else {
            panic!("Data type cannot be inferred from the value, use another constructor such as new_data_value")
        }
    }

    pub fn from_attributes<S>(node_id: &NodeId, browse_name: S, attributes: VariableAttributes) -> Result<Self, ()>
        where S: Into<QualifiedName>
    {
        let mandatory_attributes = AttributesMask::DISPLAY_NAME | AttributesMask::ACCESS_LEVEL | AttributesMask::USER_ACCESS_LEVEL |
            AttributesMask::DATA_TYPE | AttributesMask::HISTORIZING | AttributesMask::VALUE | AttributesMask::VALUE_RANK;
        let mask = AttributesMask::from_bits(attributes.specified_attributes).ok_or(())?;
        if mask.contains(mandatory_attributes) {
            let mut node = Self::new_data_value(node_id, browse_name, attributes.display_name, attributes.data_type, attributes.value);
            node.set_value_rank(attributes.value_rank);
            node.set_historizing(attributes.historizing);
            node.set_access_level(AccessLevel::from_bits_truncate(attributes.access_level));
            node.set_user_access_level(UserAccessLevel::from_bits_truncate(attributes.user_access_level));

            if mask.contains(AttributesMask::DESCRIPTION) {
                node.set_description(attributes.description);
            }
            if mask.contains(AttributesMask::WRITE_MASK) {
                node.set_write_mask(WriteMask::from_bits_truncate(attributes.write_mask));
            }
            if mask.contains(AttributesMask::USER_WRITE_MASK) {
                node.set_user_write_mask(WriteMask::from_bits_truncate(attributes.user_write_mask));
            }
            if mask.contains(AttributesMask::ARRAY_DIMENSIONS) {
                node.set_array_dimensions(attributes.array_dimensions.unwrap().as_slice());
            }
            if mask.contains(AttributesMask::MINIMUM_SAMPLING_INTERVAL) {
                node.set_minimum_sampling_interval(attributes.minimum_sampling_interval);
            }
            Ok(node)
        } else {
            error!("Variable cannot be created from attributes - missing mandatory values");
            Err(())
        }
    }

    pub fn new_with_data_type<V>(node_id: &NodeId, browse_name: &str, display_name: &str, data_type: DataTypeId, value: V) -> Variable where V: Into<Variant> {
        Variable::new_data_value(node_id, browse_name, display_name, data_type, value)
    }

    /// Constructs a new variable with the specified id, name, type and value
    pub fn new_data_value<S, R, N, V>(node_id: &NodeId, browse_name: R, display_name: S, data_type: N, value: V) -> Variable
        where R: Into<QualifiedName>,
              S: Into<LocalizedText>,
              N: Into<NodeId>,
              V: Into<Variant>
    {
        let value = value.into();
        let array_dimensions = match value {
            Variant::Array(ref values) => Some(vec![values.len() as u32]),
            Variant::MultiDimensionArray(ref values) => {
                // Multidimensional arrays encode/decode dimensions with Int32 in Part 6, but arrayDimensions in Part 3
                // wants them as u32. Go figure... So convert Int32 to u32
                Some(values.dimensions.iter().map(|v| *v as u32).collect::<Vec<u32>>())
            }
            _ => None
        };

        let builder = VariableBuilder::new(node_id, browse_name, display_name)
            .user_access_level(UserAccessLevel::CURRENT_READ)
            .access_level(AccessLevel::CURRENT_READ)
            .data_type(data_type)
            .historizing(false)
            .value(value);

        // Set the array info
        let builder = if let Some(array_dimensions) = array_dimensions {
            builder.value_rank(array_dimensions.len() as i32).array_dimensions(&array_dimensions)
        } else {
            builder.value_rank(-1)
        };
        builder.build()
    }

    pub fn is_valid(&self) -> bool {
        self.base.is_valid()
    }

    pub fn value(&self) -> DataValue {
        if let Some(ref value_getter) = self.value_getter {
            let mut value_getter = value_getter.lock().unwrap();
            //如果错了怎么办呢?服务器就crash了?
            value_getter.get(&self.node_id(), AttributeId::Value, 0f64).unwrap().unwrap()
        } else {
            self.value.clone().into()
        }
    }

    /// Sets the variable's `Variant` value. The timestamps for the change are updated to now.
    pub fn set_value<V>(&mut self, value: V) where V: Into<Variant> {
        let value = value.into();
        // The value set to the value getter
        if let Some(ref value_setter) = self.value_setter {
            let mut value_setter = value_setter.lock().unwrap();
            let _ = value_setter.set(&self.node_id(), AttributeId::Value, value.into());
        } else {
            let now = DateTime::now();
            self.set_value_direct(value, &now, &now);
        }
    }

    /// Sets the variable's `DataValue`
    pub fn set_value_direct<V>(&mut self, value: V, server_timestamp: &DateTime, source_timestamp: &DateTime) where V: Into<Variant> {
        self.value.value = Some(value.into());
        self.value.server_timestamp = Some(server_timestamp.clone());
        self.value.source_timestamp = Some(source_timestamp.clone());
    }

    /// Sets a getter function that will be called to get the value of this variable.
    pub fn set_value_getter(&mut self, value_getter: Arc<Mutex<dyn AttributeGetter + Send>>) {
        self.value_getter = Some(value_getter);
    }

    /// Sets a setter function that will be called to set the value of this variable.
    pub fn set_value_setter(&mut self, value_setter: Arc<Mutex<dyn AttributeSetter + Send>>) {
        self.value_setter = Some(value_setter);
    }

    /// Gets the minimum sampling interval, if the attribute was set
    pub fn minimum_sampling_interval(&self) -> Option<f64> {
        self.minimum_sampling_interval.clone()
    }

    /// Sets the minimum sampling interval
    ///
    /// Specifies in milliseconds how fast the server can reasonably sample the value for changes
    ///
    /// The value 0 means server is to monitor the value continuously. The value -1 means indeterminate.
    pub fn set_minimum_sampling_interval(&mut self, minimum_sampling_interval: f64) {
        self.minimum_sampling_interval = Some(minimum_sampling_interval);
    }

    /// Test if the variable is readable. This will be called by services before getting the value
    /// of the node.
    pub fn is_readable(&self) -> bool {
        self.access_level().contains(AccessLevel::CURRENT_READ)
    }

    /// Test if the variable is writable. This will be called by services before setting the value
    /// on the node.
    pub fn is_writable(&self) -> bool {
        self.access_level().contains(AccessLevel::CURRENT_WRITE)
    }

    /// Sets the variable writable state.
    pub fn set_writable(&mut self, writable: bool) {
        let mut access_level = self.access_level();
        if writable {
            access_level.insert(AccessLevel::CURRENT_WRITE);
        } else {
            access_level.remove(AccessLevel::CURRENT_WRITE);
        }
        self.set_access_level(access_level);
    }

    /// Returns the access level of the variable.
    pub fn access_level(&self) -> AccessLevel {
        AccessLevel::from_bits_truncate(self.access_level)
    }

    /// Sets the access level of the variable.
    pub fn set_access_level(&mut self, access_level: AccessLevel) {
        self.access_level = access_level.bits();
    }

    /// Test if the variable is user readable.
    pub fn is_user_readable(&self) -> bool {
        self.user_access_level().contains(UserAccessLevel::CURRENT_READ)
    }

    /// Test if the variable is user writable.
    pub fn is_user_writable(&self) -> bool {
        self.user_access_level().contains(UserAccessLevel::CURRENT_WRITE)
    }

    /// Returns the user access level of the variable.
    pub fn user_access_level(&self) -> UserAccessLevel {
        UserAccessLevel::from_bits_truncate(self.user_access_level)
    }

    /// Set the user access level of the variable.
    pub fn set_user_access_level(&mut self, user_access_level: UserAccessLevel) {
        self.user_access_level = user_access_level.bits();
    }

    pub fn value_rank(&self) -> i32 {
        self.value_rank
    }

    pub fn set_value_rank(&mut self, value_rank: i32) {
        self.value_rank = value_rank;
    }

    pub fn historizing(&self) -> bool {
        self.historizing
    }

    pub fn set_historizing(&mut self, historizing: bool) {
        self.historizing = historizing;
    }

    pub fn array_dimensions(&self) -> Option<Vec<u32>> {
        self.array_dimensions.clone()
    }

    pub fn set_array_dimensions(&mut self, array_dimensions: &[u32]) {
        self.array_dimensions = Some(array_dimensions.to_vec());
    }

    pub fn data_type(&self) -> NodeId {
        self.data_type.clone()
    }

    pub fn set_data_type<T>(&mut self, data_type: T) where T: Into<NodeId> {
        self.data_type = data_type.into();
    }
}