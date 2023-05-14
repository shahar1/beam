use std::collections::HashMap;

use std::boxed::Box;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

use crate::elem_types::kv::KV;
use crate::elem_types::ElemType;
use crate::transforms::group_by_key::KeyExtractor;
use crate::transforms::pardo::DoFn;
use crate::worker::operators::{DynamicGroupedValues, DynamicWindowedValue, WindowedValue};
use crate::worker::Receiver;

static DO_FNS: Lazy<Mutex<HashMap<String, &'static dyn DynamicDoFn>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static KEY_EXTRACTORS: Lazy<Mutex<HashMap<String, &'static dyn DynamicKeyExtractor>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn store_do_fn(do_fn: impl DoFn + 'static) -> String {
    let mut do_fns = DO_FNS.lock().unwrap();
    let name = format!("object{}", do_fns.len());
    do_fns.insert(name.to_string(), Box::leak(Box::new(do_fn)));
    name
}

pub fn get_do_fn(name: &str) -> Option<&'static dyn DynamicDoFn> {
    let binding = DO_FNS.lock().unwrap();
    binding.get(name).copied()
}

pub fn store_key_extractor(ke: impl DynamicKeyExtractor + 'static) -> String {
    let mut kes = KEY_EXTRACTORS.lock().unwrap();
    let name = format!("object{}", kes.len());
    kes.insert(name.to_string(), Box::leak(Box::new(ke)));
    name
}

pub fn get_extractor(name: &str) -> Option<&'static dyn DynamicKeyExtractor> {
    KEY_EXTRACTORS.lock().unwrap().get(name).copied()
}

pub trait DynamicDoFn: Send + Sync {
    fn process_dyn(&self, elem: DynamicWindowedValue, receivers: &[Arc<Receiver>]);
    fn start_bundle_dyn(&self);
    fn finish_bundle_dyn(&self);
}

impl<D: DoFn> DynamicDoFn for D {
    fn process_dyn(&self, elem: DynamicWindowedValue, receivers: &[Arc<Receiver>]) {
        let typed_elem = elem.downcast_ref::<D::In>();
        for value in self.process(&typed_elem.value) {
            let windowed_value = typed_elem.with_value(value);
            for receiver in receivers {
                receiver.receive(DynamicWindowedValue::new(&windowed_value))
            }
        }
    }

    fn start_bundle_dyn(&self) {
        self.start_bundle()
    }

    fn finish_bundle_dyn(&self) {
        self.finish_bundle()
    }
}

pub trait DynamicKeyExtractor: Sync + Send {
    fn new_grouped_values(&self) -> DynamicGroupedValues;
    fn clear_grouped_values(&self, grouped_values: &mut DynamicGroupedValues);
    fn extract(&self, kv: DynamicWindowedValue, grouped_values: &mut DynamicGroupedValues);
    fn recombine(&self, grouped_values: &DynamicGroupedValues, receivers: &[Arc<Receiver>]);
}

impl<V: ElemType + Clone> DynamicKeyExtractor for KeyExtractor<V> {
    fn new_grouped_values(&self) -> DynamicGroupedValues {
        DynamicGroupedValues::new::<V>()
    }
    fn clear_grouped_values(&self, grouped_values: &mut DynamicGroupedValues) {
        grouped_values.downcast_mut::<V>().clear()
    }
    fn extract(&self, kv: DynamicWindowedValue, grouped_values: &mut DynamicGroupedValues) {
        let KV { k, v } = &kv.downcast_ref::<KV<String, V>>().value;
        let grouped_values = grouped_values.downcast_mut::<V>();

        if !grouped_values.contains_key(k) {
            grouped_values.insert(k.clone(), Vec::new());
        }
        grouped_values.get_mut(k).unwrap().push(v.clone());
    }

    fn recombine(&self, grouped_values: &DynamicGroupedValues, receivers: &[Arc<Receiver>]) {
        let typed_grouped_values = grouped_values.downcast_ref::<V>();
        for (key, values) in typed_grouped_values.iter() {
            // TODO: timestamp and pane info are wrong
            for receiver in receivers.iter() {
                // TODO: End-of-window timestamp, only firing pane.
                let mut typed_values: Vec<V> = Vec::new();
                for value in values.iter() {
                    typed_values.push(value.clone());
                }
                let res = KV {
                    k: key.to_string(),
                    v: typed_values,
                };
                receiver.receive(DynamicWindowedValue::new(&WindowedValue::in_global_window(
                    res,
                )));
            }
        }
    }
}
