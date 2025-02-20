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
    agent = "gaogaen",
    script =  "game_attackdash",
    category = ACMD_GAME,
	low_priority)]
unsafe fn incin_da(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 8.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, false);
			if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 13.0, /*Angle*/ 55, /*KBG*/ 81, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 5.4, /*X*/ 3.5, /*Y*/ 1.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KNEE);
			}
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
				macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("legl"), /*Damage*/ 9.0, /*Angle*/ 55, /*KBG*/ 81, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 4.8, /*X*/ 3.5, /*Y*/ 1.2, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KNEE);
			}
		}
		wait(fighter.lua_state_agent, 4.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			JostleModule::set_status(fighter.module_accessor, true);
		}
		frame(fighter.lua_state_agent, 20.0);
		if macros::is_excute(fighter) {
			if StatusModule::situation_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent)) != *SITUATION_KIND_AIR {  
				WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
			}
		}
}	
#[acmd_script(
    agent = "gaogaen",
    script =  "game_attacks3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn incin_ftilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 12.0);
		if macros::is_excute(fighter) {

			macros::ATTACK(fighter, 0, 0,Hash40::new("handl"),13.0, 31, 85,0, 54, 3.4,3.4, 0.0, 0.0, None, None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, 1, 0,Hash40::new("arml"),12.0, 31, 85,0, 54, 3.4,0.8, 0.0, 0.0, None, None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, 2, 0,Hash40::new("shoulderl"),12.0, 31, 85,0, 54, 3.6,0.0, -1.8, 1.4, None, None,None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON,*ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
			AttackModule::set_poison_param(fighter.module_accessor, 0, 361, 45, 2.0, false); 
}
wait(fighter.lua_state_agent, 3.0);

if macros::is_excute(fighter) {
AttackModule::clear_all(fighter.module_accessor);
}
}
		
pub fn install() {
    smashline::install_acmd_scripts!(
		incin_da,
		incin_ftilt
    );
}
