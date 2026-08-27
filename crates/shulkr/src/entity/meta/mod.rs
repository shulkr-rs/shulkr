use parking_lot::Mutex;
use std::{any::Any, marker::PhantomData, sync::Arc};
use uuid::Uuid;

pub mod refs;

use crate::auth::GameProfile;
use crate::auth::PartialProfile;
use crate::auth::PlayerModel;
use crate::auth::ProfileKind;
use crate::auth::Property;
use crate::auth::ResolvableProfile;
use crate::entity::Hand;
use crate::item::DataType2;
use crate::item::VarInt;

use crate::protocol::decode::Decode as _;
use crate::protocol::decode::DecodeError;
use crate::protocol::decode::PacketRead;
use crate::protocol::encode::Encode as _;
use crate::protocol::encode::EncodeError;
use crate::protocol::encode::PacketWrite;
use crate::text::TextComponent;
use crate::util::BlockPosition;
use crate::util::Either;
use crate::util::EntityPose;
use crate::util::HashMap;

mod avatar;
mod entity;
mod interaction;
mod living_entity;
mod mannequin;
mod mob;
mod painting_variant;
mod player;
mod raider;
mod spellcaster_illager;
mod neutral {
    mod fox;
    mod llama;
    mod piglin;
    mod wolf;
    mod zombie_nautilus;

    pub use fox::*;
    pub use llama::*;
    pub use piglin::*;
    pub use wolf::*;
    pub use zombie_nautilus::*;
}
mod passive {
    mod axolotl;
    mod cat;
    mod chicken;
    mod copper_golem;
    mod cow;
    mod frog;
    mod horse;
    mod mooshroom;
    mod parrot;
    mod pig;
    mod rabbit;
    mod strider;
    mod tropical_fish;
    mod villager;

    pub use axolotl::*;
    pub use cat::*;
    pub use chicken::*;
    pub use copper_golem::*;
    pub use cow::*;
    pub use frog::*;
    pub use horse::*;
    pub use mooshroom::*;
    pub use parrot::*;
    pub use pig::*;
    pub use rabbit::*;
    pub use strider::*;
    pub use tropical_fish::*;
    pub use villager::*;
}
mod hostile {
    mod blaze;
    mod ghast;
    mod hoglin;
    mod pillager;
    mod vex;
    mod witch;
    mod wither;
    mod zoglin;

    pub use blaze::*;
    pub use ghast::*;
    pub use hoglin::*;
    pub use pillager::*;
    pub use vex::*;
    pub use witch::*;
    pub use wither::*;
    pub use zoglin::*;
}

pub use avatar::*;
pub use entity::*;
pub use hostile::*;
pub use interaction::*;
pub use living_entity::*;
pub use mannequin::*;
pub use mob::*;
pub use neutral::*;
pub use painting_variant::*;
pub use passive::*;
pub use player::*;
pub use raider::*;
pub use spellcaster_illager::*;

pub trait MetaAccessor {
    fn new(holder: MetadataHolder) -> Self;
}

#[derive(Clone)]
pub struct MetadataHolder {
    pub entries: Arc<Mutex<HashMap<i32, AnyValue>>>,
}

impl Default for MetadataHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataHolder {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::default())),
        }
    }

    pub fn get<T: Clone + 'static>(&self, r#ref: MetadataRef<T>) -> T {
        let entries = self.entries.lock();
        match &r#ref.kind {
            MetadataRefKind::Direct { id } => {
                let Some(entry) = entries.get(id) else {
                    return r#ref.default_value();
                };
                entry
                    .downcast_ref::<T>()
                    .unwrap_or(&r#ref.default_value())
                    .clone()
            }
            MetadataRefKind::Bitmask { parent_id, mask } => {
                let Some(entry) = entries.get(parent_id) else {
                    return r#ref.default_value();
                };

                let flags = entry.downcast_ref::<u8>().unwrap_or(&0);
                let result = (flags & mask) != 0;
                // SAFETY: It is safe to transmute from bool to T (only when T is of type bool)
                unsafe { std::mem::transmute_copy(&result) }
            }
        }
    }

    pub fn set<T: Send + Sync + 'static>(&self, r#ref: MetadataRef<T>, value: T) {
        let mut entries = self.entries.lock();
        match &r#ref.kind {
            MetadataRefKind::Direct { id } => {
                let Some(entry) = entries.get_mut(id) else {
                    let type_id = r#ref.type_id().id;
                    let value = AnyValue::new(type_id, Arc::new(value));
                    entries.insert(*id, value);
                    return;
                };
                entry.set(Arc::new(value));
            }
            MetadataRefKind::Bitmask { parent_id, mask } => {
                let enabled = unsafe { *(&value as *const T as *const bool) };

                let entry = entries.entry(*parent_id).or_insert_with(|| {
                    AnyValue::new(0, Arc::new(0u8)) // BYTE type id = 0
                });

                if let Some(flags) = entry.downcast_ref::<u8>() {
                    let mut new_flags = *flags;
                    if enabled {
                        new_flags |= mask;
                    } else {
                        new_flags &= !mask;
                    }
                    entry.set(Arc::new(new_flags));
                }
            }
        }
    }
}

