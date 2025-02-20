use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd::*;
use smash::lua2cpp::*;
use smashline::*;
use smash_script::*;
use smash::app::ArticleOperationTarget;
use smash::lib::{L2CValue, L2CAgent};
use smash::phx::{Hash40, Vector2f};
use crate::util::*;

static mut DOWNB_JUMP : [bool; 8] = [false; 8];
static mut UPB_ANGLE : [i32; 8] = [1; 8];
//0 - Inwards
//1 - Middle
//2 - Outwards
static mut IS_FINAL : [bool; 8] = [false; 8];
static NONE :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 5.0, z: 0.0 };
static mut variance : [f32; 8] = [0.0; 8];
static mut N1 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 3.0, z: -15.0 };
static mut N2 :  smash::phx::Vector3f =  smash::phx::Vector3f { x: 0.0, y: 8.0, z: -24.0 };
static mut F3 : [u32; 8] = [0; 8];
static mut F4 : [u32; 8] = [0; 8];
#[acmd_script(
    agent = "kirby",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.7, /*X*/ 4.3, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("toer"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 30, /*Size*/ 3.7, /*X*/ -2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 53, /*KBG*/ 39, /*FKB*/ 0, /*BKB*/ 43, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 6.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 84, /*KBG*/ 38, /*FKB*/ 0, /*BKB*/ 35, /*Size*/ 4.4, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(6.5), /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 367, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 28, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 2.8, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.1), /*Z2*/ Some(7.0), /*Hitlag*/ 0.8, /*SDI*/ 0.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 361, /*KBG*/ 148, /*FKB*/ 0, /*BKB*/ 24, /*Size*/ 5.1, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 13.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.2), /*Z2*/ Some(7.0), /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 41.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}		
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialhi", "game_specialairhi"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_upb1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			if true{
				let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
				StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, true);
			}
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi4"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb4_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_specialhi4"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn kirby_upb4_exp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_upb4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}	
#[acmd_script(
    agent = "kirby",
    script =  "effect_specialairhi2",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_air_upb2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_attack_arc"), Hash40::new("kirby_attack_arc"), Hash40::new("top"), -20.0, 6, 0, 0, 0, 90, 1.0, true, *EF_FLIP_NONE);
			macros::LAST_EFFECT_SET_COLOR(fighter, 2.016, 0.648, 1.536);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.275);
		}
		frame(fighter.lua_state_agent, 3.0);
		for _ in 0..3 {
			if macros::is_excute(fighter) {
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
				macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire"), Hash40::new("haver"), 0, 3, 0.3, 0, 0, 0, 1, true);
				if true{
					let boma = fighter.module_accessor;
					let distance2 = smash::phx::Vector3f { x: 0.0, y: 6.0, z: 7.0 };
					let empty = smash::phx::Vector3f { x: 0.0, y: 0.0, z: 0.0 };
					let fire: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.65, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire, 1.0, 0.25, 0.25);
					EffectModule::set_alpha(boma, fire, 0.5);
					EffectModule::set_rate(boma, fire, 0.5);
					let fire2: u32 = EffectModule::req_follow(boma, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &distance2, &empty, 0.45, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(boma, fire2, 3.0, 0.5, 0.5);
					EffectModule::set_rate(boma, fire2, 0.5);
					EffectModule::set_rate(boma, fire, 0.75);
				}
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_fireflower_shot"), false, true);
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("kirby_attack_arc"), false, true);
		}
}
		
#[acmd_script(
    agent = "kirby",
    script =  "game_specialairhi2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_air_upb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 2.0);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.7, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.7, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.8, /*Angle*/ 367, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 7.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(7.5), /*Z2*/ Some(7.9), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 9.0, /*Angle*/ 83, /*KBG*/ 82, /*FKB*/ 0, /*BKB*/ 74, /*Size*/ 8.5, /*X*/ 0.0, /*Y*/ -1.5, /*Z*/ 4.9, /*X2*/ Some(0.0), /*Y2*/ Some(4.8), /*Z2*/ Some(4.9), /*Hitlag*/ 2.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			if true{
				notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
			}
		}
}		
#[acmd_script(
    agent = "kirby",
    script =  "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 4.0, /*Angle*/ 75, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
			macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 3.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 60, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
		}
		frame(fighter.lua_state_agent, 9.0);
		for _ in 0..9 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.5, /*Angle*/ 270, /*KBG*/ 100, /*FKB*/ 10, /*BKB*/ 0, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 3.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_rush"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
				AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			wait(fighter.lua_state_agent, 2.0);
		}
		frame(fighter.lua_state_agent, 56.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.0, /*Angle*/ 90, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 10, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 2.4, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 58.0);
		if macros::is_excute(fighter) {
			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 73.0);
		if macros::is_excute(fighter) {
			CancelModule::enable_cancel(fighter.module_accessor);
		}
}			
#[acmd_script(
    agent = "kirby",
    script =  "sound_specialairhi2",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, /*Frames*/ 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack07"));
		}
}			
#[acmd_script(
    agent = "kirby",
    script =  "game_attackairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi2"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -9.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -12.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -16.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ -2.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.25, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_SWORD);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}		

