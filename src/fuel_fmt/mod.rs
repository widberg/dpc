use std::collections::HashMap;

use crate::fuel_fmt::animation::AnimationObjectFormat;
use crate::fuel_fmt::binary::BinaryObjectFormat;
use crate::fuel_fmt::bitmap::BitmapObjectFormat;
use crate::fuel_fmt::camera::CameraObjectFormat;
use crate::fuel_fmt::collisionvol::CollisionVolObjectType;
use crate::fuel_fmt::common::FUELObjectFormatTrait;
use crate::fuel_fmt::fonts::FontsObjectFormat;
use crate::fuel_fmt::gameobj::GameObjObjectFormat;
use crate::fuel_fmt::genworld::GenWorldObjectFormat;
use crate::fuel_fmt::gwroad::GwRoadObjectFormat;
use crate::fuel_fmt::lightdata::LightDataObjectFormat;
use crate::fuel_fmt::lod::{LodObjectFormat, LodObjectFormatAlt, LodObjectFormatAltAlt};
use crate::fuel_fmt::loddata::LodDataObjectFormat;
use crate::fuel_fmt::material::{
    MaterialObjectFormat, MaterialObjectFormatAlt, MaterialObjectFormatAltAlt,
};
use crate::fuel_fmt::materialanim::MaterialAnimObjectFormat;
use crate::fuel_fmt::materialobj::MaterialObjObjectFormat;
use crate::fuel_fmt::mesh::{
    MeshObjectFormat, MeshObjectFormatAlt, MeshObjectFormatAltAlt, MeshObjectFormatAltAltAlt,
};
use crate::fuel_fmt::meshdata::MeshDataObjectFormat;
use crate::fuel_fmt::node::{NodeObjectFormat, NodeObjectFormatAlt};
use crate::fuel_fmt::omni::OmniObjectFormat;
use crate::fuel_fmt::particles::{ParticlesObjectFormat, ParticlesObjectFormatAlt};
use crate::fuel_fmt::particlesdata::ParticlesDataObjectFormat;
use crate::fuel_fmt::rotshape::RotShapeObjectFormat;
use crate::fuel_fmt::rotshapedata::RotShapeDataObjectFormat;
use crate::fuel_fmt::rtc::RtcObjectFormat;
use crate::fuel_fmt::skel::SkelObjectFormat;
use crate::fuel_fmt::skin::{SkinObjectFormat, SkinObjectFormatAlt};
use crate::fuel_fmt::sound::SoundObjectFormat;
use crate::fuel_fmt::spline::SplineObjectFormat;
use crate::fuel_fmt::splinegraph::SplineGraphObjectFormat;
use crate::fuel_fmt::surface::SurfaceObjectFormat;
use crate::fuel_fmt::surfacedatas::SurfaceDatasObjectFormat;
use crate::fuel_fmt::userdefine::UserDefineObjectFormat;
use crate::fuel_fmt::warp::WarpObjectFormat;
use crate::fuel_fmt::world::WorldObjectFormat;
use crate::fuel_fmt::worldref::WorldRefObjectFormat;

mod animation;
mod binary;
mod bitmap;
mod camera;
mod collisionvol;
mod common;
mod fonts;
mod gameobj;
mod genworld;
mod gwroad;
mod lightdata;
mod lod;
mod loddata;
mod material;
mod materialanim;
mod materialobj;
mod mesh;
mod meshdata;
mod node;
mod omni;
mod particles;
mod particlesdata;
mod rotshape;
mod rotshapedata;
mod rtc;
mod skel;
mod skin;
mod sound;
mod spline;
mod splinegraph;
mod surface;
mod surfacedatas;
mod userdefine;
mod warp;
mod world;
mod worldref;

