use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use geojson::{feature, Feature, GeoJson, Geometry, Value};
use osmpbfreader::{Node, NodeId, OsmObj, OsmPbfReader, Tags, TagsImpl, Way};
use serde_json::{to_value, Map};
use structopt::StructOpt;

type TagFilters = HashMap<String, HashSet<String>>;
type NodeToCoords = HashMap<NodeId, Vec<f64>>;

fn main() {
    let args = AppArgs::from_args();
    let tag_filters: TagFilters = args.parse_tag_filters();

    let mut output_file =
        BufWriter::new(File::create(&args.output).expect("Could not open output file"));

    println!("Output: {}", args.output.to_string_lossy());

    for file_path in &args.files {
        println!("Processing {}", file_path.to_string_lossy());

        let input_file = BufReader::new(File::open(file_path).expect("Could not open input file"));
        let mut pbf = OsmPbfReader::new(input_file);

        let osm_objs = pbf
            .get_objs_and_deps(|obj| {
                !obj.is_relation() && matches_tag_filters(&tag_filters, obj.tags())
            })
            .expect("Could not extract OSM data");

        let mut dep_nodes: NodeToCoords = HashMap::new();
        let mut count = 0usize;
        for (_, obj) in osm_objs {
            match obj {
                OsmObj::Node(node) => match node_to_feature(&node, &tag_filters) {
                    Some(feature) => {
                        write_geojson(&mut output_file, &feature);
                        count += 1;
                    }
                    None => {
                        dep_nodes.insert(node.id, node_to_point(&node));
                    }
                },
                OsmObj::Way(way) => {
                    if let Some(feature) = way_to_feature(way, &dep_nodes) {
                        write_geojson(&mut output_file, &feature);
                        count += 1;
                    }
                }
                OsmObj::Relation(_) => panic!("Found relation when excluded"),
            }
        }

        println!("Found {} matching features", count);
    }
}

#[derive(StructOpt, Debug)]
struct AppArgs {
    #[structopt(
        short = "f",
        long = "file",
        name = "file",
        parse(from_os_str),
        required = true
    )]
    /// Input file path
    files: Vec<PathBuf>,
    #[structopt(short = "o", long, parse(from_os_str))]
    /// Output file path (will be NDJSON file with a GeoJSON Feature per line)
    output: PathBuf,
    #[structopt(short = "t", long = "tag-filter", name = "filter", required = true)]
    /// Tag filter in form tag=val1|val2|... Multiple filters are ANDed.
    tag_filters: Vec<String>,
}

impl AppArgs {
    fn parse_tag_filters(&self) -> TagFilters {
        self.tag_filters
            .iter()
            .map(|v| {
                let mut parts = v.split("=");
                let tag_name = parts.next().unwrap();

                let values: HashSet<_> = parts
                    .flat_map(|v| v.split("|"))
                    .map(|v| v.to_owned())
                    .collect();

                (tag_name.to_owned(), values)
            })
            .collect()
    }
}

fn matches_tag_filters(tag_filters: &TagFilters, tags: &Tags) -> bool {
    tag_filters.iter().all(|(tag_name, tag_values)| {
        tag_values
            .iter()
            .any(|value| tags.contains(tag_name, value))
    })
}

fn node_to_point(node: &Node) -> Vec<f64> {
    vec![node.lon(), node.lat()]
}

fn node_to_feature(node: &Node, tag_filters: &TagFilters) -> Option<GeoJson> {
    if !matches_tag_filters(&tag_filters, &node.tags) {
        return None;
    }

    let point = node_to_point(&node);

    let feature = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(Geometry::new(Value::Point(point))),
        id: Some(feature::Id::String(format!("node/{}", node.id.0))),
        properties: tags_to_props(&node.tags),
        foreign_members: None,
    });

    Some(feature)
}

fn way_to_feature(way: Way, dep_nodes: &NodeToCoords) -> Option<GeoJson> {
    if way.is_open() {
        return None;
    }

    let ring: Vec<_> = way
        .nodes
        .into_iter()
        .map(|node_id| dep_nodes.get(&node_id).expect("Could not find node in way"))
        .map(|coords| coords.to_owned())
        .collect();

    let feature = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(Geometry::new(Value::Polygon(vec![ring]))),
        id: Some(feature::Id::String(format!("way/{}", way.id.0))),
        properties: tags_to_props(&way.tags),
        foreign_members: None,
    });

    Some(feature)
}

fn write_geojson<W: Write>(output: &mut W, geojson: &GeoJson) {
    writeln!(output, "{}", geojson.to_string()).expect("Could not write GeoJSON to output file");
}

fn tags_to_props(tags: &Tags) -> Option<Map<String, serde_json::Value>> {
    let tags_impl: &TagsImpl = &tags;
    let mut properties = Map::new();
    for (tag, value) in tags_impl {
        properties.insert(
            tag.to_owned(),
            to_value(value).expect("Could not convert tag value to JSON"),
        );
    }

    Some(properties)
}
