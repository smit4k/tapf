use tapf::AnimationFile;

fn main() -> tapf::Result<()> {
    // Load an animation from a TOML file
    let anim = AnimationFile::load("examples/sample_idle.toml")?;

    // Validate the animation
    anim.validate()?;

    // Print metadata
    println!("Animation: {}", anim.metadata.name);
    println!("Author: {}", anim.metadata.author);
    println!(
        "Dimensions: {}x{}",
        anim.metadata.width, anim.metadata.height
    );
    println!("FPS: {}", anim.animation.fps);
    println!("Loops: {}", anim.should_loop());
    println!("Frame count: {}", anim.frame_count());
    println!();

    // Display each frame
    for (i, frame) in anim.animation.frames.iter().enumerate() {
        println!("=== Frame {} ({}ms) ===", i, anim.frame_duration(i));
        println!("{}", frame.data);
        println!();
    }

    Ok(())
}

