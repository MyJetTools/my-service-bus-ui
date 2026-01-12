use ci_utils::ci_generator::CiGenerator;

fn main() {
    CiGenerator::new(env!("CARGO_PKG_NAME"))
        .as_dioxus_fullstack_service()
        .generate_github_ci_file()
        .set_docker_image_name("ghcr.io/myjettools/my-service-bus-ui")
        .build();
}
