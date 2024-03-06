
use super::*;

unsafe extern "C" fn game_attackcommand4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    let strength = agent.get_int(*FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    let remaining_startup = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        6.0
    } else if strength == *FIGHTER_RYU_STRENGTH_W {
        3.0
    } else if strength == FIGHTER_RYU_STRENGTH_M {
        6.0
    } else {
        15.0
    };
    FT_MOTION_RATE_RANGE(agent, 6.0, 15.0, remaining_startup);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::drain_direct(agent.battle_object, 2.0 * MeterModule::meter_per_level(agent.battle_object));
        }
    }
    frame(agent.lua_state_agent, 9.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        MeterModule::watch_damage(agent.battle_object, true);
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            MeterModule::watch_damage(agent.battle_object, false);
        }
        ATTACK(agent, 0, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 16.0, 361, 98, 0, 26, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("kneel"), 17.5, 361, 98, 0, 26, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 20.0);
    let recovery = if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        33.0
    } else if strength == *FIGHTER_RYU_STRENGTH_W {
        22.0
    } else if strength == FIGHTER_RYU_STRENGTH_M {
        19.0
    } else {
        16.0
    };
    FT_MOTION_RATE_RANGE(agent, 20.0, 46.0, recovery);
    if is_excute(agent) {
        MeterModule::watch_damage(agent.battle_object, false);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 46.0);
    FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_attackcommand4(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FLASH(agent, 0.95, 0.522, 0.051, 1.7);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FLASH(agent, 0.95, 0.522, 0.051, 0.7);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        if VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
            FLASH(agent, 0.95, 0.522, 0.051, 1.7);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("legl"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("ryu_savingattack_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(agent, Hash40::new("ryu_attack_line"), Hash40::new("ryu_attack_line"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
        EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FLASH(agent, 0.95, 0.522, 0.051, 0.7);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FLASH(agent, 0.95, 0.522, 0.051, 1.7);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        FLASH(agent, 0.95, 0.522, 0.051, 0.7);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent)
    && VarModule::is_flag(agent.battle_object, vars::shotos::instance::IS_USE_EX_SPECIAL) {
        EFFECT_OFF_KIND(agent, Hash40::new("ryu_savingattack_aura"), true, true);
        COL_NORMAL(agent);
    }
}

unsafe extern "C" fn sound_attackcommand4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ryu_smash_s01"));
        PLAY_SE(agent, Hash40::new("vc_ryu_smash_s01"));
    }
    wait(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        PLAY_STEP(agent, Hash40::new("se_ryu_step_left_m"));
    }
}

unsafe extern "C" fn expression_attackcommand4(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(agent, 0, 0.8, 180, 8, 0.8, -10, 7, 20, 14, 80);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    smashline::Agent::new("ryu")
        .acmd("game_attackcommand4", game_attackcommand4)
        .acmd("effect_attackcommand4", effect_attackcommand4)
        .acmd("sound_attackcommand4", sound_attackcommand4)
        .acmd("expression_attackcommand4", expression_attackcommand4)
        .install();
}