pub enum MetadataRefKind {
    Direct { id: i32 },
    Bitmask { parent_id: i32, mask: u8 },
}

pub struct MetadataRef<T> {
    kind: MetadataRefKind,
    type_id: ValueType<T>,
    default_value: T,
}

impl<T> MetadataRef<T> {
    pub const fn new(id: i32, type_id: ValueType<T>, default_value: T) -> Self {
        Self {
            kind: MetadataRefKind::Direct { id },
            type_id,
            default_value,
        }
    }

    pub fn id(&self) -> i32 {
        match &self.kind {
            MetadataRefKind::Direct { id } => *id,
            MetadataRefKind::Bitmask { parent_id, .. } => *parent_id,
        }
    }

    pub fn type_id(&self) -> &ValueType<T> {
        &self.type_id
    }

    pub fn default_value(self) -> T {
        self.default_value
    }
}

impl MetadataRef<bool> {
    pub const fn bitmask(parent_id: i32, mask: u8, default_value: bool) -> Self {
        Self {
            kind: MetadataRefKind::Bitmask { parent_id, mask },
            type_id: ValueType::BOOL,
            default_value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnyValue {
    type_id: i32,
    value: Arc<dyn Any + Send + Sync>,
}

impl AnyValue {
    fn new(id: i32, value: Arc<dyn Any + Send + Sync>) -> Self {
        Self { type_id: id, value }
    }

    pub fn type_id(&self) -> i32 {
        self.type_id
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn set(&mut self, value: Arc<dyn Any + Send + Sync>) {
        self.value = value;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ValueType<T> {
    id: i32,
    __phantom: PhantomData<T>,
}

impl<T> ValueType<T> {
    pub const fn new(id: i32) -> Self {
        Self {
            id,
            __phantom: PhantomData,
        }
    }
}

macro_rules! define_types {
    ($(const $name:ident: ValueType<$ty:ty$(, $ser:ty)?> = ValueType::new($id:expr);)*) => {
        impl ValueType<()> {
            $(pub const $name: ValueType<$ty> = ValueType::new($id);)*
        }

        // static BY_ID: phf::Map<i32, AnyDataComponent> = phf::phf_map! {
        //     $($id => DataComponent::$name.inner,)*
        // };

        // static BY_KEY: phf::Map<&str, AnyDataComponent> = phf::phf_map! {
        //     $($key => DataComponent::$name.inner,)*
        // };

        impl AnyValue {
            pub fn decode_value<R: PacketRead>(r: &mut R) -> Result<AnyValue, DecodeError> {
                let type_id = r.read_varint()?;
                let value: Arc<dyn Any + Send + Sync> = match type_id {
                    $($id => decode!(r, $ty $(, $ser)?),)*
                    _ => panic!("error")
                };
                Ok(AnyValue {
                    type_id,
                    value
                })
            }

            pub fn encode_value<W: PacketWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
                w.write_varint(self.type_id())?;
                match self.type_id() {
                    $($id => {
                        let value = match self.value.downcast_ref::<$ty>() {
                            Some(v) => v,
                            None => panic!("error"),
                        };

                        encode!(w, value, $ty $(, $ser)?)
                    },)*
                    _ => panic!("error")
                }
                Ok(())
            }
        }
    };
}

macro_rules! decode {
    ($reader:expr, $typ:ty) => {
        Arc::new(<$typ as DataType2<$typ>>::decode($reader)?)
    };
    ($reader:expr, $typ:ty, $ser:ty) => {
        Arc::new(<$ser as DataType2<$typ>>::decode($reader)?)
    };
}

macro_rules! encode {
    ($writer:expr, $value:expr, $typ:ty) => {
        <$typ as DataType2<$typ>>::encode($writer, $value)?
    };
    ($writer:expr, $value:expr, $typ:ty, $ser:ty) => {
        <$ser as DataType2<$typ>>::encode($writer, $value)?
    };
}

define_types! {
    const BOOL: ValueType<bool> = ValueType::new(-1);
    const FLOAT: ValueType<f32> = ValueType::new(3);

    const BYTE: ValueType<u8> = ValueType::new(0);
    const VAR_INT: ValueType<i32, VarInt> = ValueType::new(1);

    const OPTIONAL_TEXT_COMPONENT: ValueType<Option<TextComponent>> = ValueType::new(6);
    const OPTIONAL_POSITION: ValueType<Option<BlockPosition>> = ValueType::new(7);
    const OPTIONAL_LIVING_ENTITY: ValueType<Option<Uuid>> = ValueType::new(13);
    const VILLAGER_DATA: ValueType<VillagerData> = ValueType::new(18);
    const OPTIONAL_VAR_INT: ValueType<Option<i32>, Option<VarInt>> = ValueType::new(19);
    const POSE: ValueType<EntityPose> = ValueType::new(20);

    const COPPER_GOLEM_STATE: ValueType<CopperGolemState> = ValueType::new(32);
    const WEATHERING_COPPER_STATE: ValueType<WeatheringCopperState> = ValueType::new(33);

    const RESOLVABLE_PROFILE: ValueType<ResolvableProfile> = ValueType::new(41);
    const HUMANOID_ARM: ValueType<Hand> = ValueType::new(42);
}

impl DataType2<Self> for ResolvableProfile {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Self, DecodeError> {
        let kind = ProfileKind::try_from(r.read_varint()?)
            .map_err(|_| DecodeError::Decode("Invalid ProfileKind"))?;

        Ok(Self {
            kind,
            unpack: match kind {
                ProfileKind::Partial => Either::Left(PartialProfile {
                    username: r.read_option(R::read_string)?,
                    uuid: r.read_option(R::read_uuid)?,
                    properties: r.read_array(Property::decode)?,
                }),
                ProfileKind::Complete => Either::Right(GameProfile {
                    uuid: r.read_uuid()?,
                    name: r.read_string()?,
                    properties: r.read_array(Property::decode)?,
                }),
            },
            body: r.read_option(R::read_identifier)?,
            cape: r.read_option(R::read_identifier)?,
            elytra: r.read_option(R::read_identifier)?,
            model: r
                .read_option(R::read_varint)?
                .map(PlayerModel::try_from)
                .transpose()
                .map_err(|_| DecodeError::Decode("Invalid PlayerModel"))?,
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Self) -> Result<(), EncodeError> {
        w.write_varint(this.kind() as i32)?;
        match this.profile() {
            Either::Left(p) => {
                w.write_option(&p.username, |w, v| w.write_string(v))?;
                w.write_option(&p.uuid, W::write_uuid)?;
                w.write_array(&p.properties, Property::encode)?;
            }
            Either::Right(p) => {
                w.write_uuid(&p.uuid)?;
                w.write_string(&p.name)?;
                w.write_array(&p.properties, Property::encode)?;
            }
        }
        w.write_option(&this.body(), |w, v| w.write_identifier(v))?;
        w.write_option(&this.cape(), |w, v| w.write_identifier(v))?;
        w.write_option(&this.elytra(), |w, v| w.write_identifier(v))?;
        w.write_option(&this.player_model(), |w, v| w.write_varint(**v as i32))?;
        Ok(())
    }
}

impl DataType2<Hand> for Hand {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Hand, DecodeError> {
        Hand::try_from(r.read_varint()?).map_err(|_| DecodeError::Decode("Invalid Hand"))
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Hand) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}

impl DataType2<u8> for u8 {
    fn decode<R: PacketRead>(r: &mut R) -> Result<u8, DecodeError> {
        r.read_u8()
    }
    fn encode<W: PacketWrite>(w: &mut W, this: &u8) -> Result<(), EncodeError> {
        w.write_u8(*this)
    }
}

impl DataType2<BlockPosition> for BlockPosition {
    fn decode<R: PacketRead>(r: &mut R) -> Result<BlockPosition, DecodeError> {
        r.read_position()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &BlockPosition) -> Result<(), EncodeError> {
        w.write_position(this)
    }
}

impl DataType2<Uuid> for Uuid {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Uuid, DecodeError> {
        r.read_uuid()
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Uuid) -> Result<(), EncodeError> {
        w.write_uuid(this)
    }
}

impl<T> DataType2<Option<T>> for Option<T>
where
    T: DataType2<T>,
{
    fn decode<R: PacketRead>(r: &mut R) -> Result<Option<T>, DecodeError> {
        if r.read_bool()? {
            Ok(Some(T::decode(r)?))
        } else {
            Ok(None)
        }
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Option<T>) -> Result<(), EncodeError> {
        w.write_bool(this.is_some())?;
        if let Some(this) = this {
            T::encode(w, this)?;
        }
        Ok(())
    }
}

impl DataType2<Option<i32>> for Option<VarInt> {
    fn decode<R: PacketRead>(r: &mut R) -> Result<Option<i32>, DecodeError> {
        Ok(match r.read_varint()? {
            0 => None,
            value => Some(value - 1),
        })
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &Option<i32>) -> Result<(), EncodeError> {
        w.write_varint(this.map_or(0, |value| value + 1))
    }
}

impl DataType2<EntityPose> for EntityPose {
    fn decode<R: PacketRead>(r: &mut R) -> Result<EntityPose, DecodeError> {
        EntityPose::try_from(r.read_varint()?).map_err(|_| todo!())
    }

    fn encode<W: PacketWrite>(w: &mut W, this: &EntityPose) -> Result<(), EncodeError> {
        w.write_varint(*this as i32)
    }
}