#[acmd_script(
    agent = "kirby",
    script =  "game_attackairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi4"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 15.0);
		for _ in 0..5 {
			if macros::is_excute(fighter) {
				macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 1.2, /*Angle*/ 367, /*KBG*/ 85, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ -3.5, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-3.5), /*Z2*/ Some(5.0), /*Hitlag*/ 0.8, /*SDI*/ 1.2, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
			}
			wait(fighter.lua_state_agent, 2.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			wait(fighter.lua_state_agent, 1.0);
		}	
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 1.9, /*Angle*/ 270, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ -3.5, /*Z*/ -3.0, /*X2*/ Some(0.0), /*Y2*/ Some(-3.5), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_SWORD);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ -2.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_attackairlw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_dair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.85, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -6, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), -5, -7, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 3, -9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("kirby_star"), Hash40::new("top"), 7, -7, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.85, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -6, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
		}
		frame(fighter.lua_state_agent, 21.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -3, 0, 0, 0, 0, 0.85, true, *EF_FLIP_YZ);
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -6, 0, 0, 0, 0, 0.6, true, *EF_FLIP_YZ);
		}
	}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_attackairlw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_dair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, /*Frames*/ 15.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_swing_l"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
	}	
#[acmd_script(
    agent = "kirby",
    script =  "game_landingairlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_landing_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ 4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 60, /*KBG*/ 100, /*FKB*/ 40, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 3.2, /*Z*/ -4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}

#[acmd_script(
    agent = "kirby",
    script =  "game_attackhi4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 125, /*KBG*/ 100, /*FKB*/ 155, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 5.5, /*X2*/ Some(0.0), /*Y2*/ Some(5.0), /*Z2*/ Some(-5.5), /*Hitlag*/ 0.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 2, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ false, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 11.0, /*Angle*/ 89, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 25.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 16.0, /*Angle*/ 89, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 30.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BODY);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 11.0, /*Angle*/ 90, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_sting"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_BODY);
		}
		wait(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 42.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_attackhi4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_usmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 20, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
			macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
			macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, 31, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "sound_attackhi4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_usmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
		}
		frame(fighter.lua_state_agent, /*Frames*/ 9.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_attack06"));
		}
		frame(fighter.lua_state_agent, /*Frames*/ 12.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_smash_h01"));
		}
}

#[acmd_script(
    agent = "kirby",
    script =  "game_attacklw4",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 12.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi5"),false,0.0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 6.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 50, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 3.0, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(3.2), /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ -1.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 24.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 12.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 27.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 36.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 54.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("have"), /*Damage*/ 15.0, /*Angle*/ 361, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 2.5, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_B, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 57.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 76.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_attacklw4",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_dsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		//Going into Wheel
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::EFFECT(fighter, Hash40::new("kirby_stone_s"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		}

		//First roll right
		wait(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 2, 6.5, 0, 0, 180, 0, 1.0, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.7, 1.0);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
		}

		//Roll left
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7.5, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5, 0, 0, 0, 0, 1.0, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.7, 1.0);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
		}
		frame(fighter.lua_state_agent, 56.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
		}

		//Last roll right
		frame(fighter.lua_state_agent, 58.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -7.5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 61.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 6.5, 0, 0, 180, 0, 1.0, true);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.6, 0.7, 1.0);
			macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
		}

		//Exiting wheel
		frame(fighter.lua_state_agent, 74.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.35, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 76.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("kirby_stone_e"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "sound_attacklw4",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_dsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		//Entering wheel
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_kirby_special_l01"));
		}

		//Wheel sounds
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
		}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_008"));
		}
		frame(fighter.lua_state_agent, 58.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_008"));
		}
		frame(fighter.lua_state_agent, 73.0);
		if macros::is_excute(fighter) {
			macros::STOP_SE(fighter, Hash40::new("se_kirby_special_h02"));
		}

		//Exiting wheel
		frame(fighter.lua_state_agent, 76.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_special_lw"));
		}
		frame(fighter.lua_state_agent, 82.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_l03"));
		}
}

