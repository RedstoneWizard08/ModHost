quickhost::quickhost! {
    versions = [astro::vers::get_astro_versions().await?];
    loaders = [modhost::loaders!["AstroModIntegrator", "UE4SS"]];
    tags = [astro::tags::tags()];
}
