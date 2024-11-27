use super::super::types::AddressType;
use super::super::types::IdType;

pub trait GettersSetters {
    fn get_id(&self) -> IdType;
    fn set_id(&mut self, id: IdType);
    fn increment_id(&mut self);

    fn get_source_device_identifier(&self) -> AddressType;
}
