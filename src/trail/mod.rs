use smash::app::sv_animcmd::*;
use smash::phx::{Hash40, Vector2f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::hash40;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::lib::{L2CValue, L2CAgent};
use std::mem;
use smash::app::*;
use crate::util::*;
static mut HANDS :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut HAS_WALLJUMP: [bool; 8] = [false; 8];
static mut TO_FAIR: [bool; 8] = [false; 8];

#[acmd_script(
    agent = "trail",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_jab1(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 320, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 320, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 11.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 180, /*KBG*/ 22, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ -1.2, /*Y*/ 9.2, /*Z*/ -6.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 8.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.8, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ -6.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 8.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 3,false);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		};
		frame(fighter.lua_state_agent, 22.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
		};
}
#[acmd_script(
    agent = "trail",
    script =  "game_attack12",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_jab2(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 60, /*KBG*/ 12, /*FKB*/ 0, /*BKB*/ 42, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 80, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 100, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 26, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 361, /*KBG*/ 16, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 5, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.6, /*Angle*/ 180, /*KBG*/ 22, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.3, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 4, /*Frames*/ 11.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 5, /*Frames*/ 11.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		};
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
		};
}
#[acmd_script(
    agent = "trail",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_dair(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
			KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
		};
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
			KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
		};
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.8, /*Angle*/ 58, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 0.0, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.8, /*Angle*/ 58, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 3.2, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.8, /*Angle*/ 58, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 6.4, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 14.8, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 0.0, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_LL, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 14.8, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 3.2, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_LL, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 14.8, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 25, /*Size*/ 3.8, /*X*/ 0.4, /*Y*/ 6.4, /*Z*/ 0.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_LL, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 4.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.75);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}
#[acmd_script(
    agent = "trail",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sora_dair_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		frame(lua_state, 11.0);
		if macros::is_excute(fighter) {
			macros::PLAY_STATUS(fighter, Hash40::new("se_trail_attackair_b01"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_attack03"));
		}
}	
#[acmd_script(
    agent = "trail",
    scripts =  ["sound_landingairlw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn sora_dair_land_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		frame(lua_state, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_LANDING_SE(fighter, Hash40::new("se_trail_landing02"));
		};
}
#[acmd_script(
    agent = "trail",
    scripts =  ["effect_landingairlw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sora_dair_land_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "trail",
    scripts =  ["game_landingairlw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_dair_land(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "trail",
    scripts =  ["expression_landingairlw"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn sora_dair_land_expr(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
		}
}
#[acmd_script(
    agent = "trail",
    script =  "game_attackairn",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_nair_1(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 66, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 78, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 82, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 44, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 4.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 5.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 74, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 88, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 92, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 1.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 4.0, /*Unk*/ false);
		};
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 58, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 76, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 76, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 76, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 92, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 58, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 72, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 78, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 76, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 76, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 50, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 80, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 52, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 82, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 54, /*KBG*/ 4, /*FKB*/ 0, /*BKB*/ 84, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 128, /*KBG*/ 32, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 128, /*KBG*/ 32, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 3.8, /*Angle*/ 128, /*KBG*/ 32, /*FKB*/ 0, /*BKB*/ 72, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 8.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		};
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON);
		};
		wait(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		};
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO);
		};
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		};
}
#[acmd_script(
    agent = "trail",
    script =  "game_attackairn3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_nair_3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 6.8, /*Angle*/ 68, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ -1.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 6.8, /*Angle*/ 68, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 6.2, /*Z*/ -1.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 6.8, /*Angle*/ 68, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 4.6, /*X*/ 0.0, /*Y*/ 10.6, /*Z*/ -1.8, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
			
	}
#[acmd_script(
    agent = "trail",
    script =  "effect_attackairn3",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sora_nair_3_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x13dcc5dae1), Hash40::new_raw(0x1345cc8b5b), 20, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::AFTER_IMAGE_OFF(fighter, 3);
		}
	}
#[acmd_script(
    agent = "trail",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_fair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 8.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 45, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 4.2, /*X*/ 0.0, /*Y*/ -1.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 45, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 45, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.4, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_SLASH, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
	}
