#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// The following structs are used to implement the lowest level
// of the FFI, and thus useful to multiple uniffied crates.
// We ensure they are declared exactly once, with a header guard, UNIFFI_SHARED_H.
#ifdef UNIFFI_SHARED_H
    // We also try to prevent mixing versions of shared uniffi header structs.
    // If you add anything to the #else block, you must increment the version suffix in UNIFFI_SHARED_HEADER_V4
    #ifndef UNIFFI_SHARED_HEADER_V4
        #error Combining helper code from multiple versions of uniffi is not supported
    #endif // ndef UNIFFI_SHARED_HEADER_V4
#else
#define UNIFFI_SHARED_H
#define UNIFFI_SHARED_HEADER_V4

// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️

typedef struct RustBuffer
{
    int32_t capacity;
    int32_t len;
    uint8_t *_Nullable data;
} RustBuffer;

typedef struct ForeignBytes
{
    int32_t len;
    const uint8_t *_Nullable data;
} ForeignBytes;
typedef struct RustCallStatus {
    int8_t code;
    RustBuffer errorBuf;
} RustCallStatus;


typedef int32_t (*ForeignCallback)(uint64_t, int32_t, const uint8_t *_Nonnull, int32_t, RustBuffer *_Nonnull);

typedef void (*UniFfiRustFutureContinuation)(uint64_t, int16_t);

// ⚠️ Attention: If you change this #else block (ending in `#endif // def UNIFFI_SHARED_H`) you *must* ⚠️
// ⚠️ increment the version suffix in all instances of UNIFFI_SHARED_HEADER_V4 in this file.           ⚠️
#endif // def UNIFFI_SHARED_H

void uniffi_ed25519_bip32_fn_free_xprvwrapper(void*_Nonnull ptr_, RustCallStatus *_Nonnull out_status);
void*_Nonnull uniffi_ed25519_bip32_fn_method_xprvwrapper_derive(void*_Nonnull ptr_, RustBuffer scheme_, uint32_t index_, RustCallStatus *_Nonnull out_status);
void*_Nonnull uniffi_ed25519_bip32_fn_method_xprvwrapper_public(void*_Nonnull ptr_, RustCallStatus *_Nonnull out_status);
void uniffi_ed25519_bip32_fn_free_xpubwrapper(void*_Nonnull ptr_, RustCallStatus *_Nonnull out_status);
void*_Nonnull uniffi_ed25519_bip32_fn_method_xpubwrapper_derive(void*_Nonnull ptr_, RustBuffer scheme_, uint32_t index_, RustCallStatus *_Nonnull out_status);
RustBuffer ffi_ed25519_bip32_rustbuffer_alloc(int32_t size_, RustCallStatus *_Nonnull out_status);
RustBuffer ffi_ed25519_bip32_rustbuffer_from_bytes(ForeignBytes bytes_, RustCallStatus *_Nonnull out_status);
void ffi_ed25519_bip32_rustbuffer_free(RustBuffer buf_, RustCallStatus *_Nonnull out_status);
RustBuffer ffi_ed25519_bip32_rustbuffer_reserve(RustBuffer buf_, int32_t additional_, RustCallStatus *_Nonnull out_status);
uint16_t uniffi_ed25519_bip32_checksum_method_xprvwrapper_derive(void);
uint16_t uniffi_ed25519_bip32_checksum_method_xprvwrapper_public(void);
uint16_t uniffi_ed25519_bip32_checksum_method_xpubwrapper_derive(void);
uint32_t ffi_ed25519_bip32_uniffi_contract_version(void);
