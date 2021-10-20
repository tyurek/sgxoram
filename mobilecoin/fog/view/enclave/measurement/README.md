# View Enclave Measurement

Compile-time declaration of enclave measurement data.

This crate's ostensible purpose is to provide programmatic access to the Fog View Enclave's `SIGSTRUCT` artifact, which contains `MRENCLAVE`, `MRSIGNER`, and other metadata about an enclave. This metadata file is provided at compile-time.

It's practical purpose is to "bake in" those values for a release, and if no metadata is provided, compile and sign the enclave binary, then extract that metadata to be "baked in". In order to facilitate this purpose, the build script will use the following environment variables to determine what artifacts will be generated:

|Variable|Type|Actions|
---------|----|-------|
|`VIEW_ENCLAVE_CSS`|`css`|If present, the build will read the file at the given path, and inject it into the crate for runtime evaluation of it's contents.|
|`VIEW_ENCLAVE_SIGNED`|`.signed.so`|The signed enclave binary used to extract `VIEW_ENCLAVE_CSS`.|
|`VIEW_ENCLAVE_UNSIGNED`|`.so`|The pre-compiled enclave binary which will be used to create `VIEW_ENCLAVE_SIGNED`.|
|`VIEW_ENCLAVE_PRIVKEY`|`.pem`|The private key used to create a `VIEW_ENCLAVE_SIGNED` using `VIEW_ENCLAVE_UNSIGNED`.|
|`VIEW_ENCLAVE_GENDATA`|`.dat`|Data previously extracted from `VIEW_ENCLAVE_UNSIGNED` which has been signed offline.|
|`VIEW_ENCLAVE_SIGNATURE`|`.sig`|The signature over `VIEW_ENCLAVE_GENDATA` produced by the owner of `VIEW_ENCLAVE_PUBKEY`.|
|`VIEW_ENCLAVE_PUBKEY`|`.pem`|The public key of the signing key which generated `VIEW_ENCLAVE_SIGNATURE`.|
|`VIEW_ENCLAVE_LDS`|`.lds`|An optional linker script to provide when building `VIEW_ENCLAVE_UNSIGNED` from scratch.|

 The basic procedure here is:

  1. If `CSS` is defined, use that file to extract the `SIGSTRUCT`.
  1. Else, if `SIGNED` is defined, use that file in step 1.
  1. Else, if `UNSIGNED` is defined
      1. If `PRIVKEY` is defined, do a one-shot, insecure signature to produce `SIGNED` for step 2.
      1. If `GENDATA`, `SIGNATURE`, and `PUBKEY` are defined, simply assemble the `SIGNED` binary from the four components for step 2.
      1. If neither `PRIVKEY` nor `GENDATA`/`SIGNATURE`/`PUBKEY` are defined, generate a one-time private key and perform an online/one-shot/insecure signature to produce the `SIGNED` binary for step 2.
  1. Else, compile and link the `UNSIGNED` binary for use in step 3.

 Some additional notes:

  1. If the `UNSIGNED` binary was built from scratch, it's `GENDATA` must match the one provided for step 3.2 to succeed.
  1. If a private signing key was generated in step 3.3, then these artifacts will have a unique `MRSIGNER` value, and any clients must necessarily extract the data using this measurement crate.
  1. Generated private signing keys are not exported from the build process, so they cannot be re-used for other enclaves.