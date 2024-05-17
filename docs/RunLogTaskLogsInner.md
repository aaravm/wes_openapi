# RunLogTaskLogsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The task or workflow name | 
**cmd** | Option<**Vec<String>**> | The command line that was executed | [optional]
**start_time** | Option<**String**> | When the command started executing, in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\" | [optional]
**end_time** | Option<**String**> | When the command stopped executing (completed, failed, or cancelled), in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\" | [optional]
**stdout** | Option<**String**> | A URL to retrieve standard output logs of the workflow run or task.  This URL may change between status requests, or may not be available until the task or workflow has finished execution.  Should be available using the same credentials used to access the WES endpoint. | [optional]
**stderr** | Option<**String**> | A URL to retrieve standard error logs of the workflow run or task.  This URL may change between status requests, or may not be available until the task or workflow has finished execution.  Should be available using the same credentials used to access the WES endpoint. | [optional]
**exit_code** | Option<**i32**> | Exit code of the program | [optional]
**system_logs** | Option<**Vec<String>**> | System logs are any logs the system decides are relevant, which are not tied directly to a task. Content is implementation specific: format, size, etc.  System logs may be collected here to provide convenient access.  For example, the system may include the name of the host where the task is executing, an error message that caused a SYSTEM_ERROR state (e.g. disk is full), etc. | [optional]
**id** | **String** | A unique identifier which may be used to reference the task | 
**tes_uri** | Option<**String**> | An optional URL pointing to an extended task definition defined by a [TES api](https://github.com/ga4gh/task-execution-schemas) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


