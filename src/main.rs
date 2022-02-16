use chrono::Datelike;
use clap::Parser;
use memory_lol::db::table::ReadOnly;
use simplelog::LevelFilter;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use std::iter::Sum;
use std::path::Path;
use tera::{Context, Tera};

mod chart;
mod error;
mod item;
mod model;

use error::Error;

const TOP_SUSPENSIONS_COUNT: usize = 20;
const UNTRACKED_SUSPENSIONS_FOLLOWERS_COUNT_LIMIT: usize = 1000;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    init_logging(opts.verbose)?;

    let suspensions = model::SuspensionRecord::read_file(opts.suspensions)?;
    let screen_names = model::ScreenNameRecord::read_file(opts.screen_names)?;
    let flagged = model::read_flagged_file(opts.flagged)?;
    let db = memory_lol::db::Database::<ReadOnly>::open(opts.memory_lol_db)?;

    let mut seen_dates = suspensions.keys().collect::<Vec<_>>();
    seen_dates.extend(screen_names.keys());
    seen_dates.sort();
    seen_dates.dedup();

    let mut stats = item::Stats::default();

    let mut date_stats = HashMap::new();
    let mut top_suspensions = vec![];
    let mut all_suspensions = vec![];

    let mut tera = Tera::new("src/templates/*")?;
    tera.autoescape_on(vec![]);

    let report_base_dir = Path::new("reports");

    for date in seen_dates {
        log::info!("Processing date: {}", date.naive_utc());
        if let Some((records, unknown_count)) = suspensions.get(date) {
            let mut tracked_suspensions = vec![];
            let mut tracked_screen_name_changes = vec![];
            let mut untracked_suspensions = vec![];
            let mut untracked_suspensions_count = 0;

            for record in records {
                let ranking = flagged.get(&record.user_id);

                if ranking.is_some()
                    || record.followers_count >= UNTRACKED_SUSPENSIONS_FOLLOWERS_COUNT_LIMIT
                {
                    let mut other_screen_names = db
                        .lookup_by_user_id(record.user_id)?
                        .keys()
                        .map(|screen_name| screen_name.to_lowercase())
                        .collect::<HashSet<_>>();
                    other_screen_names.remove(&record.screen_name.to_lowercase());

                    let image_url =
                        make_profile_image_thumbnail_url(&record.profile_image_url, "thumbnails");

                    let item = item::SuspensionItem {
                        record,
                        image_url,
                        ranking: ranking.copied(),
                        other_screen_name_count: other_screen_names.len(),
                    };

                    if ranking.is_some() {
                        tracked_suspensions.push(item.clone());

                        if record.reversal.is_none() {
                            top_suspensions.push((item, date.naive_utc()));
                        }
                    } else {
                        untracked_suspensions.push(item);
                    }
                }

                if ranking.is_none() {
                    untracked_suspensions_count += 1;
                }

                stats.total_suspensions_count += 1;

                if record.reversal.is_none() {
                    all_suspensions.push((record, date));
                } else {
                    stats.total_reversals_count += 1;
                }

                if record.verified {
                    stats.total_verified_suspended_count += 1;
                }

                if record.protected {
                    stats.total_protected_suspended_count += 1;
                }

                if !record.withheld_in_countries.is_empty() {
                    stats.total_withheld_suspended_count += 1;
                }
            }

            let mut total_screen_name_changes_count = 0;

            if let Some(screen_name_records) = screen_names.get(date) {
                total_screen_name_changes_count += screen_name_records.len();

                for record in screen_name_records {
                    if let Some(ranking) = flagged.get(&record.user_id) {
                        let mut other_screen_names = db
                            .lookup_by_user_id(record.user_id)?
                            .keys()
                            .map(|screen_name| screen_name.to_lowercase())
                            .collect::<HashSet<_>>();
                        other_screen_names.remove(&record.new_screen_name.to_lowercase());
                        other_screen_names.remove(&record.previous_screen_name.to_lowercase());

                        let image_url = make_profile_image_thumbnail_url(
                            &record.profile_image_url,
                            "thumbnails",
                        );

                        tracked_screen_name_changes.push(item::ScreenNameItem {
                            record,
                            image_url,
                            ranking: Some(*ranking),
                            other_screen_name_count: other_screen_names.len(),
                        })
                    }
                }
            }

            stats.total_screen_name_changes_count += total_screen_name_changes_count;

            tracked_suspensions.sort();
            tracked_screen_name_changes.sort();
            untracked_suspensions.sort();

            let selected_untracked_suspensions = untracked_suspensions
                .into_iter()
                .take_while(|item| {
                    item.record.followers_count >= UNTRACKED_SUSPENSIONS_FOLLOWERS_COUNT_LIMIT
                })
                .collect::<Vec<_>>();

            date_stats.insert(
                date.naive_utc(),
                item::DateStats {
                    date: date.naive_utc(),
                    total_suspensions_count: records.len() + unknown_count,
                    total_screen_name_changes_count,
                    tracked_suspensions_count: tracked_suspensions.len(),
                    tracked_screen_name_changes_count: tracked_screen_name_changes.len(),
                },
            );

            let mut context = Context::new();
            context.insert("date", &date.naive_utc());
            context.insert("tracked_suspensions", &tracked_suspensions);
            context.insert("tracked_screen_name_changes", &tracked_screen_name_changes);
            context.insert("untracked_suspensions_count", &untracked_suspensions_count);
            context.insert(
                "untracked_suspensions_limit",
                &UNTRACKED_SUSPENSIONS_FOLLOWERS_COUNT_LIMIT,
            );
            context.insert("untracked_suspensions", &selected_untracked_suspensions);

            let date_dir = report_base_dir.join(format!("{}", date.format("%Y-%m-%d")));
            std::fs::create_dir_all(&date_dir)?;

            let date_file = File::create(date_dir.join("README.md"))?;
            let writer = BufWriter::new(date_file);

            tera.render_to("date.md", &context, writer)?;
        }
    }

    let mut date_stats_vec = date_stats.values().collect::<Vec<_>>();
    date_stats_vec.sort();
    date_stats_vec.reverse();

    top_suspensions.sort();
    let selected_top_suspensions = top_suspensions
        .into_iter()
        .take(TOP_SUSPENSIONS_COUNT)
        .collect::<Vec<_>>();

    stats.other.push((
        "Followers".to_string(),
        compute_stats(all_suspensions.iter(), |(record, _)| {
            record.followers_count as u32
        }),
    ));
    stats.other.push((
        "Age of account (days)".to_string(),
        compute_stats(all_suspensions.iter(), |(record, date)| {
            (**date - record.created_at.date()).num_days() as u32
        }),
    ));

    let mut context = Context::new();
    context.insert("top_suspensions", &selected_top_suspensions);
    context.insert("stats", &stats);
    context.insert("date_stats", &date_stats_vec);
    context.insert(
        "untracked_suspensions_limit",
        &UNTRACKED_SUSPENSIONS_FOLLOWERS_COUNT_LIMIT,
    );
    let writer = BufWriter::new(File::create("README.md")?);
    tera.render_to("index.md", &context, writer)?;

    let mut suspensions_chart = chart::Chart {
        name: "Total daily suspensions".to_string(),
        points: vec![],
        smooth: false,
        colour: "#e74c3c".to_string(),
        y_granularity: 500,
    };

    let mut screen_name_changes_chart = chart::Chart {
        name: "Total daily screen name changes".to_string(),
        points: vec![],
        smooth: false,
        colour: "#3498db".to_string(),
        y_granularity: 500,
    };

    let date_labels = date_stats
        .keys()
        .min()
        .zip(date_stats.keys().max())
        .map(|(first_date, last_date)| {
            std::iter::successors(Some(*first_date), |date| Some(date.succ()))
                .take_while(|date| date <= last_date)
                .filter_map(|date| {
                    let date_id = (date - *first_date).num_days();
                    let suspensions_count = date_stats
                        .get(&date)
                        .map(|item| item.total_suspensions_count)
                        .unwrap_or_default();
                    suspensions_chart.add_point(date_id as usize, suspensions_count);

                    let screen_name_changes_count = date_stats
                        .get(&date)
                        .map(|item| item.total_screen_name_changes_count)
                        .unwrap_or_default();
                    screen_name_changes_chart
                        .add_point(date_id as usize, screen_name_changes_count);

                    if date.day() == 1 {
                        Some((date_id as usize, format!("{}", date.format("%b %Y"))))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let suspensions_chart_context = suspensions_chart.draw_svg(1024, 256, &date_labels);
    let suspensions_chart_writer = BufWriter::new(File::create("charts/suspensions.svg")?);
    tera.render_to(
        "chart.svg",
        &suspensions_chart_context,
        suspensions_chart_writer,
    )?;

    let screen_name_changes_chart_context =
        screen_name_changes_chart.draw_svg(1024, 256, &date_labels);
    let screen_name_changes_chart_writer = BufWriter::new(File::create("charts/screen-names.svg")?);
    tera.render_to(
        "chart.svg",
        &screen_name_changes_chart_context,
        screen_name_changes_chart_writer,
    )?;

    Ok(())
}

fn compute_median<V: Into<f64> + Clone>(values: &[V]) -> f64 {
    let half = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[half - 1].clone().into() + values[half].clone().into()) / 2.0
    } else {
        values[half].clone().into()
    }
}

fn compute_stats<I: Iterator, V: Into<f64> + Sum + Ord + Clone, F: Fn(I::Item) -> V>(
    items: I,
    f: F,
) -> ((f64, f64, f64), f64) {
    let mut values = items.map(f).collect::<Vec<_>>();
    values.sort();

    let total: V = values.iter().cloned().sum();
    let mean = total.into() / values.len() as f64;
    let median = compute_median(&values);
    let half = values.len() / 2;
    let p25 = compute_median(&values[0..half]);
    let p75 = compute_median(&values[half..]);

    ((p25, median, p75), mean)
}

fn make_profile_image_thumbnail_url<P: AsRef<Path>>(profile_image_url: &str, base: P) -> String {
    let re =
        regex::Regex::new(r"^https?://([^/]+)/profile_images/(\d+)/(.*)_normal(\.[a-zA-Z0-9-]+)?$")
            .unwrap();

    re.captures(profile_image_url)
        .and_then(|captures| {
            let ((id, name), extension) = captures
                .get(2)
                .map(|m| m.as_str())
                .zip(captures.get(3).map(|m| m.as_str()))
                .zip(captures.get(4).map(|m| m.as_str()))?;

            let path = format!("./thumbnails/{}-{}_400x400{}", id, name, extension);

            if base.as_ref().join(&path).exists() {
                Some(path)
            } else {
                None
            }
        })
        .unwrap_or_else(|| profile_image_url.to_string())
}

#[derive(Debug, Parser)]
#[clap(name = "daily", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// Suspensions data file path
    #[clap(long, default_value = "data/suspensions.csv")]
    suspensions: String,
    /// Screen name changes data file path
    #[clap(long, default_value = "data/screen-names.csv")]
    screen_names: String,
    /// Flagged rankings file path
    #[clap(long)]
    flagged: String,
    /// memory.lol database path
    #[clap(long)]
    memory_lol_db: String,
}

fn select_log_level_filter(verbosity: i32) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

/// Initialize a default terminal logger with the indicated log level.
pub fn init_logging(verbosity: i32) -> Result<(), log::SetLoggerError> {
    simplelog::TermLogger::init(
        select_log_level_filter(verbosity),
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
}
