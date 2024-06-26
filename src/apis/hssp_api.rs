/*
 * Handy API v2.0.0
 *
 * API to interact with the 'TheHandy'. <p> <b>IMPORTANT</b>: This API is compatible with devices running firmware version 3 (v3) only. See note below on how to deal with firmware version 2 (v2) devices with this API. <p> <h3>Join the community</h3> Send us a mail or add us on discord for a more technical chat - **Handy#8756**<br> Follow us on Reddit for updates and announcements: https://www.reddit.com/r/theHandy/ <p> <h3>Server selection</h3> When using the API make sure you connect to the server environment that the device you are trying to interact with is connected to.<br> For most users this will be the production environment API server: <b>https://www.handyfeeling.com/api/handy/v2</b><br> If you have been given special access to firmware that connects to the staging environment, you should use the staging environment API server: <b>https://staging.handyfeeling.com/api/handy/v2</b><br> <p> <h3>Handling firmware v2 devices</h3> Users with firmware v2 devices are required to update their device to firmware v3 before the device can be fully used with this API.<br> To ease this process, two of the endpoints in this API can be used with firmware v2 devices:<br> <ul> <li><code>/connected</code> - Check online status of a device.</li> <li><code>/info</code> - Get general information about the device.</li> </ul> All other endpoints will always return a 'Machine not connected' error if you try to use them with a firmware v2 device.<br><br> For v2 devices it's important to handle the fwStatus returned from the <code>/info</code> endpoint properly.<br> v2 devices will always have fwStatus = UPDATE_REQUIRED(2). See <code>/info</code> documentation for more details.<br><br> When using the API with a device, you should always start by verifying that the device have a firmware that is compatible with the API you are using in your service.<br> An example flow could look like this: <ol> <li>Check if device is conncted with <code>/connected</code></li> <li>Check device firmware status with <code>/info</code></li> <li>If a firmware update is required, forward the user to <a href=\"https://www.handfyfeeling.com\">https://www.handfyfeeling.com</a> so they can easily update the device firmware.</li> <li>Continue your service when the firmware status returned in <code>/info</code> is UP_TO_DATE(0).</li> </ol> <p> <h3>Mode specific operations</h3> Operations in <b>BASE</b>, <b>SLIDE</b>, <b>TIMESYNC</b> and <b>HSTP</b> are mode independent and available at any time.<br> Other operations are only available in a specific mode (eg. <b>HAMP</b>, <b>HDSP</b>, <b>HSSP</b>).<br> To access these operations, the device have to first be put in the specific mode (see <code>/mode</code>).<br> If you execute an operation not available in the current mode of the device, you will receive a 'No such method' error response. <!--See live samples of use cases here: 'https://www.handyfeeling.com/api/handy/v2/demo/ -->
 *
 * The version of the OpenAPI document: 2.0.0-beta-3
 * Contact: lars@sweettech.no
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;
use crate::models;

/// struct for typed errors of method [`get_hssp_state`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHsspStateError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_loop_setting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLoopSettingError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`hssp_stop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HsspStopError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`play`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_loop_setting`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetLoopSettingError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`setup`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetupError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// Get the HSSP state of the device.
pub async fn get_hssp_state(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
) -> Result<models::GetHsspState200Response, Error<GetHsspStateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/state", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetHsspStateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the HSSP loop setting of the device. Only available in firmware >= 3.2.x
pub async fn get_loop_setting(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
) -> Result<models::GetLoopSetting200Response, Error<GetLoopSettingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/loop", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLoopSettingError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Stop script playing.
pub async fn hssp_stop(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
) -> Result<models::SetHampVelocityPercent200Response, Error<HsspStopError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/stop", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<HsspStopError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Start script playing from a specified time index. <p>For the script and a video to be correctly synchronized, the client must provide a client-side-estimated-server-time.<p>See TIMESYNC for further details on how to create a good client-side-estimated-server-time.
pub async fn play(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
    hssp_play: models::HsspPlay,
) -> Result<models::Play200Response, Error<PlayError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/play", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&hssp_play);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PlayError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// If looping is enabled, the device will start replaying the script from the beginning when the end of the script is reached. Only available in firmware >= 3.2.x
pub async fn set_loop_setting(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
    loop_setting_update: models::LoopSettingUpdate,
) -> Result<models::SetHampVelocityPercent200Response, Error<SetLoopSettingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/loop", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&loop_setting_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetLoopSettingError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Setup script synchronization by providing the device with an URL from where the script can be downloaded. The device need to be able to access the URL for setup to work. <p><b>NOTE: The maximum CSV script size that a device can handle is 524288 bytes. Script larger than this will return an error.</b></p> <p>If the sha-256 value of the script is provided, the device will only download the script if it can not be found in the device cache.</p> <p>See the hssp/setup response examples and the HSSPModeErrors schema for possible responses and error code descriptions.</p> <h2>Using token URLs</h2> <p>NOTE: This secions is only applicable for partners that have integrated with the Script API.</p> <p>If the URL provided to the hssp/setup is a token URL, the sha-256 value is ignored. In addition, more information is available in potential error responses if the error is caused by the token.</p>
pub async fn setup(
    configuration: &configuration::Configuration,
    x_connection_key: &str,
    setup: models::Setup,
) -> Result<models::Setup200Response, Error<SetupError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/hssp/setup", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("X-Connection-Key", x_connection_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&setup);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetupError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
