#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const PI: f32 = 3.14159265;
const TWO_PI: f32 = 6.2831853;

static mut RNG: u32 = 0x89ABCDEF;

fn rand_u32() -> u32 {
    unsafe {
        RNG ^= RNG << 13;
        RNG ^= RNG >> 17;
        RNG ^= RNG << 5;
        if RNG == 0 {
            RNG = 0x1337C0DE;
        }
        RNG
    }
}

fn rand_f32() -> f32 {
    (rand_u32() as f32) / 4294967295.0
}

fn clamp_f(val: f32, min: f32, max: f32) -> f32 {
    if val < min { min } else if val > max { max } else { val }
}

fn normalize_angle(mut a: f32) -> f32 {
    while a > PI { a -= TWO_PI; }
    while a < -PI { a += TWO_PI; }
    a
}

fn sin_f(a: f32) -> f32 {
    let x = normalize_angle(a);
    let x2 = x * x;
    x * (1.0 - x2 * (1.0 / 6.0 - x2 * (1.0 / 120.0 - x2 * (1.0 / 5040.0))))
}

#[allow(dead_code)]
fn cos_f(a: f32) -> f32 {
    sin_f(a + PI * 0.5)
}

fn sqrt_f(x: f32) -> f32 {
    if x <= 0.0 { return 0.0; }
    let mut guess = x;
    for _ in 0..8 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

#[allow(dead_code)]
fn atan2_f(y: f32, x: f32) -> f32 {
    if x == 0.0 {
        if y > 0.0 { PI * 0.5 } else if y < 0.0 { -PI * 0.5 } else { 0.0 }
    } else {
        let ax = if x < 0.0 { -x } else { x };
        let ay = if y < 0.0 { -y } else { y };
        let a = if ax > ay {
            let r = ay / ax;
            r * (1.0 - r * r * (1.0 / 3.0 - r * r * 0.2))
        } else {
            let r = ax / ay;
            PI * 0.5 - r * (1.0 - r * r * (1.0 / 3.0 - r * r * 0.2))
        };
        if x < 0.0 {
            if y >= 0.0 { PI - a } else { -PI + a }
        } else {
            if y >= 0.0 { a } else { -a }
        }
    }
}

#[derive(Clone, Copy)]
pub struct TuxSim {
    pub width: f32,
    pub height: f32,

    // Home Perch Position beside logo in header
    pub home_x: f32,
    pub home_y: f32,

    // Tux position & motion
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing: i32, // 1 = right, -1 = left

    // State machine:
    // 5 = SLEEPING (at home position beside logo)
    // 1 = WALKING_ACROSS_SCREEN (walks a bit for first ~32% of screen journey)
    // 2 = SWIMMING_ACROSS_SCREEN (swims across the rest of the screen to the other end)
    // 0 = RESTING (takes a rest after crossing start to end of screen)
    // 4 = HUNTING_DROPPED_FISH (swimming towards dropped fish)
    // 3 = EATING_CELEBRATION (spin roll after eating)
    // 7 = RETURNING_HOME (returning to home perch beside logo to sleep)
    pub state: i32,
    pub swim_angle: f32,
    pub spin_angle: f32,
    pub state_timer: f32,
    pub anim_timer: f32,
    pub anim_frame: i32,

    // Journey endpoints for traversing whole start to end of screen
    pub journey_start_x: f32,
    pub journey_start_y: f32,
    pub journey_target_x: f32,
    pub journey_target_y: f32,

    // Dropped fish in website
    pub fish_dropped: i32,
    pub fish_x: f32,
    pub fish_y: f32,

    // Speech
    pub speech_id: i32,
    pub speech_timer: f32,
}

static mut SIM: TuxSim = TuxSim {
    width: 1200.0,
    height: 800.0,
    home_x: 1135.0,
    home_y: 764.0,
    x: 1135.0,
    y: 764.0,
    vx: 0.0,
    vy: 0.0,
    facing: -1, // Facing left into the page from bottom-right!
    state: 5, // Default sleeping in bottom-right!
    swim_angle: 0.0,
    spin_angle: 0.0,
    state_timer: 0.0,
    anim_timer: 0.0,
    anim_frame: 4,
    journey_start_x: 45.0,
    journey_start_y: 130.0,
    journey_target_x: 1100.0,
    journey_target_y: 130.0,
    fish_dropped: 0,
    fish_x: 0.0,
    fish_y: 0.0,
    speech_id: 1,
    speech_timer: 0.0,
};

#[no_mangle]
pub extern "C" fn sim_init(width: f32, height: f32, home_x: f32, home_y: f32) {
    unsafe {
        SIM.width = width;
        SIM.height = height;
        SIM.home_x = home_x;
        SIM.home_y = home_y;
        SIM.x = home_x;
        SIM.y = home_y;
        SIM.vx = 0.0;
        SIM.vy = 0.0;
        SIM.facing = if home_x > width * 0.5 { -1 } else { 1 };
        SIM.state = 5; // Default sleeping!
        SIM.swim_angle = 0.0;
        SIM.spin_angle = 0.0;
        SIM.state_timer = 0.0;
        SIM.anim_timer = 0.0;
        SIM.anim_frame = 4;
        SIM.fish_dropped = 0;
        SIM.speech_id = 1;
        SIM.speech_timer = 0.0;
    }
}

#[no_mangle]
pub extern "C" fn set_screen_size(width: f32, height: f32) {
    unsafe {
        SIM.width = width;
        SIM.height = height;
    }
}

#[no_mangle]
pub extern "C" fn set_home_pos(hx: f32, hy: f32) {
    unsafe {
        SIM.home_x = hx;
        SIM.home_y = hy;
        if SIM.state == 5 {
            SIM.x = hx;
            SIM.y = hy;
        }
    }
}

// Dynamic speed multiplier based on screen width:
// - Desktops / laptops (>= 1100px): 1.0 (full fast speeds user loved)
// - Small screens / phones (<= 360px): 0.62 (smooth, visible, not rocketing across phone)
// - Mid-size / tablets: smoothly scaled in between
fn get_screen_speed_multiplier() -> f32 {
    unsafe {
        if SIM.width >= 1100.0 {
            1.0
        } else if SIM.width <= 360.0 {
            0.62
        } else {
            0.62 + ((SIM.width - 360.0) / (1100.0 - 360.0)) * 0.38
        }
    }
}

// Starts a new journey traversing the WHOLE screen from start to end!
fn start_cross_screen_journey() {
    unsafe {
        let is_small = SIM.width < 600.0;
        let min_x = if is_small { 26.0 } else { 45.0 };
        let max_x = if SIM.width > 120.0 {
            if is_small { SIM.width - 28.0 } else { SIM.width - 55.0 }
        } else {
            90.0
        };

        // Determine destination: cover whole start to end of screen!
        if SIM.x < SIM.width * 0.5 {
            SIM.journey_target_x = max_x;
            SIM.facing = 1;
        } else {
            SIM.journey_target_x = min_x;
            SIM.facing = -1;
        }

        // Random Y level across the website (comfortably below header)
        let min_y = if SIM.height < 600.0 { 90.0 } else { 125.0 };
        let max_y = if SIM.height > 200.0 {
            let limit = SIM.height - (if SIM.height < 600.0 { 55.0 } else { 85.0 });
            if limit > 440.0 { 440.0 } else { limit }
        } else {
            140.0
        };
        SIM.journey_target_y = min_y + rand_f32() * (max_y - min_y);

        // If currently sleeping at home beside logo (header level), move down to website walking line
        if SIM.y < min_y {
            SIM.y = min_y;
        }

        SIM.journey_start_x = SIM.x;
        SIM.journey_start_y = SIM.y;

        // Step 1: Walk a bit!
        SIM.state = 1; // WALKING_ACROSS_SCREEN
        if rand_f32() < 0.40 && SIM.speech_timer <= 0.0 {
            SIM.speech_id = 50 + (rand_u32() % 10) as i32; // Funny roaming dialogue (50..59)
            SIM.speech_timer = 2.6;
        }
    }
}

// User clicks on Tux:
// If sleeping, wake up and start journey across the screen (walk a bit, swim a bit, rest)!
#[no_mangle]
pub extern "C" fn click_tux() {
    unsafe {
        if SIM.state == 5 {
            start_cross_screen_journey();
            SIM.speech_id = 10 + (rand_u32() % 10) as i32; // Funny wake-up dialogue (10..19)
            SIM.speech_timer = 2.8;
        } else if SIM.state == 1 || SIM.state == 2 || SIM.state == 0 {
            // Already awake: do a playful barrel roll!
            SIM.state = 3;
            SIM.state_timer = 0.8;
            SIM.spin_angle = 0.0;
            SIM.speech_id = (rand_u32() % 10) as i32; // Funny barrel-roll dialogue (0..9)
            SIM.speech_timer = 2.5;
        }
    }
}

// User drops the fish outside the header onto the website!
#[no_mangle]
pub extern "C" fn drop_fish(x: f32, y: f32) {
    unsafe {
        SIM.fish_dropped = 1;
        SIM.fish_x = x;
        SIM.fish_y = y;

        // Tux immediately breaks out of roaming and hunts/chases the dropped fish!
        SIM.state = 4; // HUNTING_DROPPED_FISH
        SIM.speech_id = 20 + (rand_u32() % 10) as i32; // Funny chase/relocate dialogue (20..29)
        SIM.speech_timer = 2.6;
    }
}

// User toggles off fish button or cancels dropped fish
#[no_mangle]
pub extern "C" fn despawn_fish() {
    unsafe {
        SIM.fish_dropped = 0;
        if SIM.state == 4 {
            start_cross_screen_journey();
        }
    }
}

#[no_mangle]
pub extern "C" fn sim_tick(dt: f32) -> i32 {
    unsafe {
        let delta = if dt > 0.05 { 0.05 } else if dt < 0.001 { 0.016 } else { dt };
        let mult = get_screen_speed_multiplier();
        let margin_x = if SIM.width < 600.0 { 24.0 } else { 35.0 };
        let margin_y = if SIM.height < 600.0 { 28.0 } else { 35.0 };
        let min_y = if SIM.height < 600.0 { 50.0 } else { 58.0 };

        if SIM.speech_timer > 0.0 {
            SIM.speech_timer -= delta;
        }

        SIM.anim_timer += delta;

        match SIM.state {
            5 => { // SLEEPING (at home perch beside logo)
                SIM.vx = 0.0;
                SIM.vy = 0.0;
                SIM.x = SIM.home_x;
                SIM.y = SIM.home_y;
                SIM.anim_frame = 4;
            }
            1 => { // 1. WALK A BIT: Travels first ~32% of the screen trip on foot
                let dx = SIM.journey_target_x - SIM.x;
                let dy = SIM.journey_target_y - SIM.y;
                let dist_remaining = sqrt_f(dx * dx + dy * dy);
                let total_dist = sqrt_f(
                    (SIM.journey_target_x - SIM.journey_start_x) * (SIM.journey_target_x - SIM.journey_start_x) +
                    (SIM.journey_target_y - SIM.journey_start_y) * (SIM.journey_target_y - SIM.journey_start_y)
                );

                // If walked ~32% of total screen journey, transition to SWIMMING!
                let walked_dist = total_dist - dist_remaining;
                if walked_dist > total_dist * 0.32 || dist_remaining < 60.0 {
                    SIM.state = 2; // SWIMMING_ACROSS_SCREEN
                } else {
                    let walk_speed = 95.0 * (0.8 + 0.2 * mult); // Responsive waddle pace
                    SIM.facing = if dx >= 0.0 { 1 } else { -1 };
                    SIM.vx = (SIM.facing as f32) * walk_speed;
                    SIM.vy = (dy / (dist_remaining + 0.1)) * 20.0;

                    SIM.x += SIM.vx * delta;
                    SIM.y += SIM.vy * delta;

                    SIM.anim_frame = ((SIM.anim_timer * 8.0) as i32) % 4;
                    SIM.x = clamp_f(SIM.x, margin_x, SIM.width - margin_x);
                    SIM.y = clamp_f(SIM.y, min_y, SIM.height - margin_y);
                }
            }
            2 => { // 2. BIT OF SWIMMING: Swims across the rest of the screen to the end
                let dx = SIM.journey_target_x - SIM.x;
                let dy = SIM.journey_target_y - SIM.y;
                let dist = sqrt_f(dx * dx + dy * dy);

                if dist > 20.0 {
                    let speed = 400.0 * mult; // Fast streamlined swim speed (scales cleanly on smaller screens)
                    let inv_dist = 1.0 / dist;
                    let dir_x = dx * inv_dist;
                    let dir_y = dy * inv_dist;

                    // Lateral wave normal for smooth natural S-curve glide
                    let norm_x = -dir_y;
                    let norm_y = dir_x;

                    // Attenuate weave when approaching destination
                    let weave_amp = if dist < 120.0 { 22.0 * (dist / 120.0) } else { 22.0 };
                    let weave = sin_f(SIM.anim_timer * 4.2) * weave_amp;

                    SIM.vx = dir_x * speed + norm_x * weave;
                    SIM.vy = dir_y * speed + norm_y * weave;

                    SIM.x += SIM.vx * delta;
                    SIM.y += SIM.vy * delta;

                    // Stable facing towards target with deadband
                    if dx > 15.0 {
                        SIM.facing = 1;
                    } else if dx < -15.0 {
                        SIM.facing = -1;
                    }

                    SIM.anim_frame = ((SIM.anim_timer * 15.0) as i32) % 4;
                    SIM.x = clamp_f(SIM.x, margin_x, SIM.width - margin_x);
                    SIM.y = clamp_f(SIM.y, min_y, SIM.height - margin_y);
                } else {
                    // Reached the destination end of the screen: 3. TAKE REST!
                    SIM.state = 0; // RESTING
                    SIM.state_timer = 3.5; // Rest for 3.5 seconds
                    SIM.vx = 0.0;
                    SIM.vy = 0.0;
                    if rand_f32() < 0.80 {
                        SIM.speech_id = 40 + (rand_u32() % 10) as i32; // Funny resting dialogue (40..49)
                        SIM.speech_timer = 2.8;
                    }
                }
            }
            0 => { // 3. TAKE REST: Resting at destination, standing/looking around
                SIM.state_timer -= delta;
                let cycle = SIM.anim_timer - ((SIM.anim_timer / 2.5) as i32 as f32) * 2.5;
                SIM.anim_frame = if cycle > 2.0 { 1 } else { 0 }; // Periodic blinks while resting

                if SIM.state_timer <= 0.0 {
                    // Rest finished! Start new cross-screen journey to the other end!
                    start_cross_screen_journey();
                }
            }
            4 => { // HUNTING_DROPPED_FISH (Non-linear path to dropped fish)
                let dx = SIM.fish_x - SIM.x;
                let dy = SIM.fish_y - SIM.y;
                let dist = sqrt_f(dx * dx + dy * dy);
                let catch_dist = if SIM.width < 600.0 { 18.0 } else { 22.0 };

                if dist > catch_dist {
                    let speed = 495.0 * mult; // High-speed dynamic fish hunt (responsive for screen width)
                    let inv_dist = 1.0 / dist;
                    let dir_x = dx * inv_dist;
                    let dir_y = dy * inv_dist;

                    let norm_x = -dir_y;
                    let norm_y = dir_x;

                    // Smooth wave that dampens to 0 as Tux closes in on the fish
                    let weave_amp = if dist < 140.0 { 26.0 * (dist / 140.0) } else { 26.0 };
                    let weave = sin_f(SIM.anim_timer * 4.5) * weave_amp;

                    SIM.vx = dir_x * speed + norm_x * weave;
                    SIM.vy = dir_y * speed + norm_y * weave;

                    SIM.x += SIM.vx * delta;
                    SIM.y += SIM.vy * delta;

                    // Stable facing towards fish
                    if dx > 12.0 {
                        SIM.facing = 1;
                    } else if dx < -12.0 {
                        SIM.facing = -1;
                    }

                    SIM.anim_frame = ((SIM.anim_timer * 17.0) as i32) % 4;
                    SIM.x = clamp_f(SIM.x, margin_x, SIM.width - margin_x);
                    SIM.y = clamp_f(SIM.y, min_y, SIM.height - margin_y);
                } else {
                    // Eat the dropped fish!
                    SIM.fish_dropped = 0;
                    SIM.state = 3; // Celebration loop!
                    SIM.state_timer = 1.3;
                    SIM.spin_angle = 0.0;
                    SIM.speech_id = 30 + (rand_u32() % 10) as i32; // Funny eating celebration dialogue (30..39)
                    SIM.speech_timer = 3.2;
                }
            }
            3 => { // EATING_CELEBRATION (Spin roll)
                SIM.state_timer -= delta;
                SIM.spin_angle += delta * 7.5;
                SIM.vx *= 0.82;
                SIM.vy *= 0.82;
                SIM.x += SIM.vx * delta;
                SIM.y += SIM.vy * delta;

                if SIM.state_timer <= 0.0 {
                    // On feeding upon a fish it moves back to its original place and goes back to sleep!
                    SIM.state = 7; // RETURNING_HOME
                    SIM.speech_id = 60 + (rand_u32() % 5) as i32; // Funny returning home dialogue (60..64)
                    SIM.speech_timer = 2.5;
                }
            }
            7 => { // RETURNING_HOME (Glides back to original place beside logo)
                let dx = SIM.home_x - SIM.x;
                let dy = SIM.home_y - SIM.y;
                let dist = sqrt_f(dx * dx + dy * dy);
                let home_dist = if SIM.width < 600.0 { 10.0 } else { 14.0 };

                if dist > home_dist {
                    let speed = 345.0 * mult; // Fast glide back to home perch
                    let inv_dist = 1.0 / dist;
                    let dir_x = dx * inv_dist;
                    let dir_y = dy * inv_dist;

                    let norm_x = -dir_y;
                    let norm_y = dir_x;

                    let weave_amp = if dist < 100.0 { 20.0 * (dist / 100.0) } else { 20.0 };
                    let weave = sin_f(SIM.anim_timer * 3.8) * weave_amp;

                    SIM.vx = dir_x * speed + norm_x * weave;
                    SIM.vy = dir_y * speed + norm_y * weave;

                    SIM.x += SIM.vx * delta;
                    SIM.y += SIM.vy * delta;

                    if dx > 12.0 {
                        SIM.facing = 1;
                    } else if dx < -12.0 {
                        SIM.facing = -1;
                    }

                    SIM.anim_frame = ((SIM.anim_timer * 10.0) as i32) % 4;
                } else {
                    // Reached home: go back to sleep!
                    SIM.x = SIM.home_x;
                    SIM.y = SIM.home_y;
                    SIM.vx = 0.0;
                    SIM.vy = 0.0;
                    SIM.facing = if SIM.home_x > SIM.width * 0.5 { -1 } else { 1 };
                    SIM.state = 5; // SLEEP
                    SIM.anim_frame = 4;
                }
            }
            _ => {}
        }

        SIM.state
    }
}

#[no_mangle] pub extern "C" fn get_x() -> f32 { unsafe { SIM.x } }
#[no_mangle] pub extern "C" fn get_y() -> f32 { unsafe { SIM.y } }
#[no_mangle] pub extern "C" fn get_vx() -> f32 { unsafe { SIM.vx } }
#[no_mangle] pub extern "C" fn get_vy() -> f32 { unsafe { SIM.vy } }
#[no_mangle] pub extern "C" fn get_facing() -> i32 { unsafe { SIM.facing } }
#[no_mangle] pub extern "C" fn get_state() -> i32 { unsafe { SIM.state } }
#[no_mangle] pub extern "C" fn get_anim_frame() -> i32 { unsafe { SIM.anim_frame } }
#[no_mangle] pub extern "C" fn get_swim_angle() -> f32 { unsafe { SIM.swim_angle } }
#[no_mangle] pub extern "C" fn get_spin_angle() -> f32 { unsafe { SIM.spin_angle } }
#[no_mangle] pub extern "C" fn get_fish_dropped() -> i32 { unsafe { SIM.fish_dropped } }
#[no_mangle] pub extern "C" fn get_fish_x() -> f32 { unsafe { SIM.fish_x } }
#[no_mangle] pub extern "C" fn get_fish_y() -> f32 { unsafe { SIM.fish_y } }
#[no_mangle] pub extern "C" fn get_speech_id() -> i32 { unsafe { SIM.speech_id } }
#[no_mangle] pub extern "C" fn get_speech_timer() -> f32 { unsafe { SIM.speech_timer } }
