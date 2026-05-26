import '@supabase/auth-helpers-sveltekit';
import { liveQuery } from 'dexie';
import { r as readable, d as derived } from './index2.js';
import 'base64-js';
import FileSaver from 'file-saver';
import { s as supabase } from './client.js';

let db;
const concatBuffers = (buf1, buf2) => {
  let ret = new Uint8Array(buf1.length + buf2.length);
  ret.set(buf1);
  ret.set(buf2, buf1.length);
  return ret;
};
const combine_patches = (patches) => {
  return patches.reduce(
    (acc, patch) => concatBuffers(acc, patch.data),
    new Uint8Array()
  );
};
const patchesLiveQuery = (model) => {
  return liveQuery(() => db.model_patches.where({ model }).toArray());
};
const documentBinaryStore = (model) => {
  return readable(
    new Uint8Array(),
    (setter) => {
      patchesLiveQuery(model).subscribe((patches) => {
        let data = combine_patches(patches);
        setter(data);
      });
    }
  );
};
const model_by_id = async (user, id) => {
  let results = await db.models.where({ id, user }).toArray();
  return results[0];
};
const model_patches = async (model) => {
  return await db.model_patches.where({ model }).toArray();
};
const model_binary = async (model) => {
  return combine_patches(await model_patches(model));
};
const save = async (model, patch) => {
  let model_dto = { id: model.id, user: model.user, name: model.name, description: model.description };
  let patch_dto = { model: patch.model, data: patch.data };
  await db.transaction("rw", [db.models, db.model_patches], async () => {
    db.models.put(model_dto, model_dto.id);
    db.model_patches.add(patch_dto);
  });
};

