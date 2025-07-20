use ash::{Entry, vk};

fn main() {
    // linked() is available because we enabled the "linked" feature
    let entry = Entry::linked();

    let instance = unsafe {
        entry.create_instance(
            &vk::InstanceCreateInfo::default(),
            None,
        )
    }.expect("Failed to create Vulkan instance");

    println!("Vulkan instance created!");
}
