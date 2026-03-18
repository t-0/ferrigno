#![allow(unpredictable_function_pointer_comparisons)]
use crate::registeredfunction::*;
use crate::state::*;
use crate::tagtype::*;

// ── sequence generation ─────────────────────────────────────────────────────

/// itertools.range([start,] stop [, step]) → table
unsafe fn it_range(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        let (start, stop, step) = match nargs {
            | 1 => (1, lual_checkinteger(state, 1), 1i64),
            | 2 => (lual_checkinteger(state, 1), lual_checkinteger(state, 2), 1),
            | _ => (lual_checkinteger(state, 1), lual_checkinteger(state, 2), lual_checkinteger(state, 3)),
        };
        if step == 0 { return lual_error(state, c"step cannot be zero".as_ptr(), &[]); }
        (*state).lua_createtable();
        let mut j: i64 = 1;
        let mut i = start;
        while (step > 0 && i <= stop) || (step < 0 && i >= stop) {
            (*state).push_integer(i);
            lua_rawseti(state, -2, j);
            j += 1;
            i += step;
        }
        1
    }
}

/// itertools.rep(val, n) → table
unsafe fn it_rep(state: *mut State) -> i32 {
    unsafe {
        lual_checkany(state, 1);
        let n = lual_checkinteger(state, 2);
        (*state).lua_createtable();
        for i in 1..=n {
            lua_pushvalue(state, 1);
            lua_rawseti(state, -2, i);
        }
        1
    }
}

/// itertools.cycle(t, n) → table repeating t n times
unsafe fn it_cycle(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let n = lual_checkinteger(state, 2);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for _ in 0..n {
            for i in 1..=len {
                lua_rawgeti(state, 1, i);
                lua_rawseti(state, -2, j);
                j += 1;
            }
        }
        1
    }
}

// ── slicing / selection ─────────────────────────────────────────────────────

/// itertools.slice(t, start [, stop [, step]]) → table
unsafe fn it_slice(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        let nargs = (*state).get_top();
        let start = lual_checkinteger(state, 2).max(1);
        let stop = if nargs >= 3 { lual_checkinteger(state, 3).min(len) } else { len };
        let step = if nargs >= 4 { lual_checkinteger(state, 4) } else { 1 };
        if step == 0 { return lual_error(state, c"step cannot be zero".as_ptr(), &[]); }
        (*state).lua_createtable();
        let mut j: i64 = 1;
        let mut i = start;
        while (step > 0 && i <= stop) || (step < 0 && i >= stop) {
            lua_rawgeti(state, 1, i);
            lua_rawseti(state, -2, j);
            j += 1;
            i += step;
        }
        1
    }
}

/// itertools.takewhile(f, t) → table
unsafe fn it_takewhile(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for i in 1..=len {
            lua_pushvalue(state, 1);
            lua_rawgeti(state, 2, i);
            (*state).lua_callk(1, 1, 0, None);
            if !lua_toboolean(state, -1) { lua_settop(state, -2); break; }
            lua_settop(state, -2);
            lua_rawgeti(state, 2, i);
            lua_rawseti(state, -2, j);
            j += 1;
        }
        1
    }
}

/// itertools.dropwhile(f, t) → table
unsafe fn it_dropwhile(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        (*state).lua_createtable();
        let result_idx = (*state).get_top();
        let mut dropping = true;
        let mut j: i64 = 1;
        for i in 1..=len {
            if dropping {
                lua_pushvalue(state, 1);
                lua_rawgeti(state, 2, i);
                (*state).lua_callk(1, 1, 0, None);
                if lua_toboolean(state, -1) { lua_settop(state, -2); continue; }
                lua_settop(state, -2);
                dropping = false;
            }
            lua_rawgeti(state, 2, i);
            lua_rawseti(state, result_idx, j);
            j += 1;
        }
        lua_pushvalue(state, result_idx);
        1
    }
}

