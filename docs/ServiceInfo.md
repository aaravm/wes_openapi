# ServiceInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique ID of this service. Reverse domain name notation is recommended, though not required. The identifier should attempt to be globally unique so it can be used in downstream aggregator services e.g. Service Registry. | 
**name** | **String** | Name of this service. Should be human readable. | 
**r#type** | [**models::ServiceType**](ServiceType.md) |  | 
**description** | Option<**String**> | Description of the service. Should be human readable and provide information about the service. | [optional]
**organization** | [**models::ServiceOrganization**](Service_organization.md) |  | 
**contact_url** | Option<**String**> | URL of the contact for the provider of this service, e.g. a link to a contact form (RFC 3986 format), or an email (RFC 2368 format). | [optional]
**documentation_url** | Option<**String**> | URL of the documentation of this service (RFC 3986 format). This should help someone learn how to use your service, including any specifics required to access data, e.g. authentication. | [optional]
**created_at** | Option<**String**> | Timestamp describing when the service was first deployed and available (RFC 3339 format) | [optional]
**updated_at** | Option<**String**> | Timestamp describing when the service was last updated (RFC 3339 format) | [optional]
**environment** | Option<**String**> | Environment the service is running in. Use this to distinguish between production, development and testing/staging deployments. Suggested values are prod, test, dev, staging. However this is advised and not enforced. | [optional]
**version** | **String** | Version of the service being described. Semantic versioning is recommended, but other identifiers, such as dates or commit hashes, are also allowed. The version should be changed whenever the service is updated. | 
**workflow_type_versions** | [**std::collections::HashMap<String, models::WorkflowTypeVersion>**](WorkflowTypeVersion.md) |  | 
**supported_wes_versions** | **Vec<String>** | The version(s) of the WES schema supported by this service | 
**supported_filesystem_protocols** | **Vec<String>** | The filesystem protocols supported by this service, currently these may include common protocols using the terms 'http', 'https', 'sftp', 's3', 'gs', 'file', or 'synapse', but others  are possible and the terms beyond these core protocols are currently not fixed.   This section reports those protocols (either common or not) supported by this WES service. | 
**workflow_engine_versions** | [**std::collections::HashMap<String, models::WorkflowEngineVersion>**](WorkflowEngineVersion.md) |  | 
**default_workflow_engine_parameters** | [**Vec<models::DefaultWorkflowEngineParameter>**](DefaultWorkflowEngineParameter.md) | Each workflow engine can present additional parameters that can be sent to the workflow engine. This message will list the default values, and their types for each workflow engine. | 
**system_state_counts** | **std::collections::HashMap<String, i64>** |  | 
**auth_instructions_url** | **String** | A web page URL with human-readable instructions on how to get an authorization token for use with a specific WES endpoint. | 
**tags** | **std::collections::HashMap<String, String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


