/* tslint:disable */
/* eslint-disable */
/**
*/
export class JuliaDrawer {
  free(): void;
/**
* @param {number} size
* @param {number} depth
* @param {number} xmin
* @param {number} ymin
* @param {number} scale
* @param {CanvasRenderingContext2D} ctx
*/
  constructor(size: number, depth: number, xmin: number, ymin: number, scale: number, ctx: CanvasRenderingContext2D);
/**
* @param {number} a
* @param {number} b
*/
  set_complex(a: number, b: number): void;
/**
*/
  display(): void;
}
/**
*/
export class MandelbrotDrawer {
  free(): void;
/**
* @param {number} size
* @param {number} depth
* @param {number} xmin
* @param {number} ymin
* @param {number} scale
* @param {CanvasRenderingContext2D} ctx
*/
  constructor(size: number, depth: number, xmin: number, ymin: number, scale: number, ctx: CanvasRenderingContext2D);
/**
*/
  display(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mandelbrotdrawer_free: (a: number) => void;
  readonly mandelbrotdrawer_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly mandelbrotdrawer_display: (a: number, b: number) => void;
  readonly __wbg_juliadrawer_free: (a: number) => void;
  readonly juliadrawer_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly juliadrawer_set_complex: (a: number, b: number, c: number) => void;
  readonly juliadrawer_display: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