#[acmd_script(
    agent = "kirby",
    script =  "game_attacklw4b",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_dsmash2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;		
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_attacklw4b",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_dsmash2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;		
}
#[acmd_script(
    agent = "kirby",
    script =  "sound_attacklw4b",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_dsmash2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}

#[acmd_script(
    agent = "kirby",
    script =  "game_speciallw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ground_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    	}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.0, /*Angle*/ 65, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(2.0), /*Z2*/ Some(5.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
    	}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_speciallw",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ground_downb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_speciallw",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ground_downb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "game_speciallw2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ground_downb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
		}
}
#[acmd_script(
    agent = "kirby",
    script =  "effect_speciallw2",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ground_downb2_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_speciallw2",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ground_downb2_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		
}	

#[status_script(
	agent = "kirby", 
	status = FIGHTER_STATUS_KIND_SPECIAL_LW, 
	condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS,
	low_priority)]
pub unsafe fn exec_downb(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if [hash40("special_lw"), hash40("special_lw2")].contains(&motion_kind) {
        fighter.status_AttackS3Common();
        0.into()
    } else {
        original!(fighter)
    }
}

#[acmd_script(
    agent = "kirby",
    scripts =  ["game_attacks3", "game_attacks3hi", "game_attacks3lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 10.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 18.5, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(5.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		wait(fighter.lua_state_agent, 5.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.538);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_ftilt_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s03"));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_ftilt_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 6.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR(fighter, 2, 0.15, 0.02, 0.7);
			macros::BURN_COLOR_FRAME(fighter, 8, 2, 0.15, 0.02, 0);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 12, 4.5, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 18, 4.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 6, 4.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
			macros::EFFECT(fighter, Hash40::new("sys_fireflower_shot"), Hash40::new("top"), 5, 4.5, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			macros::BURN_COLOR_NORMAL(fighter, );
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_attacks4", "game_attacks4hi", "game_attacks4lw"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, 0);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 5.4, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 11.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HAMMER);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 16.0, /*Angle*/ 361, /*KBG*/ 109, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 5.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_HAMMER);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 47.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_attacks4", "effect_attacks4hi", "effect_attacks4lw"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_fsmash_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_onigoroshi_wind"), Hash40::new("kirby_onigoroshi_wind"), Hash40::new("top"), 1, 6, 3, 13, -20, 0, 1, false, *EF_FLIP_YZ);
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_attacks4", "sound_attacks4hi", "sound_attacks4lw"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_fsmash_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_hammermax"));
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s01"));
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_special_s07"));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_attacks4", "expression_attacks4hi", "expression_attacks4s"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn kirby_fsmash_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
		}
		frame(fighter.lua_state_agent, 10.0);
		if macros::is_excute(fighter) {
			macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "effect_attackairb",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_bair_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, /*Frames*/ 10.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("kirby_onigoroshi_wind"), Hash40::new("kirby_onigoroshi_wind"), Hash40::new("top"), 1, 0, 0, 13, 180, 180, 1.5, false, *EF_FLIP_YZ);
			macros::LAST_EFFECT_SET_COLOR(fighter, 0.64, 1.0, 1.0);
		}
	}	
#[acmd_script(
    agent = "kirby",
    script =  "sound_attackairb",
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_bair_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, /*Frames*/ 7.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_kirby_swing_l"));
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
	}	
