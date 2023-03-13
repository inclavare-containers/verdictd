use crate::resources::opa;
use std::ffi::CStr;
use std::os::raw::c_char;

// Link import cgo function
#[link(name = "opa")]
extern "C" {
    pub fn makeDecisionGo(policy: GoString, data: GoString, input: GoString) -> *mut c_char;
}

/// String structure passed into cgo
#[derive(Debug)]
#[repr(C)]
pub struct GoString {
    pub p: *const c_char,
    pub n: isize,
}

// According to message and policy, the decision is made by opa
pub fn make_decision(policy_name: &str, data_name: &str, input: &str) -> Result<String, String> {
    // Get the content of policy from policy_name
    let policy = opa::export(policy_name)?;

    let policy_go = GoString {
        p: policy.as_ptr() as *const i8,
        n: policy.len() as isize,
    };

    let data = opa::export(data_name)?;
    let data_go = GoString {
        p: data.as_ptr() as *const i8,
        n: data.len() as isize,
    };

    let input_go = GoString {
        p: input.as_ptr() as *const i8,
        n: input.len() as isize,
    };

    // Call the function exported by cgo and process the returned decision
    let decision_buf: *mut c_char = unsafe { makeDecisionGo(policy_go, data_go, input_go) };
    let decision_str: &CStr = unsafe { CStr::from_ptr(decision_buf) };
    decision_str
        .to_str()
        .map_err(|e| e.to_string())
        .and_then(|str| Ok(str.to_string()))
}

pub fn make_decision_ext(
    policy_name: &str,
    policy_content: &str,
    policy_remote: bool,
    reference_name: &str,
    reference_content: &str,
    reference_remote: bool,
    input: &str,
) -> Result<String, String> {
    let policy = if policy_remote == true {
        policy_content.to_owned()
    } else {
        opa::export(policy_name).unwrap()
    };

    let reference = if reference_remote == true {
        reference_content.to_owned()
    } else {
        opa::export(reference_name).unwrap()
    };

    let policy_go = GoString {
        p: policy.as_str().as_ptr() as *const i8,
        n: policy.as_str().len() as isize,
    };

    let reference_go = GoString {
        p: reference.as_str().as_ptr() as *const i8,
        n: reference.as_str().len() as isize,
    };

    let input_go = GoString {
        p: input.as_ptr() as *const i8,
        n: input.len() as isize,
    };

    // Call the function exported by cgo and process the returned decision
    let decision_buf: *mut c_char = unsafe { makeDecisionGo(policy_go, reference_go, input_go) };
    let decision_str: &CStr = unsafe { CStr::from_ptr(decision_buf) };
    decision_str
        .to_str()
        .map_err(|e| e.to_string())
        .and_then(|str| Ok(str.to_string()))
}
