/*!
Reference <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/flow-schema-v1beta2/>
*/

use kfl::Decode;

use crate::meta::{
    condition::Condition,
    metadata::Metadata
};

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/flow-schema-v1beta2/#FlowSchema>
#[derive(Debug, Decode)]
pub struct FlowSchema {
    metadata: Metadata,
    spec: FlowSchemaSpec,
    status: Option<FlowSchemaStatus>
}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/flow-schema-v1beta2/#FlowSchemaSpec>
#[derive(Debug, Decode)]
pub struct FlowSchemaSpec {
    priority_level_configuration: PriorityLevelConfigurationReference,
    distinguisher_method: Option<FlowDistinguisherMethod>,
    matching_precedence: Option<i32>,
    rules: Vec<PolicyRulesWithSubjects>
}

#[derive(Debug, Decode)]
pub struct PriorityLevelConfigurationReference {
    name: String
}

#[derive(Debug, Decode)]
pub struct FlowDistinguisherMethod {
    r#type: FlowDistinguisherMethodType
}

#[derive(Debug, DecodeScalar)]
pub enum FlowDistinguisherMethodType {
    ByUser,
    ByNamespace
}

#[derive(Debug, Decode)]
pub struct PolicyRulesWithSubjects {
    subjects: Vec<Subject>,
    non_resource_rules: Vec<NonResourcePolicyRule>,
    resource_rules: Vec<ResourcePolicyRule>
}

#[derive(Debug, Decode)]
pub struct Subject {
    kind: String,
    group: Option<GroupSubject>,
    service_account: Option<ServiceAccountSubject>,
    user: Option<UserSubject>
}

#[derive(Debug, Decode)]
pub struct GroupSubject {
    name: String
}

#[derive(Debug, Decode)]
pub struct ServiceAccountSubject {
    name: String,
    namespace: String
}

#[derive(Debug, Decode)]
pub struct UserSubject {
    name: String
}

#[derive(Debug, Decode)]
pub struct NonResourcePolicyRule {

}

#[derive(Debug, Decode)]
pub struct ResourcePolicyRule {

}

/// <https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/flow-schema-v1beta2/#FlowSchemaStatus>
#[derive(Debug, Decode)]
pub struct FlowSchemaStatus {
    conditions: Vec<Condition>,
}
