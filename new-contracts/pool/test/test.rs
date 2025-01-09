use application_builder::{
    traits::{VftManager},
    VftManager as Pool
}

use sails_rs::calls::*;
use sails_rs::U256;
use sails_rs::gtest::{calls::*, System};
use sails_rs::ActorId;
use serde_json::json;
use std::fs;
