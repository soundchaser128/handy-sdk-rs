# Rust API client for openapi

API to interact with the 'TheHandy'. <p> <b>IMPORTANT</b>: This API is compatible with devices running firmware version 3 (v3) only. See note below on how to deal with firmware version 2 (v2) devices with this API. <p> <h3>Join the community</h3> Send us a mail or add us on discord for a more technical chat - **Handy#8756**<br> Follow us on Reddit for updates and announcements: https://www.reddit.com/r/theHandy/ <p> <h3>Server selection</h3> When using the API make sure you connect to the server environment that the device you are trying to interact with is connected to.<br> For most users this will be the production environment API server: <b>https://www.handyfeeling.com/api/handy/v2</b><br> If you have been given special access to firmware that connects to the staging environment, you should use the staging environment API server: <b>https://staging.handyfeeling.com/api/handy/v2</b><br> <p> <h3>Handling firmware v2 devices</h3> Users with firmware v2 devices are required to update their device to firmware v3 before the device can be fully used with this API.<br> To ease this process, two of the endpoints in this API can be used with firmware v2 devices:<br> <ul> <li><code>/connected</code> - Check online status of a device.</li> <li><code>/info</code> - Get general information about the device.</li> </ul> All other endpoints will always return a 'Machine not connected' error if you try to use them with a firmware v2 device.<br><br> For v2 devices it's important to handle the fwStatus returned from the <code>/info</code> endpoint properly.<br> v2 devices will always have fwStatus = UPDATE_REQUIRED(2). See <code>/info</code> documentation for more details.<br><br> When using the API with a device, you should always start by verifying that the device have a firmware that is compatible with the API you are using in your service.<br> An example flow could look like this: <ol> <li>Check if device is conncted with <code>/connected</code></li> <li>Check device firmware status with <code>/info</code></li> <li>If a firmware update is required, forward the user to <a href=\"https://www.handfyfeeling.com\">https://www.handfyfeeling.com</a> so they can easily update the device firmware.</li> <li>Continue your service when the firmware status returned in <code>/info</code> is UP_TO_DATE(0).</li> </ol> <p> <h3>Mode specific operations</h3> Operations in <b>BASE</b>, <b>SLIDE</b>, <b>TIMESYNC</b> and <b>HSTP</b> are mode independent and available at any time.<br> Other operations are only available in a specific mode (eg. <b>HAMP</b>, <b>HDSP</b>, <b>HSSP</b>).<br> To access these operations, the device have to first be put in the specific mode (see <code>/mode</code>).<br> If you execute an operation not available in the current mode of the device, you will receive a 'No such method' error response. <!--See live samples of use cases here: 'https://www.handyfeeling.com/api/handy/v2/demo/ -->


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0.0-beta-3
- Package version: 2.0.0-beta-3
- Generator version: 7.5.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://www.handyfeeling.com/api/handy/v2*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*BaseApi* | [**get_info**](docs/BaseApi.md#get_info) | **GET** /info | Get extended device information.
*BaseApi* | [**get_mode**](docs/BaseApi.md#get_mode) | **GET** /mode | Get the current mode of the device.
*BaseApi* | [**get_settings**](docs/BaseApi.md#get_settings) | **GET** /settings | Extended device settings.
*BaseApi* | [**get_status**](docs/BaseApi.md#get_status) | **GET** /status | Get the device status.
*BaseApi* | [**is_connected**](docs/BaseApi.md#is_connected) | **GET** /connected | Check device connectivity.
*BaseApi* | [**set_mode**](docs/BaseApi.md#set_mode) | **PUT** /mode | Set the current mode of the device.
*HampApi* | [**get_hamp_state**](docs/HampApi.md#get_hamp_state) | **GET** /hamp/state | Get the HAMP state of the device.
*HampApi* | [**get_hamp_velocity_percent**](docs/HampApi.md#get_hamp_velocity_percent) | **GET** /hamp/velocity | Get the HAMP velocity setting of the device in percent.
*HampApi* | [**hamp_stop**](docs/HampApi.md#hamp_stop) | **PUT** /hamp/stop | Stop alternating motion.
*HampApi* | [**set_hamp_velocity_percent**](docs/HampApi.md#set_hamp_velocity_percent) | **PUT** /hamp/velocity | Set the HAMP velocity setting of the device in percent.
*HampApi* | [**start**](docs/HampApi.md#start) | **PUT** /hamp/start | Start alternating motion.
*HdspApi* | [**next_position_abs_in_time**](docs/HdspApi.md#next_position_abs_in_time) | **PUT** /hdsp/xat | Sets the next absolute position (xa) of the device, and the time (t) the device should use to reach the position.
*HdspApi* | [**next_position_percent_in_time**](docs/HdspApi.md#next_position_percent_in_time) | **PUT** /hdsp/xpt | Sets the next percent position (xp) of the device, and the time (t) the device should use to reach the position.
*HdspApi* | [**next_position_percent_velocity_absolute**](docs/HdspApi.md#next_position_percent_velocity_absolute) | **PUT** /hdsp/xpva | Sets the next percent position (xp) of the device, and the absolute velocity (va) the device should use to reach the position.
*HdspApi* | [**next_position_percent_velocity_percent**](docs/HdspApi.md#next_position_percent_velocity_percent) | **PUT** /hdsp/xpvp | Sets the next percent position (xp) of the device, and the percent velocity (vp) the device should use to reach the position.
*HdspApi* | [**next_postion_abs_velocity_abs**](docs/HdspApi.md#next_postion_abs_velocity_abs) | **PUT** /hdsp/xava | Sets the next absolute position (xa) of the device, and the absolute velocity (va) the device should use to reach the position.
*HsspApi* | [**get_hssp_state**](docs/HsspApi.md#get_hssp_state) | **GET** /hssp/state | Get the HSSP state of the device.
*HsspApi* | [**get_loop_setting**](docs/HsspApi.md#get_loop_setting) | **GET** /hssp/loop | Get the HSSP loop setting of the device. Only available in firmware >= 3.2.x
*HsspApi* | [**hssp_stop**](docs/HsspApi.md#hssp_stop) | **PUT** /hssp/stop | Stop script playing.
*HsspApi* | [**play**](docs/HsspApi.md#play) | **PUT** /hssp/play | Start script playing.
*HsspApi* | [**set_loop_setting**](docs/HsspApi.md#set_loop_setting) | **PUT** /hssp/loop | Set the HSSP loop setting of the device. Only available in firmware >= 3.2.x
*HsspApi* | [**setup**](docs/HsspApi.md#setup) | **PUT** /hssp/setup | Setup script synchronization.
*HstpApi* | [**get_device_time**](docs/HstpApi.md#get_device_time) | **GET** /hstp/time | Get the current time of the device.
*HstpApi* | [**get_offset**](docs/HstpApi.md#get_offset) | **GET** /hstp/offset | Get the HSTP offset of the device.
*HstpApi* | [**get_round_trip_delay**](docs/HstpApi.md#get_round_trip_delay) | **GET** /hstp/rtd | Get the round-trip-delay-time (rtd) between the device and the server.
*HstpApi* | [**set_offset**](docs/HstpApi.md#set_offset) | **PUT** /hstp/offset | Set the HSTP offset of the device.
*HstpApi* | [**sync**](docs/HstpApi.md#sync) | **GET** /hstp/sync | Syncronize the connected device with the server clock.
*MaintenanceApi* | [**get_update_status**](docs/MaintenanceApi.md#get_update_status) | **GET** /maintenance/update/status | Get the device update status.
*MaintenanceApi* | [**restart**](docs/MaintenanceApi.md#restart) | **PUT** /maintenance/restart | Restart the device.
*MaintenanceApi* | [**update_perform_fw**](docs/MaintenanceApi.md#update_perform_fw) | **PUT** /maintenance/update/perform | Perform firmware update.
*OtaApi* | [**latest**](docs/OtaApi.md#latest) | **GET** /ota/latest | Get the latest available firmware available for the provided model and branch.
*SlideApi* | [**get_position_abs**](docs/SlideApi.md#get_position_abs) | **GET** /slide/position/absolute | Get the current slide position
*SlideApi* | [**get_slide**](docs/SlideApi.md#get_slide) | **GET** /slide | Get the slide settings.
*SlideApi* | [**set_slide**](docs/SlideApi.md#set_slide) | **PUT** /slide | Set slide settings.
*TimesyncApi* | [**get_server_time**](docs/TimesyncApi.md#get_server_time) | **GET** /servertime | Get current server time.


## Documentation For Models

 - [BaseModeErrors](docs/BaseModeErrors.md)
 - [Branch](docs/Branch.md)
 - [ConnectedResponse](docs/ConnectedResponse.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [ErrorResponseError](docs/ErrorResponseError.md)
 - [ErrorResponseErrorCode](docs/ErrorResponseErrorCode.md)
 - [Events](docs/Events.md)
 - [FirmwareStatus](docs/FirmwareStatus.md)
 - [GenericError](docs/GenericError.md)
 - [GenericResult](docs/GenericResult.md)
 - [GenericResultResponse](docs/GenericResultResponse.md)
 - [GetDeviceTime200Response](docs/GetDeviceTime200Response.md)
 - [GetDeviceTime200ResponseOneOf](docs/GetDeviceTime200ResponseOneOf.md)
 - [GetHampState200Response](docs/GetHampState200Response.md)
 - [GetHampState200ResponseOneOf](docs/GetHampState200ResponseOneOf.md)
 - [GetHampVelocityPercent200Response](docs/GetHampVelocityPercent200Response.md)
 - [GetHsspState200Response](docs/GetHsspState200Response.md)
 - [GetHsspState200ResponseOneOf](docs/GetHsspState200ResponseOneOf.md)
 - [GetInfo200Response](docs/GetInfo200Response.md)
 - [GetLoopSetting200Response](docs/GetLoopSetting200Response.md)
 - [GetMode200Response](docs/GetMode200Response.md)
 - [GetMode200ResponseOneOf](docs/GetMode200ResponseOneOf.md)
 - [GetOffset200Response](docs/GetOffset200Response.md)
 - [GetPositionAbs200Response](docs/GetPositionAbs200Response.md)
 - [GetRoundTripDelay200Response](docs/GetRoundTripDelay200Response.md)
 - [GetRoundTripDelay200ResponseOneOf](docs/GetRoundTripDelay200ResponseOneOf.md)
 - [GetSettings200Response](docs/GetSettings200Response.md)
 - [GetSlide200Response](docs/GetSlide200Response.md)
 - [GetStatus200Response](docs/GetStatus200Response.md)
 - [GetUpdateStatus200Response](docs/GetUpdateStatus200Response.md)
 - [HampModeErrors](docs/HampModeErrors.md)
 - [HampResponse](docs/HampResponse.md)
 - [HampStartResponse](docs/HampStartResponse.md)
 - [HampState](docs/HampState.md)
 - [HampStop200Response](docs/HampStop200Response.md)
 - [HampStopResponse](docs/HampStopResponse.md)
 - [HampVelocityPercent](docs/HampVelocityPercent.md)
 - [HampVelocityPercentResponse](docs/HampVelocityPercentResponse.md)
 - [HdspModeErrors](docs/HdspModeErrors.md)
 - [HdspRequest](docs/HdspRequest.md)
 - [HdspResponse](docs/HdspResponse.md)
 - [HdspResult](docs/HdspResult.md)
 - [HdspTimeRequest](docs/HdspTimeRequest.md)
 - [HsspModeErrors](docs/HsspModeErrors.md)
 - [HsspPlay](docs/HsspPlay.md)
 - [HsspPlayResponse](docs/HsspPlayResponse.md)
 - [HsspPlayResult](docs/HsspPlayResult.md)
 - [HsspResponse](docs/HsspResponse.md)
 - [HsspSetupResult](docs/HsspSetupResult.md)
 - [HsspState](docs/HsspState.md)
 - [InfoResponse](docs/InfoResponse.md)
 - [LoopSettingResponse](docs/LoopSettingResponse.md)
 - [LoopSettingUpdate](docs/LoopSettingUpdate.md)
 - [MaintenanceModeErrors](docs/MaintenanceModeErrors.md)
 - [Mode](docs/Mode.md)
 - [ModeUpdate](docs/ModeUpdate.md)
 - [ModeUpdateResponse](docs/ModeUpdateResponse.md)
 - [Model](docs/Model.md)
 - [NextPostionAbsVelocityAbs200Response](docs/NextPostionAbsVelocityAbs200Response.md)
 - [NextXat](docs/NextXat.md)
 - [NextXava](docs/NextXava.md)
 - [NextXpt](docs/NextXpt.md)
 - [NextXpva](docs/NextXpva.md)
 - [NextXpvp](docs/NextXpvp.md)
 - [OffsetResponse](docs/OffsetResponse.md)
 - [OffsetUpdate](docs/OffsetUpdate.md)
 - [OtaLatest](docs/OtaLatest.md)
 - [OtaLatestResponse](docs/OtaLatestResponse.md)
 - [Play200Response](docs/Play200Response.md)
 - [PositionAbsoluteResponse](docs/PositionAbsoluteResponse.md)
 - [RestartRequest](docs/RestartRequest.md)
 - [RpcResult](docs/RpcResult.md)
 - [ServerTimeResponse](docs/ServerTimeResponse.md)
 - [SetHampVelocityPercent200Response](docs/SetHampVelocityPercent200Response.md)
 - [SetMode200Response](docs/SetMode200Response.md)
 - [SetSlide200Response](docs/SetSlide200Response.md)
 - [SettingsResponse](docs/SettingsResponse.md)
 - [Setup](docs/Setup.md)
 - [Setup200Response](docs/Setup200Response.md)
 - [Setup200ResponseOneOf](docs/Setup200ResponseOneOf.md)
 - [SlideResponse](docs/SlideResponse.md)
 - [SlideResult](docs/SlideResult.md)
 - [SlideResultResponse](docs/SlideResultResponse.md)
 - [SlideSettings](docs/SlideSettings.md)
 - [SlideSettingsAnyOf](docs/SlideSettingsAnyOf.md)
 - [SlideSettingsAnyOf1](docs/SlideSettingsAnyOf1.md)
 - [SlideSettingsAnyOf2](docs/SlideSettingsAnyOf2.md)
 - [SlideUpdateResponse](docs/SlideUpdateResponse.md)
 - [SliderMaxResponse](docs/SliderMaxResponse.md)
 - [SliderMinResponse](docs/SliderMinResponse.md)
 - [Start200Response](docs/Start200Response.md)
 - [StateResult](docs/StateResult.md)
 - [StatusResponse](docs/StatusResponse.md)
 - [StatusResponseState](docs/StatusResponseState.md)
 - [Sync200Response](docs/Sync200Response.md)
 - [Sync200ResponseOneOf](docs/Sync200ResponseOneOf.md)
 - [SyncResponse](docs/SyncResponse.md)
 - [UpdatePerform](docs/UpdatePerform.md)
 - [UpdateStatusResponse](docs/UpdateStatusResponse.md)
 - [WebHook](docs/WebHook.md)
 - [WebHookOpts](docs/WebHookOpts.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

lars@sweettech.no

