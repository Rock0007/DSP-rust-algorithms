use super::csp::{ShiftInput, ShiftOutput, Worker};
use chrono::{DateTime, Utc, Timelike};

pub fn assign_shift(shift: ShiftInput) -> ShiftOutput {
    let shift_start: DateTime<Utc> = DateTime::parse_from_rfc3339(&shift.start)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or(Utc::now());
    let day = shift_start.format("%a").to_string();

    let new_worker = shift.workers.iter()
        .find(|w: &&Worker| { // Adjusted closure signature
            w.role == shift.role &&
            shift.required_skills.iter().all(|skill| w.skills.contains(skill)) &&
            w.current_hours < w.max_hours_per_week &&
            w.availability.get(&day).map_or(false, |avail| {
                let avail_parts: Vec<&str> = avail.split('-').collect();
                if avail_parts.len() != 2 { return false; }
                let avail_start: i32 = avail_parts[0].parse().unwrap_or(0);
                let shift_hour = shift_start.hour() as i32;
                shift_hour >= avail_start
            })
        });

    ShiftOutput {
        shift_id: shift.shift_id,
        new_worker_id: new_worker.map(|w| w.id.clone()).unwrap_or_default(),
    }
}