/// itertools.compress(t, selectors) → table
unsafe fn it_compress(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        (*state).lual_checktype(2, TagType::Table);
        let len1 = get_length_raw(state, 1) as i64;
        let len2 = get_length_raw(state, 2) as i64;
        let len = len1.min(len2);
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for i in 1..=len {
            lua_rawgeti(state, 2, i);
            let keep = lua_toboolean(state, -1);
            lua_settop(state, -2);
            if keep {
                lua_rawgeti(state, 1, i);
                lua_rawseti(state, -2, j);
                j += 1;
            }
        }
        1
    }
}

// ── combining ───────────────────────────────────────────────────────────────

/// itertools.chain(t1, t2, ...) → concatenated table
unsafe fn it_chain(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for arg in 1..=nargs {
            (*state).lual_checktype(arg, TagType::Table);
            let len = get_length_raw(state, arg) as i64;
            for i in 1..=len {
                lua_rawgeti(state, arg, i);
                lua_rawseti(state, -2, j);
                j += 1;
            }
        }
        1
    }
}

/// itertools.zip(t1, t2, ...) → table of tables
unsafe fn it_zip(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        if nargs == 0 { (*state).lua_createtable(); return 1; }
        // find minimum length
        let mut min_len = i64::MAX;
        for arg in 1..=nargs {
            (*state).lual_checktype(arg, TagType::Table);
            let len = get_length_raw(state, arg) as i64;
            if len < min_len { min_len = len; }
        }
        (*state).lua_createtable();
        for i in 1..=min_len {
            (*state).lua_createtable(); // inner tuple
            for arg in 1..=nargs {
                lua_rawgeti(state, arg, i);
                lua_rawseti(state, -2, arg as i64);
            }
            lua_rawseti(state, -2, i);
        }
        1
    }
}

/// itertools.zip_longest(fill, t1, t2, ...) → table of tables
unsafe fn it_zip_longest(state: *mut State) -> i32 {
    unsafe {
        let nargs = (*state).get_top();
        if nargs < 2 { (*state).lua_createtable(); return 1; }
        // arg 1 is fill value, args 2..n are tables
        let mut max_len: i64 = 0;
        for arg in 2..=nargs {
            (*state).lual_checktype(arg, TagType::Table);
            let len = get_length_raw(state, arg) as i64;
            if len > max_len { max_len = len; }
        }
        (*state).lua_createtable();
        for i in 1..=max_len {
            (*state).lua_createtable();
            for arg in 2..=nargs {
                let len = get_length_raw(state, arg) as i64;
                if i <= len {
                    lua_rawgeti(state, arg, i);
                } else {
                    lua_pushvalue(state, 1); // fill value
                }
                lua_rawseti(state, -2, (arg - 1) as i64);
            }
            lua_rawseti(state, -2, i);
        }
        1
    }
}

/// itertools.enumerate(t [, start]) → table of {index, value} tables
unsafe fn it_enumerate(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let start = lual_optinteger(state, 2, 1);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        for i in 1..=len {
            (*state).lua_createtable();
            (*state).push_integer(start + i - 1);
            lua_rawseti(state, -2, 1);
            lua_rawgeti(state, 1, i);
            lua_rawseti(state, -2, 2);
            lua_rawseti(state, -2, i);
        }
        1
    }
}

// ── transformation ──────────────────────────────────────────────────────────

/// itertools.accumulate(t, f [, init]) → table of running results
unsafe fn it_accumulate(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        (*state).lual_checktype(2, TagType::Closure);
        let has_init = (*state).get_top() >= 3;
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        let result_idx = (*state).get_top();
        let start: i64;
        if has_init {
            lua_pushvalue(state, 3); // accumulator
            start = 1;
        } else {
            if len == 0 { lua_pushvalue(state, result_idx); return 1; }
            lua_rawgeti(state, 1, 1); // first element as accumulator
            // save first element in result
            lua_pushvalue(state, -1);
            lua_rawseti(state, result_idx, 1);
            start = 2;
        }
        // accumulator is at top of stack
        if has_init {
            // save init as first result
            lua_pushvalue(state, -1);
            lua_rawseti(state, result_idx, 1);
        }
        let mut out_idx: i64 = if has_init { 1 } else { 1 };
        for i in start..=len {
            lua_pushvalue(state, 2); // push f
            lua_pushvalue(state, -2); // push acc copy
            lua_rawgeti(state, 1, i); // push t[i]
            (*state).lua_callk(2, 1, 0, None); // f(acc, t[i]) → new_acc
            lua_rotate(state, -2, 1); // swap: old_acc new_acc
            lua_settop(state, -2); // pop old acc
            out_idx += 1;
            lua_pushvalue(state, -1); // dup new acc
            lua_rawseti(state, result_idx, out_idx);
        }
        lua_settop(state, -2); // pop accumulator
        lua_pushvalue(state, result_idx);
        1
    }
}

