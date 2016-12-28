extern crate pkg_config;

fn main() {
    let probed = pkg_config::Config::new().atleast_version("1.11.0").probe("hwloc");
    if probed.is_ok() {
        return;
    }
}
