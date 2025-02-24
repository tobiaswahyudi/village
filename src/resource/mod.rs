use bevy::prelude::*;

#[derive(Clone, Copy, Component, PartialEq, Debug)]
pub struct PickupItem {
    pub item_type: PickupItemType,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PickupItemType {
    Wood
}

