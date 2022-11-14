# Microchip ATSAML21G18B device crate

* SVD file obtained from the _Microchip SAML21 Series Device Support_ pack from https://packs.download.microchip.com/

* converted to Rust code using `svd2rust -i ATSAML21G18B.svd` ([svd2rust docs](https://docs.rs/svd2rust/))

* unrolled using `form -i lib.rs -o src` ([form docs](https://docs.rs/form/))

* beautified using `cargo fmt`

