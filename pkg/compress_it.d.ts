/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} input
* @returns {Uint8Array}
*/
export function snappy_compress(input: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} input
* @returns {Uint8Array}
*/
export function snappy_decompress(input: Uint8Array): Uint8Array;
/**
* @param {number} input_len
* @returns {number}
*/
export function snappy_max_compress_len(input_len: number): number;
/**
* @param {Uint8Array} input
* @returns {number}
*/
export function snappy_decompress_len(input: Uint8Array): number;
/**
* @param {Uint8Array} input
* @returns {Uint8Array}
*/
export function gzip_compress(input: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly snappy_compress: (a: number, b: number, c: number) => void;
  readonly snappy_decompress: (a: number, b: number, c: number) => void;
  readonly snappy_max_compress_len: (a: number) => number;
  readonly snappy_decompress_len: (a: number, b: number) => number;
  readonly gzip_compress: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
