use std::path::PathBuf;

use sev::firmware::guest::{GuestPolicy, PlatformInfo};
use sev::firmware::host::TcbVersion;
use sev::measurement::{
    snp::{snp_calc_launch_digest, SnpMeasurementArgs},
    vmsa::{GuestFeatures, VMMType},
    vcpu_types::CpuType
};
use snafu::{whatever, ResultExt, Whatever};

fn main() {
    println!("Hello, world!");
    let result = compute_expected_hash();
    
    match result {
        Ok(_) => println!("Okay!"),
        Err(e) => println!("Not okay: {}", e),
    }
}

fn compute_expected_hash() ->  Result<[u8; 384 / 8], Whatever> {
    let snp_measure_args = SnpMeasurementArgs {
        vcpus: 1,
        vcpu_type: CpuType::EpycV4,
        ovmf_file: PathBuf::from("/home/jordi/snp-release-2024-11-12/usr/local/share/qemu/OVMF.fd"),
        guest_features: sev::measurement::vmsa::GuestFeatures(0b1),
        kernel_file: Some(PathBuf::from("/boot/config-6.8.0-48-generic")),
        initrd_file: Some(PathBuf::from("/boot/initrd.img-6.8.0-48-generic")),
        append: None,
        ovmf_hash_str: None,
        vmm_type: Some(VMMType::QEMU),
    };
    let ld = snp_calc_launch_digest(snp_measure_args)
            .whatever_context("failed to compute launch digest")?;
    let ld_vec = bincode::serialize(&ld).whatever_context("failed to bincode serialized SnpLaunchDigest to Vec<u8>")?;
    let ld_arr : [u8; 384 / 8] = match ld_vec.try_into() {
        Ok(v) => v,
        Err(_) => whatever!("SnpLaunchDigest has unexpected length"),
    };
    print!("created measurement");
    Ok(ld_arr)
}