#[acmd_script(
    agent = "trail",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sora_dair_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
			macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x13dcc5dae1), Hash40::new_raw(0x1345cc8b5b), 20, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_flare"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_light"), false, true);
			macros::AFTER_IMAGE_OFF(fighter, 3);
		}
}
#[acmd_script(
    agent = "trail",
    script =  "game_specialinput",
    category = ACMD_GAME,
	low_priority)]
unsafe fn sora_special_input(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	frame(fighter.lua_state_agent, 19.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
	}
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 35, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
	}
	frame(fighter.lua_state_agent, 26.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 25, /*BKB*/ 0, /*Size*/ 4.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 4, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 10.0, /*Unk*/ false);
	}
	frame(fighter.lua_state_agent, 32.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 110, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 110, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 9.0, /*Angle*/ 110, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 6.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_CLEAVE, /*Type*/ *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 34.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
	frame(fighter.lua_state_agent, 36.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_STAB, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 4.6, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_STAB, /*Type*/ *ATTACK_REGION_SWORD);
		macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 75, /*FKB*/ 0, /*BKB*/ 75, /*Size*/ 3.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_TRAIL_STAB, /*Type*/ *ATTACK_REGION_SWORD);
	}
	frame(fighter.lua_state_agent, 44.0);
	macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}
#[acmd_script(
    agent = "trail",
    script =  "effect_specialinput",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn sora_special_input_eff(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("trail_keyblade_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
			macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(0x13dcc5dae1), Hash40::new_raw(0x1345cc8b5b), 20, Hash40::new("haver"), 0.0, 2.0, 0.0, Hash40::new("haver"), 0.0, 13.8, 0.0, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
		}
		frame(fighter.lua_state_agent, 39.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_flare"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("trail_keyblade_light"), false, true);
			macros::AFTER_IMAGE_OFF(fighter, 5);
		}
	}
	#[acmd_script(
		agent = "trail",
		script =  "sound_specialinput",
		category = ACMD_SOUND,
		low_priority)]
	unsafe fn sora_special_input_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 17.0);
			if macros::is_excute(fighter) {
				macros::PLAY_STATUS(fighter, Hash40::new("se_trail_attackair_b01"));
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_attack03"));
			}
		}
	#[acmd_script(
		agent = "trail",
		scripts =  ["game_speciallw", "game_specialairlw"],
		category = ACMD_GAME,
		low_priority)]
	unsafe fn sora_downb(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 3.0, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 3.0, /*Angle*/ 35, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 6.2, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.75, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
			}
			frame(fighter.lua_state_agent, 22.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
		}
	#[acmd_script(
		agent = "trail",
		scripts =  ["effect_speciallw", "effect_specialairlw"],
		category = ACMD_EFFECT,
		low_priority)]
	unsafe fn sora_downb_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			
	}
	#[acmd_script(
		agent = "trail",
		scripts =  ["sound_speciallw", "sound_specialairlw"],
		category = ACMD_SOUND,
		low_priority)]
	unsafe fn sora_downb_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_smash_s"));
			}
			frame(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h01"));
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h02"));
			}
			frame(fighter.lua_state_agent, 6.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h03"));
			}
			frame(fighter.lua_state_agent, 9.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h02"));
			}
			frame(fighter.lua_state_agent, 12.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h01"));
			}
			frame(fighter.lua_state_agent, 15.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h02"));
			}
			frame(fighter.lua_state_agent, 18.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h03"));
			}
			frame(fighter.lua_state_agent, 21.0);
			if macros::is_excute(fighter) {
				macros::PLAY_SE(fighter, Hash40::new("se_trail_special_h02"));
			}
	}
	#[acmd_script(
		agent = "trail",
		scripts =  ["sound_speciallwstart", "sound_specialairlwstart"],
		category = ACMD_SOUND,
		low_priority)]
	unsafe fn sora_downb_start_snd(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
	}
	
	#[acmd_script(
		agent = "trail",
		scripts =  ["game_speciallwstart", "game_specialairlwstart"],
		category = ACMD_GAME,
		low_priority)]
	unsafe fn sora_downb_start(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		}
	#[acmd_script(
		agent = "trail",
		scripts =  ["effect_speciallwstart", "effect_specialairlwstart"],
		category = ACMD_EFFECT,
		low_priority)]
	unsafe fn sora_downb_start_eff(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			
	}
	#[acmd_script(
		agent = "trail_thunder",
		script =  "game_fall",
		category = ACMD_GAME,
		low_priority)]
	unsafe fn thundaga_1(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 10, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 52, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 4.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 50, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 94, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
	}
	#[acmd_script(
		agent = "trail_thunder",
		script =  "game_fallair",
		category = ACMD_GAME,
		low_priority)]
	unsafe fn thundaga_2(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 344, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 48, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 4.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 68, /*KBG*/ 26, /*FKB*/ 0, /*BKB*/ 98, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
	}
	#[acmd_script(
		agent = "trail_thunder",
		script =  "game_falllast",
		category = ACMD_GAME,
		low_priority)]
	unsafe fn thundaga_3(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 4.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 10.0);
	}
	#[acmd_script(
		agent = "trail_thunder",
		script =  "0x10983531cc",
		category = ACMD_GAME,
		low_priority)]
	unsafe fn thundaga_4(fighter: &mut L2CAgentBase) {
		let lua_state = fighter.lua_state_agent;
			frame(fighter.lua_state_agent, 1.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 4.0);
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.2, /*Angle*/ 64, /*KBG*/ 140, /*FKB*/ 0, /*BKB*/ 62, /*Size*/ 3.6, /*X*/ 0.0, /*Y*/ 0.4, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.6, /*SDI*/ 0.8, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_MAGIC);
			}
			wait(fighter.lua_state_agent, 10.0);
	}
