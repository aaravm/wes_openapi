# TaskListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_logs** | Option<[**Vec<models::TaskLog>**](TaskLog.md)> | The logs, and other key info like timing and exit code, for each step in the workflow run. | [optional]
**next_page_token** | Option<**String**> | A token which may be supplied as `page_token` in workflow run task list request to get the next page of results.  An empty string indicates there are no more items to return. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


