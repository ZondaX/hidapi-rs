// **************************************************************************
// Copyright (c) 2018 Roland Ruckerbauer All Rights Reserved.
//
// This file is part of hidapi-rs, based on hidapi-rs by Osspial
// **************************************************************************

use super::HidDeviceInfo;
use failure::{Compat, Error};
use libc::wchar_t;

#[derive(Debug, Fail)]
pub enum HidError {
    #[fail(display = "hidapi error: {}", message)]
    HidApiError { message: String },
    #[fail(
        display = "hidapi error: (could not get error message), caused by: {}",
        cause
    )]
    HidApiErrorEmptyWithCause {
        #[cause]
        cause: Compat<Error>,
    },
    #[fail(display = "hidapi error: (could not get error message)")]
    HidApiErrorEmpty,
    #[fail(display = "failed converting {:#X} to rust char", wide_char)]
    FromWideCharError { wide_char: wchar_t },
    #[fail(display = "Failed to initialize hidapi (maybe initialized before?)")]
    InitializationError,
    #[fail(display = "Failed opening hid device")]
    OpenHidDeviceError,
    #[fail(display = "Invalid data: size can not be 0")]
    InvalidZeroSizeData,
    #[fail(
        display = "Failed to send all data: only sent {} out of {} bytes",
        sent,
        all
    )]
    IncompleteSendError { sent: usize, all: usize },
    #[fail(display = "Can not set blocking mode to '{}'", mode)]
    SetBlockingModeError { mode: &'static str },
    #[fail(display = "Can not open hid device with: {:?}", device_info)]
    OpenHidDeviceWithDeviceInfoError { device_info: HidDeviceInfo },
}