// Use this for general per-frame fighter-level hooks
#[fighter_frame_callback]
pub fn sora(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let end_frame = MotionModule::end_frame(boma);
		if fighter_kind == *FIGHTER_KIND_TRAIL {
			if motion_kind == hash40("attack_air_f") {
				WorkModule::off_flag(boma, *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLOAT_LANDING_ATTACK_AIR_FRAME_F);
			};
			if [*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_WIN].contains(&status_kind) || smash::app::sv_information::is_ready_go() == false || StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
				HAS_WALLJUMP[ENTRY_ID] = false;
			};
			/*if motion_kind == hash40("attack_air_lw") {
				if frame > 14.0 && frame < 20.0 {
					MotionModule::change_motion(boma, Hash40::new("attack_air_lw"), 41.0, 1.0, false, 0.0, false, false);
				};
			};*/
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				if frame > 9.0 {
					StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK, true);
				};
				if frame as i32 % 3 == 0 {
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a2, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a3, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a4, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a5, 2.2, 0.4, 1.0);
				};
				if motion_duration(boma) == 1 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
					};
				};
				if motion_duration(boma) == 5 {
					if (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.2 {
						PostureModule::reverse_lr(boma);
						PostureModule::update_rot_y_lr(boma);
						let stop_rise  = smash::phx::Vector3f { x: -1.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
				};
			};
			if status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F {
				let dummy = 0;
				//if MotionModule::frame(boma) > 2.0 && MotionModule::frame(boma) < 4.0 {
					//TO_FAIR[ENTRY_ID] = true;
					//StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
				//};
			} else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
				if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("attack_air_f"){
					MotionModule::change_motion(boma, Hash40::new("attack_air_f"), 2.0, 1.0, false, 0.0, false, false);
				};
				if !WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) && MotionModule::frame(boma) > 5.0 {
					TO_FAIR[ENTRY_ID] = false;
				};
			} else if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
				if TO_FAIR[ENTRY_ID] == true && MotionModule::motion_kind(boma) != hash40("landing_air_f"){
					let landing = 1.0/(((WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0)))/ FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)), false)as f32);
					MotionModule::change_motion(boma, Hash40::new("landing_air_f"), 2.0, landing, false, 0.0, false, false);
				};
			} else {
				TO_FAIR[ENTRY_ID] = false;
			};
			if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_LW_ATTACK {
				HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
				if end_frame-frame < 4.0 || (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && frame > 12.0) {
					if GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -2.0}, false) == 1{
						StatusModule::set_keep_situation_air(boma, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END, true);
					};
				} else {
					StatusModule::set_keep_situation_air(boma, true);
					if StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR {
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
					};
				};
				if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
				};
				if frame < 9.0 {
					macros::SET_SPEED_EX(fighter, frame * 0.125, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else if frame > 20.0 {
					macros::SET_SPEED_EX(fighter, (26.0 - frame) * 0.125, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				} else {
					macros::SET_SPEED_EX(fighter, 1.25, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				MotionModule::set_rate(boma, 0.66667);
				if frame as i32 % 5 == 0 {
					let a1: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("haver"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("havel"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a3: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footr"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a4: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("footl"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					let a5: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_magicball_aura"), smash::phx::Hash40::new("hip"), &HANDS, &HANDS, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, a1, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a2, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a3, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a4, 2.2, 0.4, 1.0);
					EffectModule::set_rgb(boma, a5, 2.2, 0.4, 1.0);
				};
				if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma))< -0.7 && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)  && !HAS_WALLJUMP[ENTRY_ID]{
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
					HAS_WALLJUMP[ENTRY_ID] = true;
				};
				if !is_hitlag(boma) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
					if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
					} else {
						KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP);
						StatusModule::change_status_request_from_script(boma, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, true);
					};
				};
			};
		};
    }
}
#[status_script(agent = "trail", status = FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn fair_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    fighter.sub_attack_air_kind();
    if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION){
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        } else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    let _ = fighter.sub_attack_air_uniq_process_init();
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	TO_FAIR[ENTRY_ID] = true;
    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
    0.into()
}
//fighter.status_AttackAir()
#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    fighter.sub_attack_air_kind();
    if motion_kind != hash40("jump_aerial_f") {
        if motion_kind == hash40("jump_aerial_b"){
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                        MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                    }
                }
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
					if SPEED_Y[ENTRY_ID] > 2.5 {
						let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
						macros::SET_SPEED_EX(fighter, new_speed, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					};
                }
                fighter.sub_attack_air_uniq_process_init();
                return L2CValue::I32(0);
        }
    }
    else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                    MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				if SPEED_Y[ENTRY_ID] > 3.0 {
					let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
					macros::SET_SPEED_EX(fighter, new_speed, 3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
            }
            fighter.sub_attack_air_uniq_process_init();
            return L2CValue::I32(0);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}
