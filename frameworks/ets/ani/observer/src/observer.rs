// Copyright (C) 2025 Huawei Device Co., Ltd.
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ani_rs::{business_error::BusinessError, objects::AniFnObject, AniEnv};

use crate::{
    bridge::{ObserverOptions, DEFAULT_SIM_SLOT_ID},
    register::{CallbackFlavor, EventListener, Register, TelephonyUpdateEventType}
};

#[ani_rs::native]
pub fn on_cellular_data_flow_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCellularDataFlowUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::CellularDataFlowChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_cellular_data_flow_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCellularDataFlowUpdate,
        options.slot_id,
        CallbackFlavor::CellularDataFlowChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_cellular_data_flow_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::CellularDataFlowChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventCellularDataFlowUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_icc_account_info_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventIccAccountChange,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::IccAccountInfoChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_icc_account_info_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::IccAccountInfoChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventIccAccountChange,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_sim_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventSimStateUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::SimStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_sim_state_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventSimStateUpdate,
        options.slot_id,
        CallbackFlavor::SimStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_sim_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::SimStateChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventSimStateUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_signal_info_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventSignalStrengthsUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::SignalInfoChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_signal_info_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventSignalStrengthsUpdate,
        options.slot_id,
        CallbackFlavor::SignalInfoChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_signal_info_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::SignalInfoChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventSignalStrengthsUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_cell_info_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCellInfoUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::CellInfoChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_cell_info_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCellInfoUpdate,
        options.slot_id,
        CallbackFlavor::CellInfoChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_cell_info_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::CellInfoChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventCellInfoUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_cellular_data_connection_state_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventDataConnectionUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::CellularDataConnectionStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_cellular_data_connection_state_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventDataConnectionUpdate,
        options.slot_id,
        CallbackFlavor::CellularDataConnectionStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_cellular_data_connection_state_change(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::CellularDataConnectionStateChange(
            callback_global,
        ))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventDataConnectionUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_network_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventNetworkStateUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::NetworkStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_network_state_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventNetworkStateUpdate,
        options.slot_id,
        CallbackFlavor::NetworkStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_network_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::NetworkStateChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventNetworkStateUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_call_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCallStateUpdate,
        DEFAULT_SIM_SLOT_ID,
        CallbackFlavor::CallStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn on_call_state_change_option(
    env: &AniEnv,
    options: ObserverOptions,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventCallStateUpdate,
        options.slot_id,
        CallbackFlavor::CallStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_call_state_change(env: &AniEnv, callback: AniFnObject) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::CallStateChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventCallStateUpdate,
        callback_flavor,
    )?;

    Ok(())
}

#[ani_rs::native]
pub fn on_get_sim_active_state(
    env: &AniEnv,
    slot_id: i32,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_global = callback.into_global_callback(env).unwrap();
    let listener = EventListener::new(
        TelephonyUpdateEventType::EventSimActiveStateUpdate,
        slot_id,
        CallbackFlavor::SimActiveStateChange(callback_global),
    );
    Register::get_instance().register(listener)?;

    Ok(())
}

#[ani_rs::native]
pub fn off_get_sim_active_state(
    env: &AniEnv,
    callback: AniFnObject,
) -> Result<(), BusinessError> {
    let callback_flavor = if env.is_undefined(&callback).unwrap() {
        None
    } else {
        let callback_global = callback.into_global_callback(env).unwrap();
        Some(CallbackFlavor::SimActiveStateChange(callback_global))
    };

    Register::get_instance().unregister(
        TelephonyUpdateEventType::EventSimActiveStateUpdate,
        callback_flavor,
    )?;

    Ok(())
}
fn check_communication_permission() -> Result<(), BusinessError> {
    if !crate::wrapper::ffi::HasCommunicationPermission() {
        return Err(BusinessError::new_static(201, "Permission denied: ohos.permission.GET_NETWORK_INFO."));
    }
    Ok(())
}

#[ani_rs::native]
pub fn on_communication_state_change(env: &AniEnv, callback: AniFnObject, slot_id: i32) -> Result<(), BusinessError> {
    check_communication_permission()?;
    let callback = callback.into_global_callback(env).map_err(|_| BusinessError::new_static(401, "Invalid callback."))?;
    Register::get_instance().register(EventListener::new(TelephonyUpdateEventType::EventCommunicationStateUpdate,
        slot_id, CallbackFlavor::CommunicationStateChange(callback)))
}

#[ani_rs::native]
pub fn off_communication_state_change(env: &AniEnv, callback: AniFnObject, slot_id: i32) -> Result<(), BusinessError> {
    check_communication_permission()?;
    if !crate::wrapper::ffi::IsValidSlotIdEx(slot_id, TelephonyUpdateEventType::EventCommunicationStateUpdate.to_u32()) {
        return Err(BusinessError::new_static(8300001, "Invalid slotId."));
    }
    let callback = callback.into_global_callback(env).map_err(|_| BusinessError::new_static(401, "Invalid callback."))?;
    Register::get_instance().unregister_communication(slot_id, CallbackFlavor::CommunicationStateChange(callback))
}
