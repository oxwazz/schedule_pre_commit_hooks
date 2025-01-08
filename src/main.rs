use chrono::prelude::*;
use chrono::Duration;
use clap::Parser;
use std::collections::HashMap;
use std::process::{Command, Stdio};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    timezone: Option<String>,
    #[arg(long)]
    schedule: Option<Vec<String>>,
}

fn main() {
    std::process::exit(match schedule_commit() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

#[derive(Debug, Clone)]
struct ScheduleTime {
    start_time: NaiveTime,
    end_time: NaiveTime,
}

fn schedule_commit() -> Result<(), String> {
    let args = Args::parse();

    let timezone_reference = get_timezone_reference(&args)?;
    let allowed_schedule_references = get_allowed_schedule_references(&args)?;

    let current_commit_date = Local::now().with_timezone(&timezone_reference);
    let latest_commit_date = get_latest_commit_date(&timezone_reference)?;

    let local_timezone = Local::now().fixed_offset().timezone();
    let limit_date = Local::now().with_timezone(&timezone_reference) + Duration::days(33);
    let mut temp_date = Local::now().with_timezone(&timezone_reference);
    let mut temp: Option<DateTime<FixedOffset>> = None;

    'outer: while temp_date <= limit_date {
        if let Some(item) = allowed_schedule_references.get(&temp_date.weekday()) {
            for time in item.iter() {
                let start_date = temp_date.with_time(time.start_time).unwrap();
                let end_date = temp_date.with_time(time.end_time).unwrap();

                if latest_commit_date.gt(&end_date) {
                    continue;
                }

                if current_commit_date >= start_date && current_commit_date <= end_date {
                    temp = Some(current_commit_date.with_timezone(&local_timezone));
                    break 'outer;
                } else {
                    let add_one_min = if latest_commit_date < start_date {
                        start_date
                    } else {
                        latest_commit_date + Duration::minutes(1)
                    };
                    if add_one_min.time() >= time.start_time && add_one_min.time() <= time.end_time
                    {
                        if temp.is_some() && add_one_min < temp.unwrap() {
                            temp = Some(add_one_min);
                        } else if temp.is_none() {
                            temp = Some(add_one_min);
                        }
                    }
                }
            }
        }
        if let Some(date) = temp {
            amend_commit_date(&date.with_timezone(&local_timezone))?;
            break 'outer;
        }
        temp_date += Duration::days(1);
    }

    match temp {
        None => Err("Failed to schedule commit".to_string()),
        Some(_) => Ok(()),
    }
}

fn get_timezone_reference(args: &Args) -> Result<FixedOffset, String> {
    let tz_str = match &args.timezone {
        Some(tz) => tz,
        None => {
            return Err("[OXWAZZ-ERR] Timezone is required".to_string());
        }
    };
    let tz = DateTime::parse_from_str(
        format!("1970-01-01 00:00:00 {}", tz_str).as_str(),
        DATE_FORMAT,
    )
    .map_err(|err| format!("[OXWAZZ-ERR] Failed get_timezone_reference: {}", err))?
    .timezone();
    Ok(tz)
}

fn get_allowed_schedule_references(
    args: &Args,
) -> Result<HashMap<Weekday, Vec<ScheduleTime>>, String> {
    let mut allowed_dates: HashMap<Weekday, Vec<ScheduleTime>> = HashMap::new();
    if let Some(schedule) = &args.schedule {
        for s in schedule {
            let s = s.split("|").collect::<Vec<&str>>();
            let day = match s[0] {
                "monday" => Weekday::Mon,
                "tuesday" => Weekday::Tue,
                "wednesday" => Weekday::Wed,
                "thursday" => Weekday::Thu,
                "friday" => Weekday::Fri,
                "saturday" => Weekday::Sat,
                "sunday" => Weekday::Sun,
                _ => {
                    return Err("[OXWAZZ-ERR] Invalid day".to_string());
                }
            };

            let times = s[1].split(",").collect::<Vec<&str>>();
            let mut schedule_times = Vec::new();
            for time in times {
                let time = time.split("-").collect::<Vec<&str>>();
                let start_time = NaiveTime::parse_from_str(time[0], "%H:%M").unwrap();
                let end_time = NaiveTime::parse_from_str(time[1], "%H:%M").unwrap();
                schedule_times.push(ScheduleTime {
                    start_time,
                    end_time,
                });
            }
            allowed_dates
                .entry(day)
                .and_modify(|e| e.extend(schedule_times.clone()))
                .or_insert(schedule_times);
        }
    }
    for (_, schedule_times) in &mut allowed_dates {
        schedule_times.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    }
    Ok(allowed_dates)
}

fn get_latest_commit_date(timezone: &FixedOffset) -> Result<DateTime<FixedOffset>, String> {
    match Command::new("git")
        .args(["--no-pager", "log", "-1", "HEAD~1", "--format=%ai"])
        .output()
    {
        Ok(date) => {
            let date_parsed = String::from_utf8_lossy(&date.stdout).replace("\n", "");
            let date_parsed = DateTime::parse_from_str(&date_parsed, DATE_FORMAT)
                .map_err(|err| format!("[OXWAZZ-ERR] Failed to parse date: {}", err))?;
            Ok(date_parsed.with_timezone(timezone))
        }

        Err(err) => Err(format!(
            "[OXWAZZ-ERR] Failed to execute git log command: {}",
            err
        )),
    }
}

fn amend_commit_date(date: &DateTime<FixedOffset>) -> Result<(), String> {
    let date_str = date.format(DATE_FORMAT).to_string();

    let mut command = Command::new("git");
    let mut env_vars = HashMap::new();
    env_vars.insert("SKIP", "schedule");
    env_vars.insert("GIT_COMMITTER_DATE", &date_str);
    env_vars.insert("GIT_AUTHOR_DATE", &date_str);
    command
        .envs(&env_vars)
        .args([
            "commit",
            "--amend",
            "--no-edit",
            "--date",
            &date_str,
            "--no-verify",
        ])
        .stdin(Stdio::null())
        .output()
        .expect("Failed to execute git commit command");
    Ok(())
}
