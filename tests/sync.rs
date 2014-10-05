#![feature(phase)]
#[phase(plugin, link)]
extern crate lazy;
#[phase(plugin)]
extern crate stainless;

pub use lazy::SyncThunk;
pub use std::sync::{Arc, Mutex};

describe! sync {
    it "should evaluate when accessed" {
        let val = sync_lazy!(7i);
        assert_eq!(*val, 7i);
    }

    it "should evaluate just once" {
        let counter = Arc::new(Mutex::new(0i));
        let counter_clone = counter.clone();
        let val = sync_lazy!(*counter.lock() += 1);
        *val;
        *val;
        assert_eq!(*counter_clone.lock(), 1i);
    }

    it "should not evaluate if not accessed" {
        let counter = Arc::new(Mutex::new(0i));
        let counter_clone = counter.clone();
        let _val = sync_lazy!(*counter.lock() += 1);
        assert_eq!(*counter_clone.lock(), 0i);
    }

    it "should be send and sync" {
        Arc::new(sync_lazy!(0u));
    }

    describe! evaluated {
        it "should create an already evaluated thunk" {
            let x = SyncThunk::evaluated(10u);
            assert_eq!(*x, 10u);
        }
    }
}

// sync is all safe code, so no need to test
// destructor calls.
