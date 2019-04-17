use std::io::BufRead;

pub fn read_list_file<'a, T: 'static>(
    path: &'a str
) -> Box<dyn Iterator<Item=Result<T, String>>>
    where for<'b> T: serde::Deserialize<'b>
{
    let file = std::fs::File::open(path);

    match file {
        Err(_) => Box::new(std::iter::once(Err(format!("Could not open '{}'", path)))),
        Ok(file) => {
            let buf_reader = std::io::BufReader::new(file);
            let iter = buf_reader.lines()
                .enumerate()
                .map(|(idx, result)| match result {
                    Err(_) => Err(format!("Failed reading line {}", idx + 1)),
                    Ok(line) => serde_json::from_str(&line)
                        .map_err(|_| format!("Error parsing line {}", idx + 1))
                });

            Box::new(iter)
        }
    }
}
