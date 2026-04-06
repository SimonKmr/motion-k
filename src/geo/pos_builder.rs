use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use vector2d::Vector2D;
use crate::geo::map_generator::{MapTransform, Node, RelationDrawOrder as OtherRelationDrawOrder, WayData};
use crate::motion_graphics::attributes::attribute;
use crate::motion_graphics::attributes::attribute::Attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use crate::motion_graphics::elements::element::DrawInfo;

pub trait PositionBuilder {
    fn build(self) -> Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>;
    
}

pub struct WayPositionBuilder<'a>{
    pub way_points: &'a Vec<Node>,
    pub transform: &'a MapTransform,
    pub draw_info: &'a DrawInfo
}
impl PositionBuilder for WayPositionBuilder<'_> {
    fn build(self) -> Vec<Box<dyn Attribute<Vector2D<f32>>>> {
        let outer_frame_scale = 2f64;
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for wp in self.way_points.iter() {
            let (y, x) = self.transform.apply(wp);

            if  MapTransform::is_on_screen(x,y,self.draw_info,outer_frame_scale) {
                points.push(Vector2D::new(x as f32, -y as f32).into_bsa());
            }
        }
        points
    }
}

pub struct AreaPositionBuilder<'a>{
    pub area: &'a Vec<WayData>,
    pub transform: &'a MapTransform,
    pub draw_info: &'a DrawInfo
}

impl PositionBuilder for AreaPositionBuilder<'_> {
    fn build(self) -> Vec<Box<dyn Attribute<Vector2D<f32>>>> {
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for ways in self.area.iter(){
            for wp in &ways.way_points {
                let (y, x) = self.transform.apply(wp);
                if  MapTransform::is_on_screen(x,y,self.draw_info,2f64) {
                    points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
                }
            }
        }
        points
    }
}

pub struct OrderedAreaPositionBuilder<'a>{
    pub area: &'a Vec<WayData>,
    pub transform: &'a MapTransform,
    pub draw_info: &'a DrawInfo,
    pub order: &'a Vec<RelationDrawOrder>
}

impl PositionBuilder for OrderedAreaPositionBuilder<'_> {
    fn build(self) -> Vec<Box<dyn Attribute<Vector2D<f32>>>> {
        let mut points = Vec::<Box<dyn attribute::Attribute<Vector2D<f32>>>>::new();
        for draw_order in self.order{
            let way_data = &self.area[draw_order.index];
            let mut way_points_list = way_data.way_points.clone();
            if draw_order.is_reversed{
                way_points_list.reverse();
            }

            for wp in way_points_list {
                let (y, x) = self.transform.apply(&wp);
                if  MapTransform::is_on_screen(x,y,self.draw_info,2f64) {
                    points.push(Vector2D::new(x as f32, -y as f32).into_bsa())
                }
            }
        }
        points
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RelationDrawOrder{
    pub index: usize,
    pub is_reversed: bool,
}

impl RelationDrawOrder {
    pub(crate) fn from_ways(ways: &Vec<WayData>) -> Result<Vec<Self>, String> {

        let node = match ways.first() {
            Some(node) => node,
            None => return Ok(Vec::new()),
        };

        let mut first_node = match node.way_points.first(){
            Some(node) => node,
            None => return Ok(Vec::new()),
        };
        let mut last_node = match node.way_points.last(){
            Some(node) => node,
            None => return Ok(Vec::new()),
        };

        let mut prev_inversed: bool = false;

        let mut already_inserted = Vec::new();
        let mut temp_res = VecDeque::<RelationDrawOrder>::new();
        temp_res.push_back(RelationDrawOrder{
            index: 0,
            is_reversed: false,
        });

        let mut res = Vec::new();
        for _ in ways.iter(){
            for (i,way) in ways[1..].iter().enumerate() {
                let i = i + 1;
                let current_node_start = way.way_points.first().unwrap();
                let current_node_end = way.way_points.last().unwrap();

                if already_inserted.contains(&way.id) { continue; }

                if last_node == current_node_start{
                    //append after
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.push_back(x);
                    last_node = current_node_end;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if first_node == current_node_end{
                    //append before
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.push_front(x);
                    first_node = current_node_start;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if last_node == current_node_end {
                    //append after : reversed
                    let is_reversed= true ^ prev_inversed;
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed,
                    };
                    prev_inversed = is_reversed;
                    temp_res.push_back(x);

                    last_node = current_node_start;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                if first_node == current_node_start {
                    //append before : reversed
                    let is_reversed= true ^ prev_inversed;
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed,
                    };
                    prev_inversed = is_reversed;
                    temp_res.push_front(x);

                    first_node = current_node_end;
                    already_inserted.push(way.id.clone());
                    continue;
                }

                let has_remaining_nodes = i < ways.len();
                if first_node == last_node && has_remaining_nodes
                {
                    for tr in &temp_res{
                        res.push(tr.clone());
                    }
                    let x = RelationDrawOrder{
                        index: i,
                        is_reversed: false,
                    };
                    temp_res.clear();
                    temp_res.push_back(x);

                    first_node = current_node_start;
                    last_node = current_node_end;
                }
            }
        }

        //add logic if relation consists of multiple areas instead of one big
        for tr in &temp_res{
            res.push(tr.clone());
        }

        Ok(res)
    }
}