
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::task::spawn;
use tokio::time::sleep;
use mouse_position::mouse_position::Mouse;
use enigo::*;
use std::str::FromStr;

const DELTA: i32 = 1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init();

    let mut args = std::env::args();

    match args.nth(1) {
        Some(arg) => twiggle_the_whiggle(u32::from_str(&arg)?).await?,
        None => twiggle_the_whiggle(1).await?,
    }

    Ok(())
}

async fn twiggle_the_whiggle(secs: u32) -> anyhow::Result<()> {

    log::info!("{secs} secs");

    let is_right = Arc::new(AtomicBool::new(false));

    let ir = is_right.clone();
    let handle = spawn(async move {
        let mut enigo = Enigo::new();
        loop {
            sleep(std::time::Duration::from_secs(secs as u64)).await;

            let is_right = ir.load(std::sync::atomic::Ordering::Acquire);

            match Mouse::get_mouse_position() {
                Mouse::Position { mut x, y } => {
                    if is_right {
                        x -= DELTA;
                        log::info!("left");
                    }
                    else {
                        x += DELTA;
                        log::info!("right");
                    }

                    enigo.mouse_move_to(x, y);

                    ir.store(!is_right, std::sync::atomic::Ordering::Relaxed);
                }
                _ => anyhow::bail!("Can't get mouse position"),
            }

            
        }
        #[allow(unreachable_code)]
        anyhow::Ok(())
    });

    let _ = handle.await?;

    Ok(())
}