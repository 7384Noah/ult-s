use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackairf",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_fair1(fighter: &mut L2CAgentBase) {
    	let lua_state = fighter.lua_state_agent;
    	frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 0, true);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 50, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 9.2, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(9.2), /*Z2*/ Some(7.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 28, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 14.8, /*Z*/ 3.2, /*X2*/ Some(0.0), /*Y2*/ Some(14.8), /*Z2*/ Some(7.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 70, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(11.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 45, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.8, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 5.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(11.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 2.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 2.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 2.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 2.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
		}
		frame(fighter.lua_state_agent, 30.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackairf2",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_fair2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 0, true);
		}
		frame(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
		frame(fighter.lua_state_agent, 7.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 56, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 8.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.5), /*Z2*/ Some(11.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 44, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 3.7, /*X*/ 0.0, /*Y*/ 15.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(15.5), /*Z2*/ Some(11.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 82, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 50, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 60, /*KBG*/ 10, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.4, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 8.0, /*X2*/ Some(0.0), /*Y2*/ Some(12.0), /*Z2*/ Some(15.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 5.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 5.0, /*Unk*/ false);
		}
		wait(fighter.lua_state_agent, 3.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		}
		wait(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		}
}	
//AttackModule::set_no_damage_fly_smoke_all(true, false)
#[acmd_script(
    agent = "bayonetta",
    script =  "game_specials",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_sideb_start(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
		}
		frame(fighter.lua_state_agent, 15.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.0, /*Angle*/ 45, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 100, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-8.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 17.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.0, /*Angle*/ 47, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-8.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 19.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 7.5, /*Angle*/ 50, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 83, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-8.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 25.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 7.0, /*Angle*/ 62, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 68, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-8.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.9, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
		}
		frame(fighter.lua_state_agent, 32.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HOLD_END_CHECK);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		}
		frame(fighter.lua_state_agent, 40.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
		}
		frame(fighter.lua_state_agent, 40.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.843);
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_specialsholdend",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_sideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 0, true);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footr"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 60, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(-8.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.0, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
		}
		frame(fighter.lua_state_agent, 23.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
		}
		frame(fighter.lua_state_agent, 34.0);
		frame(fighter.lua_state_agent, 37.0);
		if macros::is_excute(fighter) {
			macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.75);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 98, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 5.3, /*X*/ 13.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 98, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 5.3, /*X*/ 7.5, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneer"), /*Damage*/ 5.0, /*Angle*/ 98, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 4.0, /*X*/ 1.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("waist"), /*Damage*/ 5.0, /*Angle*/ 90, /*KBG*/ 65, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ -0.8, /*Z*/ -1.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.3, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_NO_FLOOR, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		frame(fighter.lua_state_agent, 44.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attack11",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_jab1(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10, 3, 10, 5, true);
		}
		frame(fighter.lua_state_agent, 2.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.25);
		frame(fighter.lua_state_agent, 6.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.4, /*Angle*/ 361, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 36, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 7.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.4, /*Angle*/ 361, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 36, /*Size*/ 2.6, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.4, /*Angle*/ 361, /*KBG*/ 8, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 2.2, /*X*/ 0.0, /*Y*/ 5.5, /*Z*/ 3.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.4, /*Angle*/ 180, /*KBG*/ 6, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_FIGHTER, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 4, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.4, /*Angle*/ 361, /*KBG*/ 6, /*FKB*/ 0, /*BKB*/ 32, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 11.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 0, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 1, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 2, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 3, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(fighter.module_accessor, /*ID*/ 4, /*Frames*/ 8.0, /*Unk*/ false);
		}
		frame(fighter.lua_state_agent, 13.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
		}
		frame(fighter.lua_state_agent, 14.0);
		if macros::is_excute(fighter) {
			WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
		}
}	
	
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attacklw3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 3, 15, 5, true);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.5);
		frame(fighter.lua_state_agent, 12.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.8);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.0, /*Angle*/ 85, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.0), /*Z2*/ Some(3.5), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 95, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 2.8, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.0), /*Z2*/ Some(11.0), /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.3, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
		wait(fighter.lua_state_agent, 2.0);
		if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		}
}
#[acmd_script(
    agent = "bayonetta",
    script =  "game_attackhi3",
    category = ACMD_GAME,
	low_priority)]
unsafe fn bayo_utilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
			notify_event_msc_cmd!(fighter, 0x2d51fcdb09u64, *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, false, false, 10, 3, 10, 5, true);
		}
		frame(fighter.lua_state_agent, 7.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
		if macros::is_excute(fighter) {
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 85, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.5), /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 1.5, /*Angle*/ 110, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 9.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(9.5), /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 9.0);
		if macros::is_excute(fighter) {
			AttackModule::clear(fighter.module_accessor, /*ID*/ 1, false);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 128, /*KBG*/ 27, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 18.0, /*Z*/ 7.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.8, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		frame(fighter.lua_state_agent, 11.0);
		if macros::is_excute(fighter) {
			AttackModule::clear_all(fighter.module_accessor);
			macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 93, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 25.5, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(25.5), /*Z2*/ Some(2.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
			macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 93, /*KBG*/ 60, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 2.0, /*X*/ 0.0, /*Y*/ 20.0, /*Z*/ -6.0, /*X2*/ Some(0.0), /*Y2*/ Some(20.0), /*Z2*/ Some(6.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
		}
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 3);
		frame(fighter.lua_state_agent, 12.0);
		frame(fighter.lua_state_agent, 13.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.733);
		frame(fighter.lua_state_agent, 28.0);
		macros::FT_MOTION_RATE(fighter, /*FSM*/ 1);
}
#[fighter_frame( agent = FIGHTER_KIND_BAYONETTA )]
fn bayo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let motion_kind = MotionModule::motion_kind(boma);
		let frame = MotionModule::frame(boma);
		let can_sideb = !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
		let can_upb = !WorkModule::is_flag(boma, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
		let stick_y = ControlModule::get_stick_y(boma);
		let cat1 = ControlModule::get_command_flag_cat(boma, 0);
		if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F {
			if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME) < 1{
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && can_sideb {
					if stick_y <= -0.5 {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, true);
					} else {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, true);
					};
				};
			};
		};
    }
}	
pub fn install() {
    smashline::install_acmd_scripts!(
		bayo_fair1,
		bayo_fair2,
		bayo_sideb,
		bayo_dtilt,
		bayo_utilt,
		bayo_jab1,
		bayo_sideb_start
    );
    smashline::install_agent_frames!(
        bayo_frame
    );
}
