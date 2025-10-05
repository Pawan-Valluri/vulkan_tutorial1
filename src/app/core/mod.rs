use thiserror::Error;
use std::ffi::CStr;
use std::os::raw::c_void;

use log::*;

use vulkanalia::vk::{
    DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCallbackDataEXT,
    Bool32, FALSE
};
#[derive(Debug, Error)]
#[error("Missing {0}.")]
pub struct SuitabilityError(pub &'static str);


pub extern "system" fn debug_callback(
    severity: DebugUtilsMessageSeverityFlagsEXT,
    type_: DebugUtilsMessageTypeFlagsEXT,
    data: *const DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> Bool32 {
    let data = unsafe { *data };
    let message = unsafe { CStr::from_ptr(data.message) }.to_string_lossy();

    if severity >= DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("({:?}) {}", type_, message);
    } else if severity >= DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("({:?}) {}", type_, message);
    } else if severity >= DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("({:?}) {}", type_, message);
    } else {
        trace!("({:?}) {}", type_, message);
    }

    FALSE
}
