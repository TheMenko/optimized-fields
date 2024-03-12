#![doc = include_str!("../README.md")]
#![doc(issue_tracker_base_url = "https://github.com/andrewmilson/optimized-fields/issues/")]
#![warn(clippy::all, clippy::pedantic, clippy::cargo, clippy::nursery)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod fp31;
pub mod fp64;

pub(crate) mod macros;
