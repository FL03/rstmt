/*
    Appellation: motion <example>
    Contrib: @FL03
*/
use rstmt::Octave;
use rstmt::nrt::motion::MotionPlanner;
use rstmt::nrt::{HyperTonnetz, Triad, TriadError};

fn main() -> Result<(), TriadError> {
    // Set up tracing
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_file(false)
        .with_line_number(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .init();

    tracing::info!("Motion Planning Example");

    // Create a new tonnetz
    let mut tonnetz = HyperTonnetz::new();

    // Add pitch classes (0-11)
    let _ = tonnetz.scaffold_layer(Octave(4))?;

    let c_major = Triad::major(0); // C Major (0,4,7)
    // Add some common triads to the Tonnetz
    let c_major_idx = tonnetz.add_triad(c_major)?; // C Major (0,4,7)

    // Create a motion planner
    let mut planner = MotionPlanner::new(&tonnetz).with_max_depth(4);

    // Find paths from C Major to triads containing C# (1)
    let target_note = 1; // C#
    let paths = planner.find_paths_to_pitch(c_major_idx, target_note);

    tracing::info! {
        "Found {} paths from C Major to triads containing C# (1):",
        paths.len()
    }
    tracing::info_span!("paths", count = paths.len()).in_scope(|| {
        for (i, path) in paths.iter().enumerate() {
            tracing::info!("Path {}", i + 1);
            tracing::info!("Transformations: {:?}", path.transforms());
            tracing::info!("Resulting triads:");

            for (j, triad) in path.triads().iter().enumerate() {
                let edge_str = match path.get_edge(j) {
                    Some(id) => format!("Edge {}", id),
                    None => "Virtual".to_string(),
                };

                tracing::info!(
                    "\t\t{}: {} {:?} {}",
                    j,
                    triad.class(),
                    triad.chord(),
                    edge_str
                );
            }

            // Show the final triad that contains the target note
            let final_triad = path.triads().last().unwrap();
            tracing::info!(
                "\t\tFinal triad: {} {:?}",
                final_triad.class(),
                final_triad.chord()
            );
            tracing::info!(
                "\t\tTarget note {} is in final triad: {}",
                target_note,
                final_triad.contains(&target_note)
            );
        }
    });

    Ok(())
}