const dexie = /*#__PURE__*/Object.freeze(/*#__PURE__*/Object.defineProperty({
  __proto__: null,
  documentBinaryStore,
  model_binary,
  model_by_id,
  model_patches,
  save
}, Symbol.toStringTag, { value: 'Module' }));

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); }
let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5aafbc2ebd344371(arg0, arg1, addHeapObject(arg2));
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1) >>> 0;
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
*/
function setPanicHook() {
    wasm.setPanicHook();
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
function __wbg_adapter_171(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h16771083f99e8c84(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}
/**
*/
class Cell {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Cell.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cell_free(ptr);
    }
    /**
    * @returns {any}
    */
    get kind() {
        const ret = wasm.__wbg_get_cell_kind(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    get row() {
        const ret = wasm.__wbg_get_cell_row(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get column() {
        const ret = wasm.__wbg_get_cell_column(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Array<any>}
    */
    get placements() {
        const ret = wasm.cell_placements(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string | undefined}
    */
    get audience() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cell_audience(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {string | undefined}
    */
    get stream() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.cell_stream(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {boolean}
    */
    is_empty() {
        const ret = wasm.cell_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {GridPlacement | undefined}
    */
    get placement() {
        const ret = wasm.cell_placement(this.__wbg_ptr);
        return ret === 0 ? undefined : GridPlacement.__wrap(ret);
    }
}
/**
*/
class EventModelGrid {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EventModelGrid.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eventmodelgrid_free(ptr);
    }
    /**
    * @returns {any}
    */
    get state() {
        const ret = wasm.__wbg_get_eventmodelgrid_state(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_eventmodelgrid_name(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {string}
    */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_eventmodelgrid_description(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {number}
    */
    get column_count() {
        const ret = wasm.__wbg_get_eventmodelgrid_column_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get row_count() {
        const ret = wasm.__wbg_get_eventmodelgrid_row_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {GridLane}
    */
    get default_audience() {
        const ret = wasm.__wbg_get_eventmodelgrid_default_audience(this.__wbg_ptr);
        return GridLane.__wrap(ret);
    }
    /**
    * @returns {GridLane}
    */
    get timeline() {
        const ret = wasm.__wbg_get_eventmodelgrid_timeline(this.__wbg_ptr);
        return GridLane.__wrap(ret);
    }
    /**
    * @returns {GridLane}
    */
    get default_stream() {
        const ret = wasm.__wbg_get_eventmodelgrid_default_stream(this.__wbg_ptr);
        return GridLane.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.eventmodelgrid_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {Array<any>}
    */
    get audiences() {
        const ret = wasm.eventmodelgrid_audiences(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    get streams() {
        const ret = wasm.eventmodelgrid_streams(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    get flows() {
        const ret = wasm.eventmodelgrid_flows(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} maybe_placement_id
    * @returns {GridPlacement | undefined}
    */
    placement_by_id(maybe_placement_id) {
        const ptr0 = passStringToWasm0(maybe_placement_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelgrid_placement_by_id(this.__wbg_ptr, ptr0, len0);
        return ret === 0 ? undefined : GridPlacement.__wrap(ret);
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {Cell | undefined}
    */
    cell_by_row_col(row, col) {
        const ret = wasm.eventmodelgrid_cell_by_row_col(this.__wbg_ptr, row, col);
        return ret === 0 ? undefined : Cell.__wrap(ret);
    }
}
/**
*/
class EventModelStateManager {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(EventModelStateManager.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_eventmodelstatemanager_free(ptr);
    }
    /**
    * @returns {Promise<string | undefined>}
    */
    name() {
        const ret = wasm.eventmodelstatemanager_name(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string | undefined} maybe_id_str
    * @param {string} user
    */
    constructor(maybe_id_str, user) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = isLikeNone(maybe_id_str) ? 0 : passStringToWasm0(maybe_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(user, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            wasm.eventmodelstatemanager_new(retptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return EventModelStateManager.__wrap(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} bin
    */
    refresh(bin) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.eventmodelstatemanager_refresh(retptr, this.__wbg_ptr, addHeapObject(bin));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Promise<string>}
    */
    export() {
        const ret = wasm.eventmodelstatemanager_export(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Promise<EventModelGrid>}
    */
    grid() {
        const ret = wasm.eventmodelstatemanager_grid(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} name
    * @returns {Promise<EventModelGrid>}
    */
    create(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_create(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string} name
    * @param {string} model_id_str
    * @returns {Promise<EventModelGrid>}
    */
    rename(name, model_id_str) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_rename(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @returns {Promise<EventModelGrid>}
    */
    delete(model_id_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_delete(this.__wbg_ptr, ptr0, len0);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} name
    * @param {number} index
    * @param {string | undefined} maybe_audience_str
    * @returns {Promise<EventModelGrid>}
    */
    define_and_place_interface(model_id_str, name, index, maybe_audience_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_audience_str) ? 0 : passStringToWasm0(maybe_audience_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_define_and_place_interface(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} name
    * @param {number} index
    * @returns {Promise<EventModelGrid>}
    */
    define_and_place_command(model_id_str, name, index) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_define_and_place_command(this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} name
    * @param {number} index
    * @param {string | undefined} maybe_stream_str
    * @returns {Promise<EventModelGrid>}
    */
    define_and_place_event(model_id_str, name, index, maybe_stream_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_stream_str) ? 0 : passStringToWasm0(maybe_stream_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_define_and_place_event(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} name
    * @param {number} index
    * @returns {Promise<EventModelGrid>}
    */
    define_and_place_read_model(model_id_str, name, index) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_define_and_place_read_model(this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @param {string | undefined} maybe_audience_str
    * @returns {Promise<EventModelGrid>}
    */
    duplicate_interface_placement(model_id_str, placement_id_str, index, maybe_audience_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_audience_str) ? 0 : passStringToWasm0(maybe_audience_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_duplicate_interface_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @returns {Promise<EventModelGrid>}
    */
    duplicate_timeline_placement(model_id_str, placement_id_str, index) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_duplicate_timeline_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @param {string | undefined} maybe_stream_str
    * @returns {Promise<EventModelGrid>}
    */
    duplicate_event_placement(model_id_str, placement_id_str, index, maybe_stream_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_stream_str) ? 0 : passStringToWasm0(maybe_stream_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_duplicate_event_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {number} index
    * @param {string} direction
    * @param {number} count
    * @returns {Promise<EventModelGrid>}
    */
    insert_columns(model_id_str, index, direction, count) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(direction, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_insert_columns(this.__wbg_ptr, ptr0, len0, index, ptr1, len1, count);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {Uint8Array} json
    * @param {number} offset
    * @returns {Promise<EventModelGrid>}
    */
    import(model_id_str, json, offset) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_import(this.__wbg_ptr, ptr0, len0, addHeapObject(json), offset);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @param {string | undefined} maybe_audience_str
    * @returns {Promise<EventModelGrid>}
    */
    move_interface_placement(model_id_str, placement_id_str, index, maybe_audience_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_audience_str) ? 0 : passStringToWasm0(maybe_audience_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_move_interface_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @returns {Promise<EventModelGrid>}
    */
    move_timeline_placement(model_id_str, placement_id_str, index) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_move_timeline_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {number} index
    * @param {string | undefined} maybe_stream_str
    * @returns {Promise<EventModelGrid>}
    */
    move_event_placement(model_id_str, placement_id_str, index, maybe_stream_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(maybe_stream_str) ? 0 : passStringToWasm0(maybe_stream_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_move_event_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @returns {Promise<EventModelGrid>}
    */
    remove_placement(model_id_str, placement_id_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_remove_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} placement_id_str
    * @param {string} name
    * @returns {Promise<EventModelGrid>}
    */
    rename_placement(model_id_str, placement_id_str, name) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_rename_placement(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} kind
    * @param {string} lane_id_str
    * @param {string} name
    * @returns {Promise<EventModelGrid>}
    */
    rename_lane(model_id_str, kind, lane_id_str, name) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(lane_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_rename_lane(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} kind
    * @param {string} lane_id_str
    * @param {number} index
    * @returns {Promise<EventModelGrid>}
    */
    reorder_lane(model_id_str, kind, lane_id_str, index) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(lane_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_reorder_lane(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, index);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} kind
    * @param {string} lane_id_str
    * @returns {Promise<EventModelGrid>}
    */
    remove_lane(model_id_str, kind, lane_id_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(lane_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_remove_lane(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} kind
    * @param {number} index
    * @param {string} name
    * @returns {Promise<EventModelGrid>}
    */
    add_lane(model_id_str, kind, index, name) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(kind, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_add_lane(this.__wbg_ptr, ptr0, len0, ptr1, len1, index, ptr2, len2);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {number} index
    * @param {number} deletion_count
    * @param {string} addition
    * @returns {Promise<EventModelGrid>}
    */
    edit_description(model_id_str, index, deletion_count, addition) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(addition, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_edit_description(this.__wbg_ptr, ptr0, len0, index, deletion_count, ptr1, len1);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} component_type_str
    * @param {string} component_id_str
    * @param {number} index
    * @param {number} deletion_count
    * @param {string} addition
    * @returns {Promise<EventModelGrid>}
    */
    edit_component_description(model_id_str, component_type_str, component_id_str, index, deletion_count, addition) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(component_type_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(component_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(addition, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_edit_component_description(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, index, deletion_count, ptr3, len3);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} source_placement_id_str
    * @param {string | undefined} source_anchor_str
    * @param {string} target_placement_id_str
    * @param {string | undefined} target_anchor_str
    * @returns {Promise<EventModelGrid>}
    */
    connect_flow(model_id_str, source_placement_id_str, source_anchor_str, target_placement_id_str, target_anchor_str) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(source_placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(source_anchor_str) ? 0 : passStringToWasm0(source_anchor_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(target_placement_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        var ptr4 = isLikeNone(target_anchor_str) ? 0 : passStringToWasm0(target_anchor_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len4 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_connect_flow(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
        return takeObject(ret);
    }
    /**
    * @param {string} model_id_str
    * @param {string} interface_id_str
    * @param {string} interface_type
    * @param {string | undefined} interface_url
    * @returns {Promise<EventModelGrid>}
    */
    configure_interface(model_id_str, interface_id_str, interface_type, interface_url) {
        const ptr0 = passStringToWasm0(model_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(interface_id_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(interface_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        var ptr3 = isLikeNone(interface_url) ? 0 : passStringToWasm0(interface_url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len3 = WASM_VECTOR_LEN;
        const ret = wasm.eventmodelstatemanager_configure_interface(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
        return takeObject(ret);
    }
}
/**
*/
class FlowArrow {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FlowArrow.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_flowarrow_free(ptr);
    }
    /**
    * @returns {FlowPort}
    */
    get from() {
        const ret = wasm.__wbg_get_flowarrow_from(this.__wbg_ptr);
        return FlowPort.__wrap(ret);
    }
    /**
    * @param {FlowPort} arg0
    */
    set from(arg0) {
        _assertClass(arg0, FlowPort);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_flowarrow_from(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {FlowPort}
    */
    get to() {
        const ret = wasm.__wbg_get_flowarrow_to(this.__wbg_ptr);
        return FlowPort.__wrap(ret);
    }
    /**
    * @param {FlowPort} arg0
    */
    set to(arg0) {
        _assertClass(arg0, FlowPort);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_flowarrow_to(this.__wbg_ptr, ptr0);
    }
    /**
    * @returns {string}
    */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.flowarrow_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
}
/**
*/
class FlowPort {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(FlowPort.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_flowport_free(ptr);
    }
    /**
    * @returns {number}
    */
    get anchor() {
        const ret = wasm.__wbg_get_flowport_anchor(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set anchor(arg0) {
        wasm.__wbg_set_flowport_anchor(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {string}
    */
    get kind() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_flowport_kind(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set kind(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_flowport_kind(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get placement_id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.flowport_placement_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
}
/**
*/
class GridLane {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GridLane.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gridlane_free(ptr);
    }
    /**
    * @returns {any}
    */
    get kind() {
        const ret = wasm.__wbg_get_gridlane_kind(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number | undefined}
    */
    get index() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_gridlane_index(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return r0 === 0 ? undefined : r1 >>> 0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    get row() {
        const ret = wasm.__wbg_get_gridlane_row(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_gridlane_name(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {string | undefined}
    */
    get id() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.gridlane_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {Array<any>}
    */
    get cells() {
        const ret = wasm.gridlane_cells(this.__wbg_ptr);
        return takeObject(ret);
    }
}
/**
*/
class GridPlacement {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GridPlacement.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gridplacement_free(ptr);
    }
    /**
    * @returns {any}
    */
    get kind() {
        const ret = wasm.__wbg_get_gridplacement_kind(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    get index() {
        const ret = wasm.__wbg_get_gridplacement_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_gridplacement_name(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {string}
    */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_gridplacement_description(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {InterfaceConfig | undefined}
    */
    get interface_config() {
        const ret = wasm.__wbg_get_gridplacement_interface_config(this.__wbg_ptr);
        return ret === 0 ? undefined : InterfaceConfig.__wrap(ret);
    }
    /**
    * @returns {string}
    */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.flowport_placement_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @returns {string}
    */
    get component_id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.gridplacement_component_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
}
/**
*/
class InterfaceConfig {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(InterfaceConfig.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_interfaceconfig_free(ptr);
    }
    /**
    * @returns {any}
    */
    get kind() {
        const ret = wasm.__wbg_get_interfaceconfig_kind(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {string | undefined}
    */
    get url() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_interfaceconfig_url(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v1;
            if (r0 !== 0) {
                v1 = getStringFromWasm0(r0, r1).slice();
                wasm.__wbindgen_free(r0, r1 * 1);
            }
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
}
/**
*/
class Model {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Model.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_model_free(ptr);
    }
    /**
    * @returns {string}
    */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_model_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get user() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_model_user(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set user(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_user(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_model_name(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {string}
    */
    get description() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_model_description(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set description(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_description(this.__wbg_ptr, ptr0, len0);
    }
}
/**
*/
class Patch {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Patch.prototype);
        obj.__wbg_ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_patch_free(ptr);
    }
    /**
    * @returns {string}
    */
    get model() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_model_id(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1);
        }
    }
    /**
    * @param {string} arg0
    */
    set model(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
    * @returns {Uint8Array}
    */
    get data() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_patch_data(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Uint8Array} arg0
    */
    set data(arg0) {
        const ptr0 = passArray8ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_model_user(this.__wbg_ptr, ptr0, len0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_eventmodelgrid_new = function(arg0) {
        const ret = EventModelGrid.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbg_gridplacement_new = function(arg0) {
        const ret = GridPlacement.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_flowarrow_new = function(arg0) {
        const ret = FlowArrow.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_gridlane_new = function(arg0) {
        const ret = GridLane.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_save_25e3ea303ee3bc2f = function() { return handleError(function (arg0, arg1) {
        const ret = save(Model.__wrap(arg0), Patch.__wrap(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_cell_new = function(arg0) {
        const ret = Cell.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1);
        }
    };
    imports.wbg.__wbg_instanceof_Window_c5579e140698a9dc = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_sessionStorage_263f344230ee7188 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).sessionStorage;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_log_dc06ec929fc95a20 = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_log_c9bb086ced3cfca3 = function(arg0, arg1) {
        console.log(getObject(arg0), getObject(arg1));
    };
    imports.wbg.__wbg_getItem_84095995ffbc84fc = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    }, arguments) };
    imports.wbg.__wbg_setItem_e9a65f0e6892d9c9 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_3774744e221a22ad = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_e950366c42764a07 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_crypto_70a96de3b6b73dac = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_process_dd1577445152112e = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_58036bec3add9e6f = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_6a9d28205ed5b0d8 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_msCrypto_adbc770ec9eca9c7 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_f05d779769764e82 = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_new_0394642eae39db16 = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_c9e6043b8ad84109 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_557a2f2deacc4912 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_742dd6eab3e9211e = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_c409e731db53a0e2 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_b70c095388441f2d = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_1c72617491ed7194 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_push_109cfc26d02582dd = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_call_587b30eea3e09332 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_2b55e405e4af4986 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_171(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_ae38ad63c43ff98b = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_8df675b8bb5d5e3c = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_835b073a479138e5 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_55ba7a6b1b92e2ac = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_88d1d8be5df94b9b = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_09938a7d020f049b = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_3698e3ca519b3c3c = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_length_0aab7ffd65ad19ed = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_newwithlength_89eeca401d8918c2 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_d82be056deb4ad27 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper918 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 257, __wbg_adapter_24);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedInt32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('state_client_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

const exportJson = (filename, content) => {
  const blob = new Blob([content], { type: "application/json;charset=utf-8" });
  FileSaver.saveAs(blob, `${filename}.json`, { autoBom: true });
};

const BUCKET = "interface-images";
const blobSha = (blob) => {
  return new Promise((resolve, reject) => {
    const fileReader = new FileReader();
    fileReader.addEventListener("load", () => {
      crypto.subtle.digest("SHA-1", fileReader.result).then((buffer) => {
        resolve(btoa(String.fromCharCode(...new Uint8Array(buffer))));
      });
    });
    fileReader.addEventListener("error", () => {
      reject(fileReader.error);
    });
    console.log("blob", blob);
    fileReader.readAsArrayBuffer(blob);
  });
};
const upload_interface_image = async (model_id, blob) => {
  let sha = await blobSha(blob);
  console.log("upload_interface_image", model_id, blob, sha);
  let name = `${model_id}/${sha}`;
  await supabase.storage.from(BUCKET).upload(name, blob, { upsert: true });
  const { data } = supabase.storage.from(BUCKET).getPublicUrl(name);
  console.log("upload_interface_image returning:", data);
  return data.publicUrl;
};

const initialize_decider = async (id, user) => {
  await __wbg_init();
  setPanicHook();
  let manager = new EventModelStateManager(id, user);
  const { model_binary, documentBinaryStore } = await Promise.resolve().then(() => dexie);
  let store;
  if (id) {
    let initial_bin = await model_binary(id);
    manager.refresh(initial_bin);
    let $doc_binary_store = documentBinaryStore(id);
    store = derived($doc_binary_store, (bin, setter) => {
      try {
        manager.refresh(bin);
        manager.grid().then((grid) => {
          setter(grid);
        }).catch((_) => {
          setter(null);
        });
      } catch {
        setter(null);
      }
      return () => console.debug("Unsubscribed last subscriber to empty Event Model State");
    });
  } else {
    store = readable(null, (_) => {
      return () => console.debug("Unsubscribed last subscriber to empty Event Model State");
    });
  }
  return {
    grid: store,
    decider: {
      export_json: async () => {
        let name = await manager.name();
        if (name) {
          try {
            exportJson(name, await manager.export());
          } catch (e) {
            console.error("Error exporting JSON:", e);
          }
        }
      },
      placement_by_id: async (placement_id) => {
        let grid = await manager.grid();
        return grid.placement_by_id(placement_id);
      },
      create_model: async (name) => {
        return await manager.create(name);
      },
      rename_model: async (name) => {
        return await manager.rename(id, name);
      },
      edit_description: async (index, deletion_count, addition) => {
        return await manager.edit_description(id, index, deletion_count, addition);
      },
      define_and_place_interface: async (name, index, audience) => {
        return await manager.define_and_place_interface(id, name, index, audience);
      },
      define_and_place_command: async (name, index) => {
        return await manager.define_and_place_command(id, name, index);
      },
      define_and_place_event: async (name, index, stream) => {
        return await manager.define_and_place_event(id, name, index, stream);
      },
      define_and_place_read_model: async (name, index) => {
        return await manager.define_and_place_read_model(id, name, index);
      },
      delete_model: async () => {
        return await manager.delete(id);
      },
      duplicate_interface_placement: async (placement_id, index, audience) => {
        return await manager.duplicate_interface_placement(id, placement_id, index, audience);
      },
      duplicate_timeline_placement: async (placement_id, index) => {
        return await manager.duplicate_timeline_placement(id, placement_id, index);
      },
      duplicate_event_placement: async (placement_id, index, stream) => {
        return await manager.duplicate_event_placement(id, placement_id, index, stream);
      },
      import_json: async (json_bytes, offset) => {
        return await manager.import(id, json_bytes, offset);
      },
      move_interface_placement: async (placement_id, index, audience) => {
        return await manager.move_interface_placement(id, placement_id, index, audience);
      },
      move_timeline_placement: async (placement_id, index) => {
        return await manager.move_timeline_placement(id, placement_id, index);
      },
      move_event_placement: async (placement_id, index, stream) => {
        return await manager.move_event_placement(id, placement_id, index, stream);
      },
      remove_placement: async (placement) => {
        return await manager.remove_placement(id, placement);
      },
      rename_placement: async (placement, name) => {
        return await manager.rename_placement(id, placement, name);
      },
      rename_lane: async (kind, lane_id, name) => {
        return await manager.rename_lane(id, kind, lane_id, name);
      },
      reorder_lane: async (kind, lane_id, index) => {
        return await manager.reorder_lane(id, kind, lane_id, index);
      },
      remove_lane: async (kind, lane_id) => {
        return await manager.remove_lane(id, kind, lane_id);
      },
      add_lane: async (kind, index, name) => {
        return manager.add_lane(id, kind, index, name);
      },
      insert_columns: async (index, direction, count) => {
        return manager.insert_columns(id, index, direction, count);
      },
      connect_flow: async (source_placement_id_str, source_anchor_str, target_placement_id_str, target_anchor_str) => {
        return await manager.connect_flow(id, source_placement_id_str, source_anchor_str, target_placement_id_str, target_anchor_str);
      },
      configure_interface: async (interface_id_str, interface_type, interface_url = void 0, image_blob = void 0) => {
        let url;
        if (image_blob) {
          try {
            url = await upload_interface_image(id, image_blob);
          } catch (e) {
            console.error("Error uploading interface image", e);
            throw e;
          }
        } else {
          url = interface_url;
        }
        return await manager.configure_interface(id, interface_id_str, interface_type, url);
      }
    }
  };
};

export { dexie as d, initialize_decider as i };