/// itertools.pairwise(t) → table of {t[i], t[i+1]} tables
unsafe fn it_pairwise(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        for i in 1..len {
            (*state).lua_createtable();
            lua_rawgeti(state, 1, i);
            lua_rawseti(state, -2, 1);
            lua_rawgeti(state, 1, i + 1);
            lua_rawseti(state, -2, 2);
            lua_rawseti(state, -2, i);
        }
        1
    }
}

/// itertools.flatten(t) → flattened one level
unsafe fn it_flatten(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for i in 1..=len {
            lua_rawgeti(state, 1, i);
            if lua_type(state, -1) == Some(TagType::Table) {
                let inner_len = get_length_raw(state, -1) as i64;
                for k in 1..=inner_len {
                    lua_rawgeti(state, -1, k);
                    lua_rawseti(state, -3, j);
                    j += 1;
                }
                lua_settop(state, -2); // pop inner table
            } else {
                lua_rawseti(state, -2, j);
                j += 1;
            }
        }
        1
    }
}

/// itertools.batched(t, n) → table of chunks
unsafe fn it_batched(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let n = lual_checkinteger(state, 2);
        if n <= 0 { return lual_error(state, c"batch size must be positive".as_ptr(), &[]); }
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        let mut batch_num: i64 = 1;
        let mut i: i64 = 1;
        while i <= len {
            (*state).lua_createtable();
            let mut k: i64 = 1;
            while k <= n && i <= len {
                lua_rawgeti(state, 1, i);
                lua_rawseti(state, -2, k);
                k += 1;
                i += 1;
            }
            lua_rawseti(state, -2, batch_num);
            batch_num += 1;
        }
        1
    }
}

/// itertools.reversed(t) → reversed table
unsafe fn it_reversed(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        for i in 1..=len {
            lua_rawgeti(state, 1, len - i + 1);
            lua_rawseti(state, -2, i);
        }
        1
    }
}

/// itertools.starmap(f, t) → table (unpacks each element as args)
unsafe fn it_starmap(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Closure);
        (*state).lual_checktype(2, TagType::Table);
        let len = get_length_raw(state, 2) as i64;
        (*state).lua_createtable();
        let result_idx = (*state).get_top();
        for i in 1..=len {
            lua_pushvalue(state, 1); // push f
            lua_rawgeti(state, 2, i); // push t[i] (should be a table)
            if lua_type(state, -1) == Some(TagType::Table) {
                let inner_len = get_length_raw(state, -1) as i32;
                let table_pos = (*state).get_top();
                for k in 1..=inner_len as i64 {
                    lua_rawgeti(state, table_pos, k);
                }
                lua_rotate(state, table_pos, -1); // remove the inner table from stack
                lua_settop(state, -(1) - 1);
                (*state).lua_callk(inner_len, 1, 0, None);
            } else {
                // not a table, call f(element)
                (*state).lua_callk(1, 1, 0, None);
            }
            lua_rawseti(state, result_idx, i);
        }
        lua_pushvalue(state, result_idx);
        1
    }
}

// ── combinatoric ────────────────────────────────────────────────────────────

/// itertools.product(t1, t2) → table of {a, b} pairs
unsafe fn it_product(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        (*state).lual_checktype(2, TagType::Table);
        let len1 = get_length_raw(state, 1) as i64;
        let len2 = get_length_raw(state, 2) as i64;
        (*state).lua_createtable();
        let mut j: i64 = 1;
        for a in 1..=len1 {
            for b in 1..=len2 {
                (*state).lua_createtable();
                lua_rawgeti(state, 1, a);
                lua_rawseti(state, -2, 1);
                lua_rawgeti(state, 2, b);
                lua_rawseti(state, -2, 2);
                lua_rawseti(state, -2, j);
                j += 1;
            }
        }
        1
    }
}

