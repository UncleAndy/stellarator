use std::os::raw::c_void;

#[repr(C)]
pub struct Config {
    pub states: [*mut c_void; 8],
    pub prefix: [u8; 64],
    pub prefix_count: usize,
    pub suffix: [u8; 64],
    pub suffix_count: usize,
    pub stop_at_count: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            states: [std::ptr::null_mut(); 8],
            prefix: [0; 64],
            prefix_count: 0,
            suffix: [0; 64],
            suffix_count: 0,
            stop_at_count: 1,
        }
    }
}

extern "C" {
    fn vanity_setup(vanity: *mut Config);
    fn vanity_run(vanity: *mut Config);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <prefix> <suffix> [count]", args[0]);
        return;
    }

    println!("Stelarator");
    
    let mut config = Config::default();
    
    let mut prefix = args[1].clone();
    if !prefix.starts_with('G') {
        prefix.insert(0, 'G');
    }
    let suffix = args[2].clone();
    let count = if args.len() > 3 {
        args[3].parse().unwrap_or(1)
    } else {
        1
    };

    let p_bytes = prefix.as_bytes();
    let p_len = p_bytes.len().min(63);
    config.prefix[..p_len].copy_from_slice(&p_bytes[..p_len]);
    config.prefix_count = p_len;

    let s_bytes = suffix.as_bytes();
    let s_len = s_bytes.len().min(63);
    config.suffix[..s_len].copy_from_slice(&s_bytes[..s_len]);
    config.suffix_count = s_len;

    config.stop_at_count = count;
    
    unsafe {
        println!("Starting GPU setup...");
        vanity_setup(&mut config);
        
        println!("Starting GPU run...");
        vanity_run(&mut config);
    }
}
