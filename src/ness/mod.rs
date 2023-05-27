use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
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
use smash::phx::Vector3f;
use crate::util::*;


#[acmd_script(
    agent = "ness",
    script = "game_throwlw",
    category = ACMD_GAME,
	low_priority)]
unsafe fn ness_fthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
	    macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
	macros::FT_LEAVE_NEAR_OTTOTTO(fighter, 1.5, 1.5);
	macros::ATTACK_ABS(fighter,*FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 70, 80, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
	macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE,*ATTACK_REGION_THROW);
	}
	frame(fighter.lua_state_agent, 6.0);
	for _ in 0..3 {
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 100, 0, 0, 4.3, 2.0, 1.8, 0.0, Some(-2.0), Some(1.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F,false, 0, 0.0,0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
			AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
		}
			wait(fighter.lua_state_agent, 4.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
		}
	}
		frame(fighter.lua_state_agent, 26.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361,100, 0, 0, 6.0, 2.0, 2.2, 0.0, Some(-2.0), Some(2.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA,*COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false,Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M,*COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
			AttackModule::set_catch_only_all(fighter.module_accessor,true, false);
			CHECK_FINISH_CAMERA(2);
			

			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 15.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {


			macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), 	WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),  WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
		
		}
			wait(fighter.lua_state_agent, 3.0);
			if macros::is_excute(fighter) {
				AttackModule::clear_all(fighter.module_accessor);
			}
			
		}

pub fn install() {
    smashline::install_acmd_scripts!(
	ness_fthrow
    );
	
}