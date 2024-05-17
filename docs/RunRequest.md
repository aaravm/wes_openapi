# RunRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflow_params** | Option<[**serde_json::Value**](.md)> | REQUIRED The workflow run parameterizations (JSON encoded), including input and output file locations | [optional]
**workflow_type** | **String** | REQUIRED The workflow descriptor type, must be \"CWL\" or \"WDL\" currently (or another alternative supported by this WES instance) | 
**workflow_type_version** | **String** | REQUIRED The workflow descriptor type version, must be one supported by this WES instance | 
**tags** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**workflow_engine_parameters** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**workflow_engine** | Option<**String**> | The workflow engine, must be one supported by this WES instance. Required if workflow_engine_version is provided. | [optional]
**workflow_engine_version** | Option<**String**> | The workflow engine version, must be one supported by this WES instance. If workflow_engine is provided, but workflow_engine_version is not, servers can make no assumptions with regard to the engine version the WES instance uses to process the request if  that WES instance supports multiple versions of the requested engine. | [optional]
**workflow_url** | **String** | REQUIRED The workflow CWL or WDL document. When `workflow_attachments` is used to attach files, the `workflow_url` may be a relative path to one of the attachments. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


