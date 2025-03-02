use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Timelike};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShiftInput {
    pub shift_id: String,
    pub absent_worker_id: String,
    pub start: String, // ISO datetime
    pub end: String,
    pub role: String,
    pub required_skills: Vec<String>,
    #[serde(default)] // Make workers optional, default to empty Vec
    pub workers: Vec<Worker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftOutput {
    pub shift_id: String,
    pub new_worker_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Worker {
    pub id: String,
    pub role: String,
    pub skills: Vec<String>,
    pub availability: std::collections::HashMap<String, String>,
    pub max_hours_per_week: i32,
    pub current_hours: i32,
    pub shift_preferences: Option<std::collections::HashMap<String, String>>,
    pub fatigue_threshold: Option<std::collections::HashMap<String, i32>>,
}

pub fn rebalance_shift(input: ShiftInput) -> ShiftOutput {
    let shift_start: DateTime<Utc> = DateTime::parse_from_rfc3339(&input.start)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or(Utc::now());
    let day = shift_start.format("%a").to_string();

    let mut candidates: Vec<(Worker, i32)> = input.workers.iter() // Use iter() to borrow
        .filter(|w| {
            w.role == input.role &&
            input.required_skills.iter().all(|skill| w.skills.contains(skill)) &&
            w.id != input.absent_worker_id &&
            w.current_hours < w.max_hours_per_week &&
            w.availability.get(&day).map_or(false, |avail| {
                let avail_parts: Vec<&str> = avail.split('-').collect();
                if avail_parts.len() != 2 { return false; }
                let avail_start: i32 = avail_parts[0].parse().unwrap_or(0);
                let shift_hour = shift_start.hour() as i32;
                shift_hour >= avail_start
            })
        })
        .map(|w| {
            let score = calculate_happiness(w, &input);
            (w.clone(), score)
        })
        .collect();

    candidates.sort_by(|a, b| b.1.cmp(&a.1));

    let new_worker_id = candidates.first().map(|(w, _)| w.id.clone()).unwrap_or_default();

    ShiftOutput {
        shift_id: input.shift_id,
        new_worker_id,
    }
}

fn calculate_happiness(worker: &Worker, shift: &ShiftInput) -> i32 {
    let mut score = 0;
    if let Some(prefs) = &worker.shift_preferences {
        let shift_start: DateTime<Utc> = DateTime::parse_from_rfc3339(&shift.start)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now());
        let shift_hour = shift_start.hour();
        if prefs.get("pref") == Some(&"mornings".to_string()) && shift_hour < 12 {
            score += 5;
        }
        if prefs.get("avoid") == Some(&"nights".to_string()) && shift_hour >= 18 {
            score -= 5;
        }
    }
    if let Some(fatigue) = &worker.fatigue_threshold {
        if worker.current_hours > *fatigue.get("max_hrs").unwrap_or(&0) {
            score -= 8;
        }
    }
    score
}