/// itertools.combinations(t, r) → table of r-length subsequences
unsafe fn it_combinations(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let r = lual_checkinteger(state, 2) as usize;
        let n = get_length_raw(state, 1);
        (*state).lua_createtable();
        if r > n { return 1; }
        let result_idx = (*state).get_top();
        let mut indices: Vec<usize> = (0..r).collect();
        let mut out: i64 = 1;
        loop {
            // emit current combination
            (*state).lua_createtable();
            for (k, &idx) in indices.iter().enumerate() {
                lua_rawgeti(state, 1, (idx + 1) as i64);
                lua_rawseti(state, -2, (k + 1) as i64);
            }
            lua_rawseti(state, result_idx, out);
            out += 1;
            // advance to next combination
            let mut i = r;
            loop {
                if i == 0 { lua_pushvalue(state, result_idx); return 1; }
                i -= 1;
                indices[i] += 1;
                if indices[i] <= n - r + i { break; }
            }
            for j in (i + 1)..r {
                indices[j] = indices[j - 1] + 1;
            }
        }
    }
}

/// itertools.permutations(t [, r]) → table of r-length permutations
unsafe fn it_permutations(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let n = get_length_raw(state, 1);
        let r = lual_optinteger(state, 2, n as i64) as usize;
        (*state).lua_createtable();
        if r > n { return 1; }
        let result_idx = (*state).get_top();
        let mut indices: Vec<usize> = (0..n).collect();
        let mut cycles: Vec<usize> = (0..r).map(|i| n - i).collect();
        let mut out: i64 = 1;
        // emit first permutation
        (*state).lua_createtable();
        for k in 0..r {
            lua_rawgeti(state, 1, (indices[k] + 1) as i64);
            lua_rawseti(state, -2, (k + 1) as i64);
        }
        lua_rawseti(state, result_idx, out);
        out += 1;
        loop {
            let mut found = false;
            for i in (0..r).rev() {
                cycles[i] -= 1;
                if cycles[i] == 0 {
                    let tmp = indices[i];
                    for j in i..n - 1 { indices[j] = indices[j + 1]; }
                    indices[n - 1] = tmp;
                    cycles[i] = n - i;
                } else {
                    let j = n - cycles[i];
                    indices.swap(i, j);
                    (*state).lua_createtable();
                    for k in 0..r {
                        lua_rawgeti(state, 1, (indices[k] + 1) as i64);
                        lua_rawseti(state, -2, (k + 1) as i64);
                    }
                    lua_rawseti(state, result_idx, out);
                    out += 1;
                    found = true;
                    break;
                }
            }
            if !found { break; }
        }
        lua_pushvalue(state, result_idx);
        1
    }
}

/// itertools.groupby(t, f) → table of {key, group} tables
unsafe fn it_groupby(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        (*state).lual_checktype(2, TagType::Closure);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable();
        let result_idx = (*state).get_top();
        if len == 0 { return 1; }
        let mut group_num: i64 = 0;
        let mut group_elem: i64 = 0;
        // prev_key slot: we'll use stack position result_idx + 1
        (*state).push_nil(); // placeholder for prev_key
        let prev_key_idx = (*state).get_top();
        for i in 1..=len {
            // compute key for element i
            lua_pushvalue(state, 2);
            lua_rawgeti(state, 1, i);
            (*state).lua_callk(1, 1, 0, None);
            // stack top = current_key
            let new_group = if group_num == 0 {
                true
            } else {
                !lua_compare(state, prev_key_idx, -1, 0)
            };
            if new_group {
                // update prev_key
                lua_pushvalue(state, -1);
                lua_copy(state, -1, prev_key_idx);
                lua_settop(state, -2);
                // start new group
                group_num += 1;
                group_elem = 0;
                (*state).lua_createtable(); // pair {key, elements}
                lua_pushvalue(state, -2); // push current_key
                lua_rawseti(state, -2, 1); // pair[1] = key
                (*state).lua_createtable(); // elements
                lua_rawgeti(state, 1, i);
                group_elem += 1;
                lua_rawseti(state, -2, group_elem);
                lua_rawseti(state, -2, 2); // pair[2] = elements
                lua_rawseti(state, result_idx, group_num);
            } else {
                // add to current group
                lua_rawgeti(state, result_idx, group_num);
                lua_rawgeti(state, -1, 2);
                group_elem += 1;
                lua_rawgeti(state, 1, i);
                lua_rawseti(state, -2, group_elem);
                lua_settop(state, -3); // pop elements + pair
            }
            lua_settop(state, prev_key_idx); // pop current_key, keep prev_key
        }
        lua_settop(state, result_idx); // clean up prev_key
        1
    }
}

