# osm-to-geojson
A CLI application that filters OSM PBF data files based on tags and then converts them to GeoJSON features in an NDJSON format.

# Installation
## Via Cargo
To build and install: `cargo install osm-to-geojson`

## Via Docker
To pull and run: `docker run tanadeau/osm-to-geojson:latest` and then pass options as described
below or use `--help`. Note that you will need to mount the directories with your input files and
for your output file using the `-v` option to `docker run`. See below for an example.

# Usage
```
osm-to-geojson 0.1.0
Trent Nadeau <tanadeau@gmail.com>
OSM PBF Filtering and GeoJSON converter

USAGE:
    osm-to-geojson --file <file>... --tag-filter <filter>... --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>...            Input file path
    -t, --tag-filter <filter>...    Tag filter in form tag=val1|val2|... Multiple filters are ANDed.
    -o, --output <output>           Output file path (will be NDJSON file with a GeoJSON Feature per line)
```

# Examples
For local install:
```
osm-to-geojson \
    --file europe.osm.pbf \
    --file central-america.osm.pbf \
    --tag-filter 'amenity=hospital|clinic' \
    --tag-filter 'emergency=yes' \
    --output emergency-medical.ndjson
```

For Docker execution:
```
docker run \
    -v /path/to/local/data:/opt/data \
    -v /path/to/output/dir:/opt/output \
    tanadeau/osm-to-geojson:latest \
    --file /opt/data/europe.osm.pbf \
    --file /opt/data/central-america.osm.pbf \
    --tag-filter 'amenity=hospital|clinic' \
    --tag-filter 'emergency=yes' \
    --output /opt/output/emergency-medical.ndjson
```

The above will filter the nodes and ways within the given two input files to only those that have
the `amenity` tag set to either "hospital" or "clinic" and that have the `emergency` tag set to
"yes". It will then convert those to GeoJSON features (either points for nodes or polygons for ways
and then output them into a newline-delimited JSON (NDJSON) file at the given path.
