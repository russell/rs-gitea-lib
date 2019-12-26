/* 
 * Gitea API.
 *
 * This documentation describes the Gitea API.
 *
 * OpenAPI spec version: 1.1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// StatusState : StatusState holds the state of a Status It can be \"pending\", \"success\", \"error\", \"failure\", and \"warning\"

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusState {
}

impl StatusState {
  /// StatusState holds the state of a Status It can be \"pending\", \"success\", \"error\", \"failure\", and \"warning\"
  pub fn new() -> StatusState {
    StatusState {
    }
  }

}



