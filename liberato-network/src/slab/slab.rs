use std::collections::HashMap;
use std::str::FromStr;

use fixedstr::str128;
use fixedstr::str256;
use fixedstr::str64;
use serde::{Serialize,Deserialize};

type SlabId = str128;
type SlabValue = str256;

/// # SlabValues
/// 
/// HashMap contains:
/// 
/// - [X] HashMap<SlabId,SlabValue> where SlabId is fixedstr128 and SlabValue is fixedstr256
/// 
/// str256 values connected to each other
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct SlabValues256 {
    pub values: HashMap<SlabId,SlabValue>,
}

impl SlabValues256 {
    pub fn new() -> SlabValues256 {
        return SlabValues256 {
            values: HashMap::new(),
        }
    }
    pub fn add<T: AsRef<str>>(&mut self, slab_id: T, slab_value: T) {
        let x = slab_id.as_ref();
        let value = slab_value.as_ref();

        if x.len() < 127 && value.len() < 255 {
            let id: fixedstr::tstr<128> = fixedstr::str128::from_str(x).unwrap();
            let output_value: fixedstr::tstr = fixedstr::str256::from_str(value).unwrap();


            self.values.insert(id, output_value);
        }
    }
}