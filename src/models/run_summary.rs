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

/// RunSummary : Small description of a workflow run
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunSummary {
    #[serde(rename = "run_id")]
    pub run_id: String,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::State>,
    /// When the run started executing, in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\"
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// When the run stopped executing (completed, failed, or cancelled), in ISO 8601 format \"%Y-%m-%dT%H:%M:%SZ\"
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Arbitrary key/value tags added by the client during run creation
    #[serde(rename = "tags")]
    pub tags: std::collections::HashMap<String, String>,
}

impl RunSummary {
    /// Small description of a workflow run
    pub fn new(run_id: String, tags: std::collections::HashMap<String, String>) -> RunSummary {
        RunSummary {
            run_id,
            state: None,
            start_time: None,
            end_time: None,
            tags,
        }
    }
}