pub fn get_formats<'a>(version: &String) -> HashMap<u32, &'a dyn FUELObjectFormatTrait> {
    let mut formats: HashMap<u32, &'a dyn FUELObjectFormatTrait> = HashMap::new();

    formats.insert(1175485833, AnimationObjectFormat::new());
    formats.insert(4240844041, CameraObjectFormat::new());
    formats.insert(2398393906, CollisionVolObjectType::new());
    formats.insert(1536002910, FontsObjectFormat::new());
    formats.insert(4096629181, GameObjObjectFormat::new());
    formats.insert(838505646, GenWorldObjectFormat::new());
    formats.insert(3845834591, GwRoadObjectFormat::new());
    formats.insert(848525546, LightDataObjectFormat::new());
    formats.insert(3412401859, LodDataObjectFormat::new());
    formats.insert(3834418854, MaterialAnimObjectFormat::new());
    formats.insert(849861735, MaterialObjObjectFormat::new());
    formats.insert(3626109572, MeshDataObjectFormat::new());
    formats.insert(549480509, OmniObjectFormat::new());
    formats.insert(954499543, ParticlesDataObjectFormat::new());
    formats.insert(866453734, RotShapeObjectFormat::new());
    formats.insert(1625945536, RotShapeDataObjectFormat::new());
    formats.insert(705810152, RtcObjectFormat::new());
    formats.insert(3611002348, SkelObjectFormat::new());
    formats.insert(1135194223, SplineObjectFormat::new());
    formats.insert(1910554652, SplineGraphObjectFormat::new());
    formats.insert(1706265229, SurfaceObjectFormat::new());
    formats.insert(3747817665, SurfaceDatasObjectFormat::new());
    formats.insert(1391959958, UserDefineObjectFormat::new());
    formats.insert(1114947943, WarpObjectFormat::new());
    formats.insert(968261323, WorldObjectFormat::new());
    formats.insert(2906362741, WorldRefObjectFormat::new());
    formats.insert(2259852416, BinaryObjectFormat::new());
    formats.insert(849267944, SoundObjectFormat::new());
    formats.insert(1471281566, BitmapObjectFormat::new());

    match version.as_str() {
        "v1.381.67.09 - Asobo Studio - Internal Cross Technology" => {
            formats.insert(2204276779, MaterialObjectFormat::new());
            formats.insert(1387343541, MeshObjectFormat::new());
            formats.insert(1396791303, SkinObjectFormat::new());
            formats.insert(2245010728, NodeObjectFormat::new());
            formats.insert(3312018398, ParticlesObjectFormat::new());
            formats.insert(1943824915, LodObjectFormat::new());
        }
        "v1.381.66.09 - Asobo Studio - Internal Cross Technology" => {
            formats.insert(2204276779, MaterialObjectFormat::new());
            formats.insert(1387343541, MeshObjectFormat::new());
            formats.insert(1396791303, SkinObjectFormat::new());
            formats.insert(2245010728, NodeObjectFormat::new());
            formats.insert(3312018398, ParticlesObjectFormat::new());
            formats.insert(1943824915, LodObjectFormat::new());
        }
        "v1.381.65.09 - Asobo Studio - Internal Cross Technology" => {
            formats.insert(2204276779, MaterialObjectFormat::new());
            formats.insert(1387343541, MeshObjectFormatAlt::new());
            formats.insert(1396791303, SkinObjectFormat::new());
            formats.insert(2245010728, NodeObjectFormat::new());
            formats.insert(3312018398, ParticlesObjectFormat::new());
            formats.insert(1943824915, LodObjectFormat::new());
        }
        "v1.381.64.09 - Asobo Studio - Internal Cross Technology" => {
            formats.insert(2204276779, MaterialObjectFormat::new());
            formats.insert(1387343541, MeshObjectFormatAlt::new());
            formats.insert(1396791303, SkinObjectFormat::new());
            formats.insert(2245010728, NodeObjectFormat::new());
            formats.insert(3312018398, ParticlesObjectFormatAlt::new());
            formats.insert(1943824915, LodObjectFormat::new());
        }
        "v1.379.60.09 - Asobo Studio - Internal Cross Technology" => {
            formats.insert(2204276779, MaterialObjectFormat::new());
            formats.insert(1387343541, MeshObjectFormatAltAlt::new());
            formats.insert(1396791303, SkinObjectFormat::new());
            formats.insert(2245010728, NodeObjectFormat::new());
            formats.insert(3312018398, ParticlesObjectFormatAlt::new());
            formats.insert(1943824915, LodObjectFormatAltAlt::new());
        }
        "v1.325.50.07 - Asobo Studio - Internal Cross Technology" => {
            // TRAFFIC_TM
            formats.insert(2204276779, MaterialObjectFormatAltAlt::new());
            formats.insert(1387343541, MeshObjectFormatAltAlt::new());
            formats.insert(1396791303, SkinObjectFormatAlt::new());
            formats.insert(2245010728, NodeObjectFormatAlt::new());
            formats.insert(3312018398, ParticlesObjectFormatAlt::new());
            formats.insert(1943824915, LodObjectFormatAlt::new());
        }
        "v1.220.50.07 - Asobo Studio - Internal Cross Technology" => {
            // P_MOTO
            formats.insert(2204276779, MaterialObjectFormatAlt::new());
            formats.insert(1387343541, MeshObjectFormatAltAltAlt::new());
            formats.insert(1396791303, SkinObjectFormatAlt::new());
            formats.insert(2245010728, NodeObjectFormatAlt::new());
            formats.insert(3312018398, ParticlesObjectFormatAlt::new());
            formats.insert(1943824915, LodObjectFormatAlt::new());
        }
        _ => panic!("bad version"),
    }

    return formats;
}
