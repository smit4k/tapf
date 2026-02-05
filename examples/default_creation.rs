use tapf::AnimationFile;

fn main() -> tapf::Result<()> {
    // Try to load a file that doesn't exist - it will create a default one
    let animation = AnimationFile::load_or_create_default("test_default.toml")?;

    println!("Created animation with {} frames", animation.frame_count());
    println!("FPS: {}", animation.animation.fps);
    println!("Loop: {}", animation.should_loop());
    println!(
        "Dimensions: {}x{}",
        animation.metadata.width, animation.metadata.height
    );
    println!("\nAnimation saved to test_default.toml");

    // Validate the animation
    animation.validate()?;
    println!("Animation is valid!");

    Ok(())
}
