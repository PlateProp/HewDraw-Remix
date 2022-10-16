use super::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    let kinetic;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        motion = Hash40::new("special_air_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        motion = Hash40::new("special_lw_start");
        kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    // <HDR>
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));
    // </HDR>
    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if situation == *SITUATION_KIND_AIR
    && param_1.get_bool() {
        if VarModule::is_flag(fighter.battle_object, vars::donkey::status::SPECIAL_AIR_LW_STOP) {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_STOP,
                0.0,
                0.5
            );
            VarModule::off_flag(fighter.battle_object, vars::donkey::status::SPECIAL_AIR_LW_STOP);
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(fighter.module_accessor)
        && fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let status = if situation != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_DONKEY_STATUS_KIND_SPECIAL_LW_LOOP
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_main
    );
}
