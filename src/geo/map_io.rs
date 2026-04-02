use std::collections::{HashMap, LinkedList};
use std::path::Path;
use osmpbf::{Element, ElementReader};
use crate::geo::map_generator::{MapData, Node, RelationData, RelationDrawOrder, Tag, WayData};
use crate::geo::style::MapStyleSettings;

pub struct MapIO { }
impl MapIO {
    pub fn load(path: &String, settings: Option<MapStyleSettings>) -> MapData{

        let file_name_bin = format!("{path}.bin");
        let path_bin = Path::new(&file_name_bin);

        if Path::new(&path_bin).exists(){
            match MapIO::import_binary(&file_name_bin) {
                Ok(res) => res,
                Err(_) => {
                    let res = MapIO::import_osm(path,settings);
                    MapIO::export_binary(file_name_bin,&res);
                    res
                }
            }
        } else {
            let res = MapIO::import_osm(path,settings);
            MapIO::export_binary(file_name_bin,&res);
            res
        }
    }

    pub fn import_osm(path: &String, settings: Option<MapStyleSettings>) -> MapData{
        let default_settings = MapStyleSettings::default();
        let settings = settings.as_ref().unwrap_or(&default_settings);

        let mut nodes = HashMap::<i64,Node>::new();
        let mut _ways = HashMap::<i64,WayData>::new();
        let mut relations = Vec::<RelationData>::new();

        let reader = ElementReader::from_path(&path).unwrap();
        reader.for_each(|element| {
            if let Element::Node(node) = element {
                let id = node.id();
                nodes.insert(id, Node{
                    id,
                    x: node.lat(),
                    y: node.lon(),
                    tag: None});
            }
            else if let Element::DenseNode(node) = element {
                let id = node.id();
                nodes.insert(id, Node{
                    id,
                    x: node.lat(),
                    y: node.lon(),
                    tag: None});
            } else if let Element::Way(way) = element{
                let mut tag : Option<Tag> = None;

                for _tag in way.tags(){
                    if settings.filter_by_tag(_tag.0) && settings.filter_by_value(_tag.1){
                        tag = settings.map_tag_to_category(_tag.0,_tag.1);
                        break;
                    }
                }

                let mut way_nodes : Vec<Node> = Vec::new();
                for way_ref in way.refs(){
                    if nodes.contains_key(&way_ref){
                        way_nodes.push(nodes[&way_ref].clone());
                    }
                }

                let id = way.id();
                _ways.insert(id, WayData{ id, tag, way_points: way_nodes });
            } else if let Element::Relation(relation) = element {
                let id = relation.id();
                if id == 3505080{
                    println!("id: {}",id);
                }

                if !relations.iter().any(|x|{
                    x.id == id
                }){
                    let mut tag : Option<Tag> = None;

                    for _tag in relation.tags(){
                        if settings.filter_by_area_tag(_tag.0) && settings.filter_by_value(_tag.1){
                            tag = settings.map_tag_to_category(_tag.0,_tag.1);
                            break;
                        }
                    }

                    let mut inner = Vec::<WayData>::new();
                    let mut outer = Vec::<WayData>::new();
                    let mut empty = Vec::<WayData>::new();

                    for member in relation.members(){
                        let index = member.member_id.clone();
                        if _ways.contains_key(&index){
                            let way = _ways[&index].clone();
                            match member.role() {
                                Ok("outer") =>
                                    { outer.push(way); }
                                Ok("inner") => {inner.push(way);}
                                Ok("") => {empty.push(way)}
                                _ => {}
                            }
                        }
                    }

                    let draw_orders = RelationDrawOrder::from_ways(&outer).unwrap();

                    let relation = RelationData{
                        id,
                        tag,
                        draw_orders,
                        outer,
                        inner,
                        empty,
                    };
                    relations.push(relation);
                }
            }
        }).unwrap();

        let ways : Vec::<WayData> = _ways.into_iter().map(|way| {way.1}).collect();

        MapData{
            relations,
            ways,
        }
    }

    fn import_binary(path: &String) -> Result<MapData, &'static str>{
        let bytes = std::fs::read(path).unwrap();
        let result : MapData = bincode::deserialize(bytes.as_slice()).unwrap();
        Ok(result)
    }

    fn export_binary(path: String, map_data: &MapData) {
        let bytes = bincode::serialize(map_data);
        std::fs::write(path, bytes.unwrap()).unwrap();
    }
}