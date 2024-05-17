# \WorkflowRunsApi

All URIs are relative to *https://www.example.com/ga4gh/wes/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_run**](WorkflowRunsApi.md#cancel_run) | **POST** /runs/{run_id}/cancel | CancelRun
[**get_run_log**](WorkflowRunsApi.md#get_run_log) | **GET** /runs/{run_id} | GetRunLog
[**get_run_status**](WorkflowRunsApi.md#get_run_status) | **GET** /runs/{run_id}/status | GetRunStatus
[**get_task**](WorkflowRunsApi.md#get_task) | **GET** /runs/{run_id}/tasks/{task_id} | GetTask
[**list_runs**](WorkflowRunsApi.md#list_runs) | **GET** /runs | ListRuns
[**list_tasks**](WorkflowRunsApi.md#list_tasks) | **GET** /runs/{run_id}/tasks | ListTasks
[**run_workflow**](WorkflowRunsApi.md#run_workflow) | **POST** /runs | RunWorkflow



## cancel_run

> models::RunId cancel_run(run_id)
CancelRun

Cancel a running workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**models::RunId**](RunId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_run_log

> models::RunLog get_run_log(run_id)
GetRunLog

This endpoint provides detailed information about a given workflow run. The returned result has information about the outputs produced by this workflow (if available), a log object which allows the stderr and stdout to be retrieved, a log array so stderr/stdout for individual tasks can be retrieved, and the overall state of the workflow run (e.g. RUNNING, see the State section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**models::RunLog**](RunLog.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_run_status

> models::RunStatus get_run_status(run_id)
GetRunStatus

This provides an abbreviated (and likely fast depending on implementation) status of the running workflow, returning a simple result with the  overall state of the workflow run (e.g. RUNNING, see the State section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |

### Return type

[**models::RunStatus**](RunStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::TaskLog get_task(run_id, task_id)
GetTask

This endpoint provides a mechanism to retrieve information on a specific task, if it exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |

### Return type

[**models::TaskLog**](TaskLog.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_runs

> models::RunListResponse list_runs(page_size, page_token)
ListRuns

This list should be provided in a stable ordering. (The actual ordering is implementation dependent.) When paging through the list, the client should not make assumptions about live updates, but should assume the contents of the list reflect the workflow list at the moment that the first page is requested.  To monitor a specific workflow run, use GetRunStatus or GetRunLog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i64**> | OPTIONAL The preferred number of workflow runs to return in a page. If not provided, the implementation should use a default page size. The implementation must not return more items than `page_size`, but it may return fewer.  Clients should not assume that if fewer than `page_size` items are returned that all items have been returned.  The availability of additional pages is indicated by the value of `next_page_token` in the response. |  |
**page_token** | Option<**String**> | OPTIONAL Token to use to indicate where to start getting results. If unspecified, return the first page of results. |  |

### Return type

[**models::RunListResponse**](RunListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> models::TaskListResponse list_tasks(run_id, page_size, page_token)
ListTasks

This endpoint provides a paginated list of tasks that were executed as part of a given workflow run. Task ordering should be the same as what would be returned in a `RunLog` response body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** |  | [required] |
**page_size** | Option<**i64**> | OPTIONAL The preferred number of task logs to return in a page. If not provided, the implementation should use a default page size. The implementation must not return more items than `page_size`, but it may return fewer.  Clients should not assume that if fewer than `page_size` items are returned that all items have been returned.  The availability of additional pages is indicated by the value of `next_page_token` in the response. |  |
**page_token** | Option<**String**> | OPTIONAL Token to use to indicate where to start getting results. If unspecified, return the first page of results. |  |

### Return type

[**models::TaskListResponse**](TaskListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_workflow

> models::RunId run_workflow(workflow_params, workflow_type, workflow_type_version, tags, workflow_engine, workflow_engine_version, workflow_engine_parameters, workflow_url, workflow_attachment)
RunWorkflow

This endpoint creates a new workflow run and returns a `RunId` to monitor its progress. The `workflow_attachment` array may be used to upload files that are required to execute the workflow, including the primary workflow, tools imported by the workflow, other files referenced by the workflow, or files which are part of the input.  The implementation should stage these files to a temporary directory and execute the workflow from there. These parts must have a Content-Disposition header with a \"filename\" provided for each part.  Filenames may include subdirectories, but must not include references to parent directories with '..' -- implementations should guard against maliciously constructed filenames. The `workflow_url` is either an absolute URL to a workflow file that is accessible by the WES endpoint, or a relative URL corresponding to one of the files attached using `workflow_attachment`. The `workflow_params` JSON object specifies input parameters, such as input files.  The exact format of the JSON object depends on the conventions of the workflow language being used.  Input files should either be absolute URLs, or relative URLs corresponding to files uploaded using `workflow_attachment`.  The WES endpoint must understand and be able to access URLs supplied in the input.  This is implementation specific. The `workflow_type` is the type of workflow language and must be \"CWL\" or \"WDL\" currently (or another alternative  supported by this WES instance). The `workflow_type_version` is the version of the workflow language submitted and must be one supported by this WES instance. The `workflow_engine` is the engine that supports the workflow_type and must be supported by this WES instance. The `workflow_engine_version` is the version of workflow engine and must be supported by this WES instance. See the `RunRequest` documentation for details about other fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_params** | Option<**String**> |  |  |
**workflow_type** | Option<**String**> |  |  |
**workflow_type_version** | Option<**String**> |  |  |
**tags** | Option<**String**> |  |  |
**workflow_engine** | Option<**String**> |  |  |
**workflow_engine_version** | Option<**String**> |  |  |
**workflow_engine_parameters** | Option<**String**> |  |  |
**workflow_url** | Option<**String**> |  |  |
**workflow_attachment** | Option<[**Vec<std::path::PathBuf>**](std::path::PathBuf.md)> |  |  |

### Return type

[**models::RunId**](RunId.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

