// rust-analyzer does not show rustdoc for the following attribute proc macro - regardless whether
// imported under the original name, or under a different name:
use allow_prefixed::clippy_await_holding_lock as await_holding_lock_i_know_better;
// rust-analyzer does show rustdoc for the following function alias:
use self::my_fn as my_fn_aliased;

/// Simple function.
fn my_fn() {}
pub fn call_my_fn_alias() {
    // rust-analyzer does show rustdoc for the following (aliased) call:
    my_fn_aliased();
}

// rust-analyzer does not show rustdoc for the following invocation of an attribute proc macro -
// regardless whether under a fully specified original name, or imported under an alias:
#[await_holding_lock_i_know_better]
#[allow_prefixed::clippy_await_holding_lock]
pub fn invoke_attrib_macro_aliased() {}