#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
	let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
	if true{ //(stick_x <= -0.4 || stick_x >= 0.4) || (stick_y <= -0.4 || stick_y >= 0.4) {//motion_kind != hash40("attack_air_n") {
    	fighter.status_pre_AttackAir();
		0.into()
	} else {
		fighter.change_status(
			L2CValue::I32(*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N),
			L2CValue::Bool(false)
		);
		0.into()
		//original!(fighter)
	}
}
#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.status_AttackAir_Main();
	0.into()
}
#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
	let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if [hash40("attack_air_n"), 0x0d7484f6cfu64, 0x0d0383c659u64].contains(&motion_kind) {
    	if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) && frame > 4.0{
			let new_speed = SPEED_X[ENTRY_ID]*PostureModule::lr(fighter.module_accessor);
			macros::SET_SPEED_EX(fighter, new_speed, 1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		};
		if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON) && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_ENABLE_COMBO) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
			WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_CHECK_COMBO_BUTTON_ON);
			if motion_kind == 0x0d7484f6cfu64 {
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
				MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x0d0383c659), -1.0, 1.0, false, 0.0, false, false);
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
			};
			if motion_kind == hash40("attack_air_n") {
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
				MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x0d7484f6cf), -1.0, 1.0, false, 0.0, false, false);
				MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
			};
		};
	};
	original!(fighter)
}


pub fn install() {
    smashline::install_acmd_scripts!(
		sora_jab1,
		sora_jab2,
		sora_dair,
		sora_nair_1,
		sora_dair_snd,
		sora_downb,
		sora_downb_eff,
		sora_downb_snd,
		sora_downb_start,
		sora_downb_start_snd,
		sora_downb_start_eff,
		sora_dair_eff,
		sora_dair_land,
		sora_dair_land_eff,
		sora_dair_land_snd,
		sora_dair_land_expr,
		sora_fair,
		sora_special_input,
		sora_special_input_eff,
		sora_special_input_snd,
		thundaga_1,
		thundaga_2,
		thundaga_3,
		thundaga_4
    );
    smashline::install_agent_frame_callbacks!(sora);
	install_status_scripts!(fair_init, init_attack_air, pre_attack_air, exec_attack_air);
}