#[acmd_script(
    agent = "kirby",
    script =  "game_landingairb",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_landing_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "kirby",
    script =  "game_specialairlw2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_downb_end_air(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		if macros::is_excute(fighter) {
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE,smash::phx::Hash40::new("special_air_lw2"),false,0.0);
			macros::SET_SPEED_EX(fighter, 0.0, 1.2, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		}
}	
#[acmd_script(
    agent = "kirby_finalcuttershot",
    script =  "effect_finalcutterregular",
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_beam_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
}
#[acmd_script(
    agent = "kirby_finalcuttershot",
    script =  "game_finalcutterregular",
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_beam(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 140, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 140, /*KBG*/ 30, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.7, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ -6, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ true, /*Absorbable*/ true, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_magic"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_MAGIC, /*Type*/ *ATTACK_REGION_ENERGY);
		}
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialsstart", "game_specialairsstart"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_sideb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
	
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specials", "game_specialairs", "game_specialss", "game_specialairss"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
			ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::phx::Hash40::new("special_hi3"),false,0.0);
		}
		frame(fighter.lua_state_agent, /*Frames*/ 17.0);
		if macros::is_excute(fighter) {
			macros::SET_SPEED_EX(fighter, -1.0, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTERSHOT, false, 0);
		}
		frame(fighter.lua_state_agent, 46.0);
		if macros::is_excute(fighter) {
			ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		}
}	
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_specials", "effect_specialairs", "effect_specialss", "effect_specialairss"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_sideb_eff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 16.0);
		if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_magic"), Hash40::new("have"), 0.0, 5.2, 13.5, 0, 0, 0, 0.3, true);
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specials", "sound_specialairs", "sound_specialss", "sound_specialairss"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_sideb_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	
		frame(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("vc_kirby_002"));
		}	
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_swing_l"));
		}
		frame(fighter.lua_state_agent, 18.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_frieze_l"));
			macros::PLAY_SE(fighter, Hash40::new("se_common_sleep"));
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["expression_specials", "expression_specialairs", "expression_specialss", "expression_specialairss"],
    category = ACMD_EXPRESSION,
	low_priority)]
unsafe fn kirby_sideb_expr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialhi", "sound_specialairhi"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_upb_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, /*Frames*/ 1.0);
		if macros::is_excute(fighter) {
			macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_kirby_rnd_attack"));
		}
}

#[acmd_script(
    agent = "kirby",
    scripts =  ["game_specialinput"],
    category = ACMD_GAME,
	low_priority)]
unsafe fn kirby_special_input(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 5.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("hip"), /*Damage*/ 12.2, /*Angle*/ 46, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 15.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_BODY);
		}
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 45.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["sound_specialinput"],
    category = ACMD_SOUND,
	low_priority)]
unsafe fn kirby_special_input_snd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		if macros::is_excute(fighter) {
			macros::PLAY_SE(fighter, Hash40::new("se_common_electric_hit_ll"));
		}
}
#[acmd_script(
    agent = "kirby",
    scripts =  ["effect_specialinput"],
    category = ACMD_EFFECT,
	low_priority)]
