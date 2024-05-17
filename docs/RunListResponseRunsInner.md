# RunListResponseRunsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | **String** |  | 
**state** | Option<[**models::State**](State.md)> |  | [optional]
**start_time** | Option<**String**> | When the run started executing, in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\" | [optional]
**end_time** | Option<**String**> | When the run stopped executing (completed, failed, or cancelled), in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\" | [optional]
**tags** | **std::collections::HashMap<String, String>** | Arbitrary key/value tags added by the client during run creation | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


