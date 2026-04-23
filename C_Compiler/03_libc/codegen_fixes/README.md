# C Codegen Fixes — Priority Tracker

## Fix C-01: Struct Field Access
**Files:** `isa/compiler/expressions.rs`, `isa/compiler/statements.rs`  
**Blocks:** struct tm, FILE*, div_t, sigaction, ALL DirectX  
**Status:** 🔴 Pending  

## Fix C-02: Float/Double SSE2 Codegen
**Files:** `isa/compiler/expressions.rs`, `isa/encoder.rs`  
**Blocks:** math.h, OpenGL advanced, DirectX, atof, strtod  
**Status:** 🔴 Pending  

## Fix C-03: Byte-Level Memory Operations
**Files:** `isa/compiler/expressions.rs`  
**Blocks:** memcpy, memset, memcmp, strchr  
**Status:** 🔴 Pending  

## Fix C-04: Function Pointers / Indirect Calls
**Files:** `isa/compiler/functions.rs`, `isa/compiler/expressions.rs`  
**Blocks:** qsort, bsearch, signal, atexit, DX vtable  
**Status:** 🔴 Pending  

## Fix C-05: goto / Label Forward References
**Files:** `isa/bit_resolver.rs`, `isa/compiler/statements.rs`  
**Blocks:** DX9-12 vtable patterns  
**Status:** 🔴 Pending  

## Fix C-06: va_list / Variadic Arguments
**Files:** `isa/compiler/functions.rs`  
**Blocks:** vprintf, vfprintf, custom variadic  
**Status:** 🔴 Pending  

## Fix C-07: Global/Static Variables
**Files:** `isa/compiler/statements.rs`  
**Blocks:** errno, stdin/stdout/stderr  
**Status:** 🔴 Pending  

## Fix C-08: Array Initializers
**Files:** `isa/compiler/arrays.rs`  
**Blocks:** int a[]={1,2,3}, char s[]="hello"  
**Status:** 🔴 Pending  

## Fix C-09: Cast Codegen
**Files:** `isa/compiler/expressions.rs`  
**Blocks:** pointer casts, int↔ptr, signed↔unsigned  
**Status:** 🔴 Pending  

## Fix C-10: sizeof Codegen
**Files:** `isa/compiler/expressions.rs`  
**Blocks:** sizeof(struct), sizeof(array)  
**Status:** 🔴 Pending  