unsafe fn kirby_special_input_eff(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_screw"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.5, true);
		macros::LAST_EFFECT_SET_COLOR(fighter, 0.0119, 1.0, 0.4);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.45);
	}
	frame(fighter.lua_state_agent, 37.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_screw"), false, true);
	}
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		let is_near_ground = GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: 0.0, y: -1.0}, true);

		//CI shit
		if [hash40("special_input")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 0.0 {
				macros::SET_SPEED_EX(fighter,0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
			};
		};

		//Downb shit
		if DOWNB_JUMP[ENTRY_ID] && status_kind == *FIGHTER_STATUS_KIND_JUMP {
			MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw2"), 0.0, 1.0, false, 0.0, false, false);
			DOWNB_JUMP[ENTRY_ID] = false;
		};
		if DOWNB_JUMP[ENTRY_ID] && status_kind != *FIGHTER_STATUS_KIND_JUMP {
			DOWNB_JUMP[ENTRY_ID] = false;
		};
		if [hash40("special_lw")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 4.0 && MotionModule::frame(boma) <= 24.0 {
				if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT){
					DOWNB_JUMP[ENTRY_ID] = true;
					macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
					macros::PLAY_SE(fighter, Hash40::new("se_kirby_jump01"));
					StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP, true);
				};
			};
		};
		if [hash40("special_lw")].contains(&MotionModule::motion_kind(boma)) {
			if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_MOTION {
				KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
			};
			if MotionModule::frame(boma) >= 40.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
			};
		};
		if [hash40("special_lw2")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 29.0 {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
			};
		};

		//Upb shit
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 && MotionModule::frame(boma) >= 37.0 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2 {
			if  MotionModule::frame(boma) < 2.0 {
				let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
				if stick_x <= -0.3 {
					UPB_ANGLE[ENTRY_ID] = 0;
				} else if stick_x >= 0.45 {
					UPB_ANGLE[ENTRY_ID] = 2;
				} else {
					UPB_ANGLE[ENTRY_ID] = 1;
				};
			};
			if MotionModule::frame(boma) <= 6.0 && !is_hitlag(boma) {
				if UPB_ANGLE[ENTRY_ID] == 1 {
					//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 1.0, 0.12, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if UPB_ANGLE[ENTRY_ID] == 2 {
					//let speed = smash::phx::Vector3f { x: ((6.0/MotionModule::frame(boma))*0.05)/6.0, y: -0.03, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 1.75, 0.08, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
				if UPB_ANGLE[ENTRY_ID] == 0 {
					//let speed = smash::phx::Vector3f { x: *(((6.0/MotionModule::frame(boma))*0.003)/6.0)*0.2)-0.03, y: 0.0, z: 0.0 };
					//KineticModule::add_speed(boma, &speed);
					macros::SET_SPEED_EX(fighter, 0.0, 0.16, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				};
			};
			if MotionModule::frame(boma) <= 10.0 && MotionModule::frame(boma) > 6.0 && !is_hitlag(boma) {
				if UPB_ANGLE[ENTRY_ID] != 0 {
					KineticModule::clear_speed_all(boma);
					if UPB_ANGLE[ENTRY_ID] == 1 {
						let speed = smash::phx::Vector3f { x: -0.1, y: 0.0, z: 0.0 };
						KineticModule::add_speed(boma, &speed);
					};
					/*let speed = smash::phx::Vector3f { x: -2.0*(0.02-((MotionModule::frame(boma)-6.0/6.0)*0.02)), y: 0.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);*/
				};
			};
		} else {
			UPB_ANGLE[ENTRY_ID] = 1;
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
		};
		if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4 {
			StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
		};
		if status_kind == *FIGHTER_STATUS_KIND_FINAL || status_kind == *FIGHTER_STATUS_KIND_FINAL_JUMP_END {
			IS_FINAL[ENTRY_ID] = true;
		} else {
			IS_FINAL[ENTRY_ID] = false;
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_START].contains(&status_kind) == false {
			ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD].contains(&status_kind) == false {
			ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER,smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
		};
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
			StatusModule::change_status_request_from_script(boma,*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, true);
		};
		if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
			macros::STOP_SE(fighter, Hash40::new("se_kirby_special_h02"));
		};
	}
}
#[weapon_frame( agent = WEAPON_KIND_KIRBY_FINALCUTTERSHOT )]
pub fn ball_frame(weapon : &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(weapon.module_accessor) as i32;
		if frame < 15 {
			if frame >= 2 {
				if frame % 4 == 0 {
					variance[ENTRY_ID] = 8.0;
				} else if (frame+1) % 2 == 0 {
					variance[ENTRY_ID] = 4.0;
				} else {
					variance[ENTRY_ID] = -2.0;
				};
			} else {
				variance[ENTRY_ID] = 0.0;
			};
			if frame % 3 == 0 {
				let f1: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &NONE, &NONE, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, f1, 1.0, 0.5, 3.0);
				EffectModule::set_alpha(weapon.module_accessor, f1, 0.65);
				EffectModule::set_rate(weapon.module_accessor, f1, 1.5);
				if frame >= 2 {
					let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &N1, &NONE, 0.4, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(weapon.module_accessor, f2, 1.0, 0.5, 3.0);
					EffectModule::set_alpha(weapon.module_accessor, f2, 0.65);
					EffectModule::set_rate(weapon.module_accessor, f2, 1.5);
				};
				if frame >= 5 {
					let f3: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_fireflower_shot"), smash::phx::Hash40::new("top"), &N2, &NONE, 0.35, true, 0, 0, 0, 0, 0, true, true) as u32;
					EffectModule::set_rgb(weapon.module_accessor, f3, 1.0, 0.5, 3.0);
					EffectModule::set_alpha(weapon.module_accessor, f3, 0.65);
					EffectModule::set_rate(weapon.module_accessor, f3, 1.5);
				};
			};
			if frame % 5 == 0 {
				let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_smash_flash"), smash::phx::Hash40::new("top"), &NONE, &NONE, 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
			};
			if frame % 20 == 0 {
				EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
				let f2: u32 = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &NONE, &NONE, 2.1, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, f2, 2.75, 0.5, 4.5);
			};
			if frame == 2 {
				F3[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &N1, &NONE, 0.84, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, F3[ENTRY_ID], 2.75, 0.5, 4.5);
				EffectModule::set_alpha(weapon.module_accessor, F3[ENTRY_ID], 0.65);
			};
			if frame == 5 {
				F4[ENTRY_ID] = EffectModule::req_follow(weapon.module_accessor, smash::phx::Hash40::new("sys_sscope_bullet"), smash::phx::Hash40::new("top"), &N2, &NONE, 0.735, true, 0, 0, 0, 0, 0, true, true) as u32;
				EffectModule::set_rgb(weapon.module_accessor, F4[ENTRY_ID], 2.75, 0.5, 4.5);
				EffectModule::set_alpha(weapon.module_accessor, F4[ENTRY_ID], 0.5);
			};
			if frame >= 2 {
				let n1 =  smash::phx::Vector3f { x: 0.0, y: 2.0+variance[ENTRY_ID], z: -15.0 };
				EffectModule::set_pos(boma, F3[ENTRY_ID], &n1);
			};
			if frame >= 5 {
				let n2 =  smash::phx::Vector3f { x: 0.0, y: 8.0-variance[ENTRY_ID], z: -24.0 };
				EffectModule::set_pos(boma, F4[ENTRY_ID], &n2);
			};
		} else {
			EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_sscope_bullet"), false, true);
		};
    }
}
#[weapon_frame( agent = WEAPON_KIND_KIRBY_HAMMER )]
fn finalcutter_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let boma = smash::app::sv_battle_object::module_accessor(otarget_id);
		let ENTRY_ID = WorkModule::get_int(&mut *boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        
		if [hash40("attack_s4_s")].contains(&MotionModule::motion_kind(boma)) {
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("HammerShape"),false);
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("HammerShape.001"),false);
		};
		if [hash40("attack_air_b")].contains(&MotionModule::motion_kind(boma)) {
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("UltraswordMShape"),true);
			ModelModule::set_mesh_visibility(weapon.module_accessor,Hash40::new("UltraswordMShape.001"),true);
		};
    }
}
#[status_script(agent = "kirby", status = FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        smash::app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
		smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_AIR_LASSO | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
	