// ── set operations ──────────────────────────────────────────────────────────

/// itertools.unique(t) → table with duplicates removed (preserves order)
unsafe fn it_unique(state: *mut State) -> i32 {
    unsafe {
        (*state).lual_checktype(1, TagType::Table);
        let len = get_length_raw(state, 1) as i64;
        (*state).lua_createtable(); // result
        (*state).lua_createtable(); // seen set
        let result_idx = (*state).get_top() - 1;
        let seen_idx = (*state).get_top();
        let mut j: i64 = 1;
        for i in 1..=len {
            lua_rawgeti(state, 1, i);
            lua_pushvalue(state, -1); // dup for seen check
            if lua_rawget(state, seen_idx) == TagType::Nil {
                lua_settop(state, -2); // pop nil
                // mark as seen
                lua_pushvalue(state, -1); // dup value as key
                (*state).push_boolean(true);
                lua_rawset(state, seen_idx);
                // add to result
                lua_rawseti(state, result_idx, j);
                j += 1;
            } else {
                lua_settop(state, -3); // pop rawget result + value
            }
        }
        lua_pushvalue(state, result_idx);
        1
    }
}

const ITERTOOLS_FUNCTIONS: [RegisteredFunction; 22] = [
    RegisteredFunction { registeredfunction_name: c"range".as_ptr(), registeredfunction_function: Some(it_range as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"rep".as_ptr(), registeredfunction_function: Some(it_rep as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"cycle".as_ptr(), registeredfunction_function: Some(it_cycle as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"slice".as_ptr(), registeredfunction_function: Some(it_slice as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"takewhile".as_ptr(), registeredfunction_function: Some(it_takewhile as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"dropwhile".as_ptr(), registeredfunction_function: Some(it_dropwhile as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"compress".as_ptr(), registeredfunction_function: Some(it_compress as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"chain".as_ptr(), registeredfunction_function: Some(it_chain as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"zip".as_ptr(), registeredfunction_function: Some(it_zip as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"zip_longest".as_ptr(), registeredfunction_function: Some(it_zip_longest as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"enumerate".as_ptr(), registeredfunction_function: Some(it_enumerate as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"accumulate".as_ptr(), registeredfunction_function: Some(it_accumulate as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"pairwise".as_ptr(), registeredfunction_function: Some(it_pairwise as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"flatten".as_ptr(), registeredfunction_function: Some(it_flatten as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"batched".as_ptr(), registeredfunction_function: Some(it_batched as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"reversed".as_ptr(), registeredfunction_function: Some(it_reversed as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"starmap".as_ptr(), registeredfunction_function: Some(it_starmap as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"product".as_ptr(), registeredfunction_function: Some(it_product as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"combinations".as_ptr(), registeredfunction_function: Some(it_combinations as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"permutations".as_ptr(), registeredfunction_function: Some(it_permutations as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"groupby".as_ptr(), registeredfunction_function: Some(it_groupby as unsafe fn(*mut State) -> i32) },
    RegisteredFunction { registeredfunction_name: c"unique".as_ptr(), registeredfunction_function: Some(it_unique as unsafe fn(*mut State) -> i32) },
];

pub unsafe fn luaopen_itertools(state: *mut State) -> i32 {
    unsafe {
        (*state).lua_createtable();
        lual_setfuncs(state, ITERTOOLS_FUNCTIONS.as_ptr(), ITERTOOLS_FUNCTIONS.len(), 0);
        1
    }
}
