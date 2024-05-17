# RunLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**String**> | workflow run ID | [optional]
**request** | Option<[**models::RunRequest**](RunRequest.md)> |  | [optional]
**state** | Option<[**models::State**](State.md)> |  | [optional]
**run_log** | Option<[**models::Log**](Log.md)> |  | [optional]
**task_logs_url** | Option<**String**> | A reference to the complete url which may be used to obtain a paginated list of task logs for this workflow | [optional]
**task_logs** | Option<[**Vec<models::RunLogTaskLogsInner>**](RunLog_task_logs_inner.md)> | The logs, and other key info like timing and exit code, for each step in the workflow run. This field is deprecated and the `task_logs_url` should be used to retrieve a paginated list of steps from the workflow run. This field will be removed in the next major version of the  specification (2.0.0) | [optional]
**outputs** | Option<[**serde_json::Value**](.md)> | The outputs from the workflow run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


