use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, EventType, InputEvent, KeyCode};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define which keys your virtual device is capable of pressing
    let mut keys = AttributeSet::<KeyCode>::new();
    keys.insert(KeyCode::KEY_A);

    // 2. Build the virtual device (requires root or uinput group privileges)
    let mut device = VirtualDevice::builder()?
        .name("High-Performance-Virtual-Keyboard")
        .with_keys(&keys)?
        .build()?;

    // Allow the OS a brief moment to register the new virtual input device
    thread::sleep(Duration::from_millis(100));

    // 3. Simulate Key Press (KeyDown)
    let down_event = InputEvent::new(EventType::KEY.0, KeyCode::KEY_A.0, 1);
    device.emit(&[down_event])?;

    thread::sleep(Duration::from_millis(1000));

    // 4. Simulate Key Release (KeyUp)
    let up_event = InputEvent::new(EventType::KEY.0, KeyCode::KEY_A.0, 0);
    device.emit(&[up_event])?;

    println!("Keypress dispatched via kernel space.");
    Ok(())
}