pub fn install() {
    smashline::install_acmd_scripts!(
		kirby_dtilt,
		kirby_fair,
		kirby_upb1,
		kirby_air_upb2,
		kirby_air_upb2_effect,
		kirby_upb2_sound,
		kirby_upb_sound,
		kirby_upb4,
		kirby_bair,
		kirby_dair,
		kirby_dair_eff,
		kirby_dair_snd,
		kirby_usmash,
		kirby_usmash_eff,
		kirby_usmash_snd,
		kirby_dsmash,
		kirby_dsmash_eff,
		kirby_dsmash_snd,
		kirby_dsmash2,
		kirby_dsmash2_eff,
		kirby_dsmash2_snd,
		kirby_ground_downb,
		kirby_ground_downb_eff,
		kirby_ground_downb_snd,
		kirby_ground_downb2,
		kirby_ground_downb2_eff,
		kirby_ground_downb2_snd,
		kirby_landing_dair,
		kirby_landing_bair,
		kirby_bair_eff,
		kirby_bair_snd,
		kirby_dthrow,
		kirby_upb4_exp,
		kirby_upb4_sound,
		kirby_beam_eff,
		kirby_beam,
		kirby_sideb,
		kirby_sideb_eff,
		kirby_sideb_expr,
		kirby_sideb_start,
		kirby_sideb_snd,
		kirby_fsmash,
		kirby_fsmash_eff,
		kirby_fsmash_snd,
		kirby_fsmash_expr,
		kirby_ftilt,
		kirby_ftilt_eff,
		kirby_ftilt_sound,
		kirby_downb_end_air,
		kirby_special_input,
		kirby_special_input_snd,
		kirby_special_input_eff
    );
    smashline::install_agent_frames!( kirby_frame, ball_frame, finalcutter_frame);
	install_status_scripts!(special_s_pre, exec_downb);
}
