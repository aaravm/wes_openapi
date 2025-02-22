/*
 * Workflow Execution Service
 *
 * *Run standard workflows on workflow execution platforms in a platform-agnostic way.* ## Executive Summary The Workflow Execution Service (WES) API provides a standard way for users to submit workflow requests to workflow execution systems, and to monitor their execution. This API lets users run a single workflow (currently [**CWL**](https://www.commonwl.org/) or [**WDL**](http://www.openwdl.org/) formatted workflows, other types may be supported in the future) on multiple different platforms, clouds, and environments. Key features of the API: + can request that a workflow be run + can pass parameters to that workflow (e.g. input files, cmdline arguments) + can get information about running workflows (e.g. status, errors, output file locations) + can cancel a running workflow 
 *
 * The version of the OpenAPI document: 1.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// DefaultWorkflowEngineParameter : A message that allows one to describe default parameters for a workflow engine.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultWorkflowEngineParameter {
    /// The name of the parameter
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Describes the type of the parameter, e.g. float.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The stringified version of the default parameter. e.g. \"2.45\".
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

impl DefaultWorkflowEngineParameter {
    /// A message that allows one to describe default parameters for a workflow engine.
    pub fn new() -> DefaultWorkflowEngineParameter {
        DefaultWorkflowEngineParameter {
            name: None,
            r#type: None,
            default_value: None,
        }
    }
}

